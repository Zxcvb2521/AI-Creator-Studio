import React, { useEffect, useMemo, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './styles.css';
import './components/timeline.css';
import { Timeline } from './components/Timeline';
import { PreviewPlayer } from './components/PreviewPlayer';
import { createProject, type ProjectAsset, type ProjectDocument } from '../../../../projects/src/model';
import { addAssetToTimeline } from '../../../../projects/src/timeline';
import { saveProject } from '../../../../projects/src/store';
import { WanGPBridgeClient } from '../../../../engine/wan-gp-adapter/src/bridge-client';

type EngineStatus = { running: boolean; runtime_dir: string; engine_dir: string };
const tauriInvoke = async <T,>(command: string): Promise<T> => {
  const { invoke } = await import('@tauri-apps/api/core');
  return await invoke<T>(command);
};

function App() {
  const [status, setStatus] = useState<EngineStatus | null>(null);
  const [error, setError] = useState('');
  const [project, setProject] = useState<ProjectDocument>(() => createProject('New Creator Project'));
  const [selectedAssetId, setSelectedAssetId] = useState<string>();
  const [prompt, setPrompt] = useState('A cinematic magical forest at sunset');
  const [generating, setGenerating] = useState(false);
  const client = useMemo(() => new WanGPBridgeClient(), []);

  const refresh = async () => {
    try { setStatus(await tauriInvoke<EngineStatus>('engine_status')); }
    catch (e) { setError(e instanceof Error ? e.message : String(e)); }
  };
  useEffect(() => { void refresh(); }, []);

  const updateProject = (next: ProjectDocument) => { setProject(next); saveProject(next); };

  const generateVideo = async () => {
    if (!prompt.trim() || generating) return;
    try {
      setError(''); setGenerating(true);
      const job = await client.generateVideo(prompt.trim(), project.id);
      const result = await client.waitForJob(job.job_id, snapshot => {
        if (snapshot.error) setError(snapshot.error);
      });
      if (result.state !== 'completed' || !result.result?.files?.length) {
        throw new Error(result.error ?? 'WanGP не вернул готовый видеофайл');
      }
      const asset: ProjectAsset = {
        id: crypto.randomUUID(), kind: 'video', name: result.result.files[0].split(/[\\/]/).pop() ?? 'Generated Video',
        path: result.result.files[0], duration: result.result.duration, createdAt: new Date().toISOString(), metadata: result.result.metadata,
      };
      updateProject(addAssetToTimeline({ ...project, assets: [...project.assets, asset] }, asset));
      setSelectedAssetId(asset.id);
    } catch (e) { setError(e instanceof Error ? e.message : String(e)); }
    finally { setGenerating(false); }
  };

  const selectedAsset = project.assets.find(a => a.id === selectedAssetId);

  return <main className="shell">
    <header className="topbar"><div><div className="eyebrow">AI CREATOR STUDIO</div><h1>{project.name}</h1></div><div className={`status ${status?.running ? 'online' : ''}`}><span className="dot" />{status?.running ? 'WanGP запущен' : 'WanGP не подключён'}</div></header>
    <section className="workspace">
      <aside className="sidebar">{['Project','Generate','Timeline','Deepy','Models','Settings'].map(x => <button className={`nav ${x === 'Project' ? 'active' : ''}`} key={x}>{x}</button>)}</aside>
      <section className="content">
        <div className="generator"><div><span className="badge">WAN2GP GENERATOR</span><h2>Создать видео</h2><textarea value={prompt} onChange={e => setPrompt(e.target.value)} placeholder="Опишите сцену..." /></div><button className="primary" onClick={generateVideo} disabled={generating}>{generating ? 'Генерация...' : 'Generate Video'}</button></div>
        {error && <div className="error">{error}</div>}
        <div className="studio-grid"><PreviewPlayer asset={selectedAsset} /><div className="assets"><div className="panel-title">Project Assets <span>{project.assets.length}</span></div>{project.assets.length === 0 && <div className="empty-assets">Здесь появятся результаты генерации</div>}{project.assets.map(asset => <button className={`asset ${asset.id === selectedAssetId ? 'selected' : ''}`} key={asset.id} onClick={() => setSelectedAssetId(asset.id)}><strong>{asset.name}</strong><small>{asset.kind}{asset.duration ? ` · ${asset.duration.toFixed(1)}s` : ''}</small></button>)}</div></div>
        <Timeline project={project} onChange={updateProject} />
      </section>
    </section>
  </main>;
}

createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
