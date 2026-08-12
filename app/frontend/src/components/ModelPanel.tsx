import { useEffect, useState } from 'react';

type Model = { id: string; label: string; kind: string; availability: string };
type Catalog = { engine: string; models: Model[]; source: string; error?: string };

export function ModelPanel({ onSelect }: { onSelect?: (model: Model) => void }) {
  const [catalog, setCatalog] = useState<Catalog | null>(null);
  const [error, setError] = useState('');

  useEffect(() => {
    let active = true;
    import('@tauri-apps/api/core').then(({ invoke }) => invoke<Catalog>('model_catalog'))
      .then(value => active && setCatalog(value))
      .catch(value => active && setError(value instanceof Error ? value.message : String(value)));
    return () => { active = false; };
  }, []);

  return <section className="model-panel">
    <div className="panel-title">Models <span>{catalog?.models.length ?? 0}</span></div>
    {error && <div className="error">{error}</div>}
    {!catalog && !error && <div className="empty-assets">Reading WanGP model catalog…</div>}
    {catalog?.models.map(model => <button className="model-row" key={model.id} onClick={() => onSelect?.(model)} disabled={model.availability === 'unavailable'}>
      <span className={`capability-dot ${model.availability}`} />
      <div><strong>{model.label}</strong><small>{model.kind} · {model.id}</small></div>
    </button>)}
    {catalog?.error && <small className="model-note">{catalog.error}</small>}
  </section>;
}
