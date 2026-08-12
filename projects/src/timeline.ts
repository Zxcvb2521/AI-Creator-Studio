import type { AssetKind, ProjectAsset, ProjectDocument, TimelineItem } from './model';

function compatibleTrack(kind: AssetKind) {
  if (kind === 'video' || kind === 'image') return 'video';
  if (kind === 'voice') return 'voice';
  return 'audio';
}

export function addAssetToTimeline(project: ProjectDocument, asset: ProjectAsset): ProjectDocument {
  const trackKind = compatibleTrack(asset.kind) as AssetKind;
  const track = project.tracks.find(t => t.kind === trackKind);
  if (!track) return project;

  const end = track.items.reduce((max, item) => Math.max(max, item.start + item.duration), 0);
  const duration = asset.duration ?? 5;
  const item: TimelineItem = {
    id: crypto.randomUUID(),
    assetId: asset.id,
    trackId: track.id,
    start: end,
    duration,
    offset: 0,
  };

  return {
    ...project,
    tracks: project.tracks.map(t => t.id === track.id ? { ...t, items: [...t.items, item] } : t),
    updatedAt: new Date().toISOString(),
  };
}

export function removeTimelineItem(project: ProjectDocument, itemId: string): ProjectDocument {
  return {
    ...project,
    tracks: project.tracks.map(track => ({ ...track, items: track.items.filter(item => item.id !== itemId) })),
    updatedAt: new Date().toISOString(),
  };
}

export function moveTimelineItem(project: ProjectDocument, itemId: string, start: number): ProjectDocument {
  const safeStart = Math.max(0, start);
  return {
    ...project,
    tracks: project.tracks.map(track => ({
      ...track,
      items: track.items.map(item => item.id === itemId ? { ...item, start: safeStart } : item),
    })),
    updatedAt: new Date().toISOString(),
  };
}
