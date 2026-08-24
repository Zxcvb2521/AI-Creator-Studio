import { useEffect, useState } from 'react';

interface StartupStep { id: string; status: string; detail: string }
interface StartupReport { ready: boolean; steps: StartupStep[] }
async function invoke<T>(command: string): Promise<T> { const { invoke } = await import('@tauri-apps/api/core'); return invoke<T>(command); }
const labels: Record<string, string> = { system: 'System check', runtime: 'Studio runtime', engine: 'Wan2GP engine', bridge: 'Wan2GP Bridge', python: 'Python', uv: 'uv bootstrap', gpu: 'GPU', ffmpeg: 'FFmpeg' };
export function StartupScreen({ onReady }: { onReady: () => void }) {
  const [report, setReport] = useState<StartupReport>({ ready: false, steps: [] });
  const [error, setError] = useState('');
  useEffect(() => { let active = true; invoke<StartupReport>('startup').then(value => { if (active) { setReport(value); if (value.ready) onReady(); } }).catch(value => active && setError(value instanceof Error ? value.message : String(value))); return () => { active = false; }; }, [onReady]);
  const failed = report.steps.filter(step => ['missing', 'failed', 'blocked'].includes(step.status));
  const installing = report.steps.some(step => step.status === 'installing');
  return <main className="startup-screen"><div className="startup-card"><div className="eyebrow">AI CREATOR STUDIO</div><h1>Preparing your workspace</h1><p className="startup-subtitle">Проверяем систему и подключаем Wan2GP.</p><div className="startup-steps">{report.steps.map(step => <div className="startup-step" key={step.id}><span className={`check ${step.status}`}>{step.status === 'ready' ? '✓' : ['missing', 'failed', 'blocked'].includes(step.status) ? '!' : '…'}</span><div><strong>{labels[step.id] ?? step.id}</strong><small>{step.detail}</small></div></div>)}</div>{!report.steps.length && <div className="startup-loading">Starting checks…</div>}{installing && <div className="startup-loading">Первый запуск: Studio автоматически скачивает и настраивает runtime. Это может занять несколько минут.</div>}{error && <div className="error">{error}</div>}{failed.length > 0 && <div className="startup-note"><strong>Не удалось подготовить runtime.</strong><small>Studio не требует отдельной установки Git, Conda или системного Python. Проверьте интернет-соединение и повторите запуск.</small></div>}{!report.ready && failed.length === 0 && !installing && <div className="startup-note"><strong>Wan2GP ещё не готов.</strong><small>Studio устанавливает все необходимые компоненты внутри своей папки. Пользователю остаётся только скачать поддерживаемые модели.</small></div>}</div></main>;
}
