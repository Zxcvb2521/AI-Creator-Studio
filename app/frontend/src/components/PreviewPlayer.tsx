import { useEffect, useRef, useState } from 'react';
import type { ProjectAsset } from '../../../../projects/src/model';

interface Props {
  asset?: ProjectAsset;
}

export function PreviewPlayer({ asset }: Props) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const audioRef = useRef<HTMLAudioElement>(null);
  const [error, setError] = useState('');

  useEffect(() => setError(''), [asset?.id]);

  if (!asset) {
    return <div className="preview empty">Выберите видео или аудио для предпросмотра</div>;
  }

  const source = asset.path;
  if (asset.kind === 'video') {
    return <div className="preview"><video ref={videoRef} src={source} controls onError={() => setError('Не удалось открыть видео')} /><span className="preview-name">{asset.name}</span>{error && <span className="preview-error">{error}</span>}</div>;
  }
  if (asset.kind === 'audio' || asset.kind === 'voice') {
    return <div className="preview audio"><div className="audio-icon">♫</div><audio ref={audioRef} src={source} controls onError={() => setError('Не удалось открыть аудио')} /><span className="preview-name">{asset.name}</span>{error && <span className="preview-error">{error}</span>}</div>;
  }

  return <div className="preview"><img src={source} alt={asset.name} onError={() => setError('Не удалось открыть изображение')} /><span className="preview-name">{asset.name}</span>{error && <span className="preview-error">{error}</span>}</div>;
}
