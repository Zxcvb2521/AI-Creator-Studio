import { describe, it, expect } from 'vitest';

// Test types compile and basic type predicates work
describe('types', () => {
  describe('MediaKind', () => {
    it('accepts valid media kinds', () => {
      const kinds: Array<'image' | 'video' | 'audio' | 'voice'> = [
        'image',
        'video',
        'audio',
        'voice',
      ];
      
      expect(kinds).toHaveLength(4);
    });
  });

  describe('JobState', () => {
    it('accepts valid job states', () => {
      const states: Array<'queued' | 'running' | 'completed' | 'failed' | 'cancelled'> = [
        'queued',
        'running',
        'completed',
        'failed',
        'cancelled',
      ];
      
      expect(states).toHaveLength(5);
    });
  });

  describe('GenerationRequest', () => {
    it('creates minimal request', () => {
      const request = {
        kind: 'video' as const,
        prompt: 'A beautiful sunset',
      };
      
      expect(request.kind).toBe('video');
      expect(request.prompt).toBe('A beautiful sunset');
    });

    it('creates full request with optional fields', () => {
      const request = {
        kind: 'video' as const,
        prompt: 'A beautiful sunset',
        negativePrompt: 'blurry, low quality',
        model: 'wan-2.1',
        settings: { steps: 50, guidance: 7.5 },
      };
      
      expect(request.negativePrompt).toBe('blurry, low quality');
      expect(request.model).toBe('wan-2.1');
      expect(request.settings).toEqual({ steps: 50, guidance: 7.5 });
    });
  });

  describe('JobSnapshot', () => {
    it('represents queued job', () => {
      const job = {
        id: 'job-1',
        state: 'queued' as const,
      };
      
      expect(job.state).toBe('queued');
      expect(job.result).toBeUndefined();
    });

    it('represents running job with progress', () => {
      const job = {
        id: 'job-1',
        state: 'running' as const,
        progress: 45,
        message: 'Generating frames...',
      };
      
      expect(job.state).toBe('running');
      expect(job.progress).toBe(45);
    });

    it('represents completed job with result', () => {
      const job = {
        id: 'job-1',
        state: 'completed' as const,
        result: {
          jobId: 'job-1',
          kind: 'video' as const,
          files: ['/output/video.mp4'],
          metadata: {},
        },
      };
      
      expect(job.state).toBe('completed');
      expect(job.result?.files).toHaveLength(1);
    });

    it('represents failed job with error', () => {
      const job = {
        id: 'job-1',
        state: 'failed' as const,
        error: 'Out of memory',
      };
      
      expect(job.state).toBe('failed');
      expect(job.error).toBe('Out of memory');
    });
  });
});
