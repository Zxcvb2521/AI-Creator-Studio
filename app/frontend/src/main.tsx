import React, { useCallback, useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './styles.css';
import './components/timeline.css';
import './components/startup.css';
import './components/capabilities.css';
import './components/demo-studio.css';
import { StartupScreen } from './components/StartupScreen';
import { DemoStudio } from './components/DemoStudio';

function App() {
  const [ready, setReady] = useState(false);
  const [demo, setDemo] = useState(true);
  const onReady = useCallback(() => setReady(true), []);
  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    if (params.get('demo') === 'false') setDemo(false);
  }, []);
  if (!ready) return <StartupScreen onReady={onReady} />;
  if (demo) return <DemoStudio />;
  return <div className="legacy-workspace">Workspace integration mode</div>;
}

createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
