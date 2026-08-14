import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { DemoModelsPanel } from './DemoModelsPanel';
import { SchemaForm } from './SchemaForm';

const samples = [
  { id: 'forest', title: 'Magical Forest', kind: 'Video', meta: 'Wan 2.2 · 720p · 5s' },
  { id: 'lumi', title: 'Lumi Character', kind: 'Image', meta: 'Qwen Image · 1024px' },
  { id: 'voice', title: 'Lumi Voice', kind: 'Audio', meta: 'Qwen3 TTS · RU' },
];
type Model = { id: string; label: string; kind: string; available: boolean };
type Catalog = { models: Model[]; source: string; error?: string };
type GenerationResult = { success: boolean; generated_files: string[]; errors: { message: string; stage?: string }[]; artifacts: { path?: string; media_type: string; fps?: number }[] };

export function DemoStudio() {
  const [prompt, setPrompt] = useState('Макс открывает загадочную светящуюся коробочку в лесу. Из неё появляется разноцветный магический туман.');
  const [active, setActive] = useState('forest');
  const [mode, setMode] = useState<'Generate' | 'Deepy'>('Generate');
  const [showModels, setShowModels] = useState(false);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [message, setMessage] = useState('');
  const [selectedModel, setSelectedModel] = useState<Model>({ id: 'wan-video', label: 'Wan Video', kind: 'Video', available: true });
  const [schema, setSchema] = useState<Record<string, unknown> | null>(null);
  const [settings, setSettings] = useState<Record<string, unknown>>({});

  useEffect(() => {
    invoke<Catalog>('model_catalog').then(catalog => {
      const firstVideo = catalog.models?.find(model => model.kind.toLowerCase().includes('video') && model.available) || catalog.models?.find(model => model.available);
      if (firstVideo) setSelectedModel(firstVideo);
    }).catch(() => undefined);
  }, []);

  useEffect(() => {
    if (!selectedModel.id) return;
    invoke<Record<string, unknown>>('model_schema', { modelType: selectedModel.id })
      .then(nextSchema => {
        setSchema(nextSchema);
        const properties = (nextSchema?.properties as Record<string, any> | undefined) || {};
        const defaults = Object.fromEntries(Object.entries(properties).filter(([, field]) => field?.default !== undefined).map(([name, field]) => [name, field.default]));
        setSettings(current => ({ ...defaults, ...current }));
      })
      .catch(() => { setSchema(null); });
  }, [selectedModel.id]);

  async function generate() {
    if (generating || !prompt.trim()) return;
    setGenerating(true);
    setMessage('Starting WanGP generation…');
    try {
      const result = await invoke<GenerationResult>('generate', { modelType: selectedModel.id, settings: { ...settings, prompt: prompt.trim() } });
      if (!result.success) setMessage(result.errors?.[0]?.message || 'Generation failed');
      else {
        const file = result.generated_files?.[0] || result.artifacts?.[0]?.path || 'generated asset';
        setMessage(`Generated: ${file}`);
        setActive('forest');
      }
    } catch (error) { setMessage(String(error)); }
    finally { setGenerating(false); }
  }

  return <div className="demo-studio">
    <header className="demo-topbar"><div className="brand"><span className="brand-mark">✦</span><div><b>AI CREATOR STUDIO</b><small>Creative AI Workspace</small></div></div><div className="engine-pill"><i /> WanGP <span>Connected</span></div></header>
    <div className="demo-body">
      <aside className="demo-sidebar">
        <div className="workspace-label">WORKSPACE</div>
        {['Project','Generate','Timeline','Assets'].map((x, i) => <button key={x} className={`demo-nav ${i === 1 ? 'selected' : ''}`}><span>{['⌂','✧','▤','▧'][i]}</span>{x}</button>)}
        <div className="workspace-label second">TOOLS</div>
        <button className="demo-nav" onClick={() => setMode('Deepy')}><span>◈</span>Deepy<em>AI</em></button>
        <button className={`demo-nav ${showModels ? 'selected' : ''}`} onClick={() => setShowModels(v => !v)}><span>◇</span>Models</button>
        <button className="demo-nav"><span>⚙</span>Settings</button>
        <div className="sidebar-bottom"><div className="gpu-dot" />RTX 5060 Ti<small>16 GB · Ready</small></div>
      </aside>
      <main className="demo-main">
        <div className="demo-heading"><div><small>CREATE</small><h1>Turn an idea into a scene</h1></div><button className="ghost">⌘ K &nbsp; Command</button></div>
        <div className="mode-tabs">{(['Generate','Deepy'] as const).map(x => <button onClick={() => setMode(x)} className={mode === x ? 'on' : ''} key={x}>{x === 'Deepy' && <span>✦</span>}{x}</button>)}</div>
        <section className="prompt-card"><div className="prompt-label"><span>{mode === 'Deepy' ? 'DEEPY · CREATIVE ASSISTANT' : 'PROMPT'}</span><span className="counter">{prompt.length}/2000</span></div><textarea value={prompt} onChange={e => setPrompt(e.target.value)} /><div className="prompt-footer"><button className="mini">＋ Reference</button><button className="mini" onClick={() => setShowModels(true)}>◇ {selectedModel.label}</button><button className="mini" onClick={() => setShowAdvanced(v => !v)}>⚙ Advanced {showAdvanced ? '⌃' : '⌄'}</button><button className="generate-btn" onClick={generate} disabled={generating}>{generating ? 'Generating…' : mode === 'Deepy' ? '✦ Ask Deepy' : 'Generate'} <span>→</span></button></div>{showAdvanced && schema && <div className="schema-panel"><SchemaForm schema={schema} values={settings} onChange={setSettings} /></div>}{message && <div className="generation-message">{message}</div>}</section>
        <div className="section-head"><span>RECENT CREATIONS</span><button>View all →</button></div>
        <section className="creation-grid">{samples.map(s => <button key={s.id} onClick={() => setActive(s.id)} className={`creation ${active === s.id ? 'active' : ''}`}><div className={`thumb ${s.id}`}><span>{s.id === 'forest' ? '✦' : s.id === 'lumi' ? '◉' : '♫'}</span><small>{s.kind}</small></div><div className="creation-info"><b>{s.title}</b><small>{s.meta}</small></div></button>)}</section>
        <section className="timeline-demo"><div className="section-head"><span>TIMELINE</span><span className="muted">00:00 — 00:15</span></div><div className="track"><div className="playhead" /><div className="clip clip-a">Scene 01</div><div className="clip clip-b">Scene 02</div><div className="clip clip-c">Voice</div></div></section>
      </main>
      {showModels && <DemoModelsPanel selected={selectedModel.id} onSelect={model => { setSelectedModel(model); setShowModels(false); }} />}
    </div>
  </div>;
}
