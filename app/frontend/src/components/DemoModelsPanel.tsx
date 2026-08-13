import { useEffect, useState } from 'react';

type Model = { id: string; label: string; kind: string; available: boolean };
type Catalog = { models: Model[]; source: string; error?: string };
const demoModels: Model[] = [
  { id: 'wan-video', label: 'Wan Video', kind: 'Video', available: true },
  { id: 'qwen-image', label: 'Qwen Image', kind: 'Image', available: true },
  { id: 'qwen3-tts', label: 'Qwen3 TTS', kind: 'Audio', available: true },
];
export function DemoModelsPanel({ selected, onSelect }: { selected: string; onSelect: (model: Model) => void }) {
  const [catalog, setCatalog] = useState<Catalog>({ models: demoModels, source: 'demo' });
  useEffect(() => { import('@tauri-apps/api/core').then(({ invoke }) => invoke<Catalog>('model_catalog')).then(value => { if (value.models?.length) setCatalog(value); }).catch(() => undefined); }, []);
  return <section className="models-drawer">
    <div className="drawer-head"><div><small>WAN2GP</small><h2>Models</h2></div><span>{catalog.models.length}</span></div>
    {catalog.error && <div className="drawer-note">Demo catalog — {catalog.error}</div>}
    <div className="model-list">{catalog.models.map(model => <button key={model.id} className={`model-card ${selected === model.id ? 'selected' : ''}`} onClick={() => onSelect(model)} disabled={!model.available}>
      <div className={`model-icon ${model.kind.toLowerCase()}`}>{model.kind === 'Video' ? '▶' : model.kind === 'Image' ? '▧' : '♫'}</div>
      <div><b>{model.label}</b><small>{model.kind}</small></div><i className={model.available ? 'ready' : ''} />
    </button>)}</div>
  </section>;
}
