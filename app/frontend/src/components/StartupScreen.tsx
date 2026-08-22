import { useEffect, useState } from 'react';

interface StartupStep { id: string; status: string; detail: string }
interface StartupReport { ready: boolean; steps: StartupStep[] }
async function invoke<T>(command: string): Promise<T> { const { invoke } = await import('@tauri-apps/api/core'); return invoke<T>(command); }
const labels: Record<string, string> = { system: 'System check', engine: 'Wan2GP engine', bridge: 'Wan2GP Bridge', python: 'Python', git: 'Git', conda: 'Conda', ffmpeg: 'FFmpeg' };
export function StartupScreen({ onReady }: { onReady: () => void }) {
  const [report, setReport] = useState<StartupReport>({ ready: false, steps: [] });
  const [error, setError] = useState('');
  useEffect(() => { let active = true; invoke<StartupReport>('startup').then(value => { if (active) { setReport(value); if (value.ready) onReady(); } }).catch(value => active && setError(value instanceof Error ? value.message : String(value))); return () => { active = false; }; }, [onReady]);
  const missing = report.steps.filter(step => ['missing', 'failed', 'blocked'].includes(step.status));
  return <main className="startup-screen"><div className="startup-card"><div className="eyebrow">AI CREATOR STUDIO</div><h1>Preparing your workspace</h1><p className="startup-subtitle">Проверяем систему и подключаем Wan2GP.</p><div className="startup-steps">{report.steps.map(step => <div className="startup-step" key={step.id}><span className={`check ${step.status}`}>{step.status === 'ready' ? '✓' : step.status === 'missing' || step.status === 'failed' || step.status === 'blocked' ? '!' : '…'}</span><div><strong>{labels[step.id] ?? step.id}</strong><small>{step.detail}</small></div></div>)}</div>{!report.steps.length && <div className="startup-loading">Starting checks…</div>}{error && <div className="error">{error}</div>}{missing.length > 0 && <div className="startup-note"><strong>Wan2GP пока не установлен.</strong><small>При обычной установке Studio все необходимые runtime-компоненты будут размещаться внутри папки Studio. Пользователю не потребуется отдельно устанавливать или настраивать Wan2GP.</small></div>}{!report.ready && missing.length === 0 && <div className="startup-note"><strong>Wan2GP engine ещё не подключён.</strong><small>После установки runtime Studio автоматически сможет использовать встроенный engine.</small></div>}</div></main>;
}
