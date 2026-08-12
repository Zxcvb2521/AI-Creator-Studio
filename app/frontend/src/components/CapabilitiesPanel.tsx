import { useEffect, useState } from 'react';

type Capability = { id: string; label: string; status: string; detail: string };
type Capabilities = { engine: string; root: string; capabilities: Capability[] };

export function CapabilitiesPanel() {
  const [data, setData] = useState<Capabilities | null>(null);
  const [error, setError] = useState('');

  useEffect(() => {
    let active = true;
    import('@tauri-apps/api/core').then(({ invoke }) => invoke<Capabilities>('capabilities'))
      .then(value => active && setData(value))
      .catch(value => active && setError(value instanceof Error ? value.message : String(value)));
    return () => { active = false; };
  }, []);

  return <section className="capabilities-panel">
    <div className="panel-title">Engine Capabilities <span>{data?.engine ?? 'Wan2GP'}</span></div>
    {error && <div className="error">{error}</div>}
    {!data && !error && <div className="empty-assets">Checking installed engine…</div>}
    {data?.capabilities.map(item => <div className="capability" key={item.id}>
      <span className={`capability-dot ${item.status}`} />
      <div><strong>{item.label}</strong><small>{item.detail}</small></div>
      <em>{item.status}</em>
    </div>)}
  </section>;
}
