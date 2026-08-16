import { describe, it, expect } from 'vitest';
import { unknownCapabilities } from './capabilities';

describe('capabilities', () => {
  describe('unknownCapabilities', () => {
    it('creates capabilities with unknown status', () => {
      const caps = unknownCapabilities('/path/to/engine');
      
      expect(caps.engine).toBe('Wan2GP');
      expect(caps.root).toBe('/path/to/engine');
      expect(caps.version).toBeUndefined();
    });

    it('includes all expected capability IDs', () => {
      const caps = unknownCapabilities('/path/to/engine');
      const capabilityIds = caps.capabilities.map(c => c.id);
      
      expect(capabilityIds).toContain('video');
      expect(capabilityIds).toContain('image');
      expect(capabilityIds).toContain('text');
      expect(capabilityIds).toContain('deepy');
      expect(capabilityIds).toContain('audio');
    });

    it('sets all capabilities to unknown status', () => {
      const caps = unknownCapabilities('/path/to/engine');
      
      caps.capabilities.forEach(cap => {
        expect(cap.status).toBe('unknown');
      });
    });

    it('includes labels for each capability', () => {
      const caps = unknownCapabilities('/path/to/engine');
      
      expect(caps.capabilities.find(c => c.id === 'video')?.label).toBe('Video generation');
      expect(caps.capabilities.find(c => c.id === 'image')?.label).toBe('Image generation');
      expect(caps.capabilities.find(c => c.id === 'text')?.label).toBe('Text / prompt generation');
      expect(caps.capabilities.find(c => c.id === 'deepy')?.label).toBe('Deepy');
      expect(caps.capabilities.find(c => c.id === 'audio')?.label).toBe('Audio / TTS');
    });

    it('creates different instances for different roots', () => {
      const caps1 = unknownCapabilities('/path/one');
      const caps2 = unknownCapabilities('/path/two');
      
      expect(caps1.root).toBe('/path/one');
      expect(caps2.root).toBe('/path/two');
    });
  });
});
