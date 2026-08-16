import { describe, it, expect } from 'vitest';
import { resultToAsset, completedJobToProject } from './generation';
import { createProject } from './model';
import type { GenerationResult, JobSnapshot } from '../../engine/wan-gp-adapter/src/types';

describe('generation', () => {
  describe('resultToAsset', () => {
    it('converts video result to asset', () => {
      const result: GenerationResult = {
        jobId: 'job-123',
        kind: 'video',
        files: ['/path/to/output.mp4'],
        duration: 5,
        metadata: { prompt: 'test prompt' },
      };
      
      const asset = resultToAsset(result, 'Generated Video');
      
      expect(asset.kind).toBe('video');
      expect(asset.name).toBe('Generated Video');
      expect(asset.path).toBe('/path/to/output.mp4');
      expect(asset.duration).toBe(5);
      expect(asset.metadata).toEqual({ prompt: 'test prompt' });
    });

    it('uses first file from result', () => {
      const result: GenerationResult = {
        jobId: 'job-123',
        kind: 'image',
        files: ['/path/to/first.png', '/path/to/second.png'],
        metadata: {},
      };
      
      const asset = resultToAsset(result);
      
      expect(asset.path).toBe('/path/to/first.png');
    });

    it('handles empty files array', () => {
      const result: GenerationResult = {
        jobId: 'job-123',
        kind: 'image',
        files: [],
        metadata: {},
      };
      
      const asset = resultToAsset(result);
      
      expect(asset.path).toBe('');
    });

    it('generates unique ID for each asset', () => {
      const result: GenerationResult = {
        jobId: 'job-123',
        kind: 'video',
        files: ['/path/to/output.mp4'],
        metadata: {},
      };
      
      const asset1 = resultToAsset(result);
      const asset2 = resultToAsset(result);
      
      expect(asset1.id).not.toBe(asset2.id);
    });
  });

  describe('completedJobToProject', () => {
    it('adds completed job result to project', () => {
      const project = createProject('Test Project');
      const job: JobSnapshot = {
        id: 'job-123',
        state: 'completed',
        result: {
          jobId: 'job-123',
          kind: 'video',
          files: ['/path/to/output.mp4'],
          duration: 5,
          metadata: {},
        },
      };
      
      const updated = completedJobToProject(project, job, 'Generated Video');
      
      expect(updated.assets).toHaveLength(1);
      expect(updated.assets[0].name).toBe('Generated Video');
      expect(updated.assets[0].kind).toBe('video');
    });

    it('throws error for non-completed job', () => {
      const project = createProject('Test Project');
      const job: JobSnapshot = {
        id: 'job-123',
        state: 'running',
        progress: 50,
      };
      
      expect(() => completedJobToProject(project, job)).toThrow(
        'Only a completed job with a result can be added to a project.'
      );
    });

    it('throws error for completed job without result', () => {
      const project = createProject('Test Project');
      const job: JobSnapshot = {
        id: 'job-123',
        state: 'completed',
        // Missing result
      };
      
      expect(() => completedJobToProject(project, job)).toThrow(
        'Only a completed job with a result can be added to a project.'
      );
    });

    it('updates project updatedAt', () => {
      const project = createProject('Test Project');
      const job: JobSnapshot = {
        id: 'job-123',
        state: 'completed',
        result: {
          jobId: 'job-123',
          kind: 'video',
          files: ['/path/to/output.mp4'],
          metadata: {},
        },
      };
      
      const updated = completedJobToProject(project, job);
      
      expect(updated.updatedAt).not.toBe(project.updatedAt);
    });
  });
});
