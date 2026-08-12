import type { ProjectAsset, ProjectDocument, TimelineItem } from './model';

const STORAGE_KEY = 'ai-creator-studio.projects.v1';

type ProjectStore = Record<string, ProjectDocument>;

function readStore(): ProjectStore {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? JSON.parse(raw) as ProjectStore : {};
  } catch {
    return {};
  }
}

function writeStore(store: ProjectStore): void {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(store));
}

export function saveProject(project: ProjectDocument): void {
  project.updatedAt = new Date().toISOString();
  const store = readStore();
  store[project.id] = project;
  writeStore(store);
}

export function loadProject(id: string): ProjectDocument | null {
  return readStore()[id] ?? null;
}

export function listProjects(): ProjectDocument[] {
  return Object.values(readStore()).sort((a, b) => b.updatedAt.localeCompare(a.updatedAt));
}

export function addAsset(project: ProjectDocument, asset: ProjectAsset): ProjectDocument {
  return { ...project, assets: [...project.assets, asset], updatedAt: new Date().toISOString() };
}

export function addTimelineItem(project: ProjectDocument, item: TimelineItem): ProjectDocument {
  return {
    ...project,
    tracks: project.tracks.map(track => track.id === item.trackId
      ? { ...track, items: [...track.items, item] }
      : track),
    updatedAt: new Date().toISOString(),
  };
}
