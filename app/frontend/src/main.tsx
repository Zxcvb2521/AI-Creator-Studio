import React, { useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './styles.css';

type EngineStatus = {
  running: boolean;
  runtime_dir: string;
  engine_dir: string;
};

const tauriInvoke = async <T,>(command: string): Promise<T> => {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(command);
  } catch {
    throw new Error('Tauri runtime недоступен. Запустите приложение через desktop shell.');
  }
};

function App() {
  const [status, setStatus] = useState<EngineStatus | null>(null);
  const [error, setError] = useState('');

  const refresh = async () => {
    try {
      setError('');
      setStatus(await tauriInvoke<EngineStatus>('engine_status'));
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  };

  useEffect(() => { void refresh(); }, []);

  const start = async () => {
    try {
      setError('');
      await tauriInvoke<string>('start_engine');
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  };

  const stop = async () => {
    try {
      setError('');
      await tauriInvoke<string>('stop_engine');
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  };

  return (
    <main className="shell">
      <header className="topbar">
        <div>
          <div className="eyebrow">AI CREATOR STUDIO</div>
          <h1>Creator Workspace</h1>
        </div>
        <div className={`status ${status?.running ? 'online' : ''}`}>
          <span className="dot" /> {status?.running ? 'WanGP запущен' : 'WanGP остановлен'}
        </div>
      </header>

      <section className="workspace">
        <aside className="sidebar">
          <button className="nav active">Project</button>
          <button className="nav">Generate</button>
          <button className="nav">Timeline</button>
          <button className="nav">Deepy</button>
          <button className="nav">Models</button>
          <button className="nav">Settings</button>
        </aside>

        <section className="content">
          <div className="hero">
            <div>
              <span className="badge">STUDIO ENGINE</span>
              <h2>Один проект — весь медиапайплайн</h2>
              <p>Изображения, видео, голос, музыка и Timeline работают поверх WanGP. Deepy остаётся родным инструментом WanGP.</p>
            </div>
            <div className="engine-card">
              <strong>{status?.running ? 'READY' : 'OFFLINE'}</strong>
              <div className="actions">
                <button onClick={start} disabled={status?.running}>Запустить WanGP</button>
                <button onClick={stop} disabled={!status?.running}>Остановить</button>
              </div>
            </div>
          </div>

          <div className="cards">
            {['Image', 'Video', 'Voice', 'Music', 'Deepy', 'Timeline'].map((item) => (
              <article className="card" key={item}>
                <span className="card-icon">{item === 'Deepy' ? '✦' : '◆'}</span>
                <h3>{item}</h3>
                <p>{item === 'Deepy' ? 'Нативный Deepy из WanGP' : 'Подключается через Studio engine adapter'}</p>
              </article>
            ))}
          </div>

          {error && <div className="error">{error}</div>}
          {status && <div className="diagnostics"><span>Runtime</span><code>{status.runtime_dir}</code><span>Engine</span><code>{status.engine_dir}</code></div>}
        </section>
      </section>
    </main>
  );
}

createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
