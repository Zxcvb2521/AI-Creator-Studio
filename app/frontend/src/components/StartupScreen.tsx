import { useEffect, useState } from 'react';

interface StartupStep { id: string; status: string; detail: string }
interface StartupReport { ready: boolean; steps: StartupStep[] }

async function invoke<T>(command: string): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<T>(command);
}

const labels: Record<string, string> = {
  system: 'System check',
  engine: 'WanGP engine',
  bridge: 'WanGP Bridge',
};

export function StartupScreen({ onReady }: { onReady: () => void }) {
  const [report, setReport] = useState<StartupReport>({ ready: false, steps: [] });
  const [error, setError] = useState('');

  useEffect(() => {
    let active = true;
    invoke<StartupReport>('startup')
      .then(value => { if (active) { setReport(value); if (value.ready) onReady(); } })
      .catch(value => active && setError(value instanceof Error ? value.message : String(value)));
    return () => { active = false; };
  }, [onReady]);

  return <main className="startup-screen">
    <div className="startup-card">
      <div className="eyebrow">AI CREATOR STUDIO</div>
      <h1>Preparing your workspace</h1>
      <p className="startup-subtitle">Проверяем систему и подключаем WanGP.</p>
      <div className="startup-steps">
        {report.steps.map(step => <div className="startup-step" key={step.id}><span className={`check ${step.status}`}>{step.status === 'ready' ? '✓' : step.status === 'failed' || step.status === 'blocked' ? '!' : '…'}</span><div><strong>{labels[step.id] ?? step.id}</strong><small>{step.detail}</small></div></div>)}
      </div>
      {!report.steps.length && <div className="startup-loading">Starting checks…</div>}
      {error && <div className="error">{error}</div>}
      {!report.ready && report.steps.some(step => ['failed', 'blocked', 'timeout'].includes(step.status)) && <button className="primary startup-retry" onClick={() => window.location.reload()}>Retry</button>}
    </div>
  </main>;
}
