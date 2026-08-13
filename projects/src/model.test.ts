import { describe, it, expect } from 'vitest';
import { createProject } from './model';

describe('createProject', () => {
  it('creates a project with default name', () => {
    const project = createProject();
    expect(project.name).toBe('Untitled Project');
    expect(project.schemaVersion).toBe(1);
    expect(project.assets).toEqual([]);
    expect(project.tracks).toHaveLength(3);
  });

  it('creates a project with custom name', () => {
    const project = createProject('My Video');
    expect(project.name).toBe('My Video');
  });

  it('initializes with three default tracks', () => {
    const project = createProject();
    const trackKinds = project.tracks.map(t => t.kind);
    expect(trackKinds).toContain('video');
    expect(trackKinds).toContain('voice');
    expect(trackKinds).toContain('audio');
  });

  it('sets createdAt and updatedAt to the same value', () => {
    const project = createProject();
    expect(project.createdAt).toBe(project.updatedAt);
  });

  it('generates unique IDs for project and tracks', () => {
    const project1 = createProject();
    const project2 = createProject();
    expect(project1.id).not.toBe(project2.id);
    expect(project1.tracks[0].id).not.toBe(project2.tracks[0].id);
  });
});
