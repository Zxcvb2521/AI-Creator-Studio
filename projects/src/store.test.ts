import { describe, it, expect, beforeEach } from 'vitest';
import { addAsset, addTimelineItem, loadProject, listProjects, saveProject } from './store';
import type { ProjectDocument, ProjectAsset, TimelineItem } from './model';
import { createProject } from './model';

// Mock localStorage for tests
const mockStorage: Record<string, string> = {};
global.localStorage = {
  getItem: (key: string) => mockStorage[key] ?? null,
  setItem: (key: string, value: string) => { mockStorage[key] = value; },
  removeItem: (key: string) => { delete mockStorage[key]; },
  clear: () => { Object.keys(mockStorage).forEach(k => delete mockStorage[k]); },
  get length() { return Object.keys(mockStorage).length; },
  key: (index: number) => Object.keys(mockStorage)[index] ?? null,
} as Storage;

describe('store', () => {
  beforeEach(() => {
    // Clear storage before each test
    Object.keys(mockStorage).forEach(k => delete mockStorage[k]);
  });

  describe('saveProject and loadProject', () => {
    it('saves and loads a project', () => {
      const project = createProject('Test Project');
      saveProject(project);
      
      const loaded = loadProject(project.id);
      expect(loaded).not.toBeNull();
      expect(loaded?.name).toBe('Test Project');
      expect(loaded?.id).toBe(project.id);
    });

    it('returns null for non-existent project', () => {
      const loaded = loadProject('non-existent-id');
      expect(loaded).toBeNull();
    });

    it('updates updatedAt when saving', () => {
      const project = createProject('Test Project');
      const originalUpdatedAt = project.updatedAt;
      
      // Simulate time passing
      setTimeout(() => {}, 10);
      saveProject(project);
      
      const loaded = loadProject(project.id);
      expect(loaded?.updatedAt).not.toBe(originalUpdatedAt);
    });
  });

  describe('listProjects', () => {
    it('returns empty array when no projects', () => {
      const projects = listProjects();
      expect(projects).toEqual([]);
    });

    it('returns all projects sorted by updatedAt', () => {
      const project1 = createProject('First');
      const project2 = createProject('Second');
      
      // Manually set different update times
      project1.updatedAt = '2024-01-01T00:00:00.000Z';
      project2.updatedAt = '2024-01-02T00:00:00.000Z';
      
      saveProject(project1);
      saveProject(project2);
      
      const projects = listProjects();
      expect(projects).toHaveLength(2);
      expect(projects[0].name).toBe('Second'); // Most recent first
      expect(projects[1].name).toBe('First');
    });
  });

  describe('addAsset', () => {
    it('adds an asset to the project', () => {
      const project = createProject('Test Project');
      const asset: ProjectAsset = {
        id: 'asset-1',
        kind: 'image',
        name: 'Test Image',
        path: '/path/to/image.png',
        createdAt: new Date().toISOString(),
      };
      
      const updated = addAsset(project, asset);
      
      expect(updated.assets).toHaveLength(1);
      expect(updated.assets[0].id).toBe('asset-1');
      expect(updated.assets[0].name).toBe('Test Image');
      expect(updated.updatedAt).not.toBe(project.updatedAt);
    });

    it('preserves existing assets', () => {
      const project = createProject('Test Project');
      const asset1: ProjectAsset = {
        id: 'asset-1',
        kind: 'image',
        name: 'First Image',
        path: '/path/to/first.png',
        createdAt: new Date().toISOString(),
      };
      const asset2: ProjectAsset = {
        id: 'asset-2',
        kind: 'video',
        name: 'First Video',
        path: '/path/to/video.mp4',
        createdAt: new Date().toISOString(),
      };
      
      const withAsset1 = addAsset(project, asset1);
      const withAsset2 = addAsset(withAsset1, asset2);
      
      expect(withAsset2.assets).toHaveLength(2);
      expect(withAsset2.assets.map(a => a.id)).toContain('asset-1');
      expect(withAsset2.assets.map(a => a.id)).toContain('asset-2');
    });
  });

  describe('addTimelineItem', () => {
    it('adds an item to the correct track', () => {
      const project = createProject('Test Project');
      const videoTrackId = project.tracks.find(t => t.kind === 'video')!.id;
      
      const item: TimelineItem = {
        id: 'item-1',
        assetId: 'asset-1',
        trackId: videoTrackId,
        start: 0,
        duration: 5,
      };
      
      const updated = addTimelineItem(project, item);
      const videoTrack = updated.tracks.find(t => t.kind === 'video');
      
      expect(videoTrack?.items).toHaveLength(1);
      expect(videoTrack?.items[0].id).toBe('item-1');
    });

    it('does not modify other tracks', () => {
      const project = createProject('Test Project');
      const videoTrackId = project.tracks.find(t => t.kind === 'video')!.id;
      
      const item: TimelineItem = {
        id: 'item-1',
        assetId: 'asset-1',
        trackId: videoTrackId,
        start: 0,
        duration: 5,
      };
      
      const updated = addTimelineItem(project, item);
      const audioTrack = updated.tracks.find(t => t.kind === 'audio');
      
      expect(audioTrack?.items).toHaveLength(0);
    });
  });
});
