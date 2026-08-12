export type AssetKind = 'image' | 'video' | 'audio' | 'voice';

export interface ProjectAsset {
  id: string;
  kind: AssetKind;
  name: string;
  path: string;
  duration?: number;
  width?: number;
  height?: number;
  createdAt: string;
  metadata?: Record<string, unknown>;
}

export interface TimelineItem {
  id: string;
  assetId: string;
  trackId: string;
  start: number;
  duration: number;
  offset?: number;
}

export interface TimelineTrack {
  id: string;
  name: string;
  kind: AssetKind;
  items: TimelineItem[];
}

export interface ProjectDocument {
  schemaVersion: 1;
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
  assets: ProjectAsset[];
  tracks: TimelineTrack[];
}

export function createProject(name = 'Untitled Project'): ProjectDocument {
  const now = new Date().toISOString();
  return {
    schemaVersion: 1,
    id: crypto.randomUUID(),
    name,
    createdAt: now,
    updatedAt: now,
    assets: [],
    tracks: [
      { id: crypto.randomUUID(), name: 'Video', kind: 'video', items: [] },
      { id: crypto.randomUUID(), name: 'Voice', kind: 'voice', items: [] },
      { id: crypto.randomUUID(), name: 'Music', kind: 'audio', items: [] },
    ],
  };
}
