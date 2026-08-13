import { useState } from 'react';
import { DemoModelsPanel } from './DemoModelsPanel';

const samples = [
  { id: 'forest', title: 'Magical Forest', kind: 'Video', meta: 'Wan 2.2 · 720p · 5s' },
  { id: 'lumi', title: 'Lumi Character', kind: 'Image', meta: 'Qwen Image · 1024px' },
  { id: 'voice', title: 'Lumi Voice', kind: 'Audio', meta: 'Qwen3 TTS · RU' },
];
type Model = { id: string; label: string; kind: string; available: boolean };

export function DemoStudio() {
  const [prompt, setPrompt] = useState('Макс открывает загадочную светящуюся коробочку в лесу. Из неё появляется разноцветный магический туман.');
  const [active, setActive] = useState('forest');
  const [mode, setMode] = useState<'Generate' | 'Deepy'>('Generate');
  const [showModels, setShowModels] = useState(false);
  const [selectedModel, setSelectedModel] = useState<Model>({ id: 'wan-video', label: 'Wan Video', kind: 'Video', available: true });

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
        <section className="prompt-card"><div className="prompt-label"><span>{mode === 'Deepy' ? 'DEEPY · CREATIVE ASSISTANT' : 'PROMPT'}</span><span className="counter">{prompt.length}/2000</span></div><textarea value={prompt} onChange={e => setPrompt(e.target.value)} /><div className="prompt-footer"><button className="mini">＋ Reference</button><button className="mini" onClick={() => setShowModels(true)}>◇ {selectedModel.label}</button><button className="mini">⚙ Advanced</button><button className="generate-btn">{mode === 'Deepy' ? '✦ Ask Deepy' : 'Generate'} <span>→</span></button></div></section>
        <div className="section-head"><span>RECENT CREATIONS</span><button>View all →</button></div>
        <section className="creation-grid">{samples.map(s => <button key={s.id} onClick={() => setActive(s.id)} className={`creation ${active === s.id ? 'active' : ''}`}><div className={`thumb ${s.id}`}><span>{s.id === 'forest' ? '✦' : s.id === 'lumi' ? '◉' : '♫'}</span><small>{s.kind}</small></div><div className="creation-info"><b>{s.title}</b><small>{s.meta}</small></div></button>)}</section>
        <section className="timeline-demo"><div className="section-head"><span>TIMELINE</span><span className="muted">00:00 — 00:15</span></div><div className="track"><div className="playhead" /><div className="clip clip-a">Scene 01</div><div className="clip clip-b">Scene 02</div><div className="clip clip-c">Voice</div></div></section>
      </main>
      {showModels && <DemoModelsPanel selected={selectedModel.id} onSelect={model => { setSelectedModel(model); setShowModels(false); }} />}
    </div>
  </div>;
}
