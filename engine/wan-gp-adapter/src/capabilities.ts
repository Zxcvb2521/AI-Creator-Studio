export type CapabilityStatus = 'available' | 'unavailable' | 'unknown';

export interface EngineCapability {
  id: string;
  label: string;
  status: CapabilityStatus;
  detail?: string;
}

export interface EngineCapabilities {
  engine: string;
  version?: string;
  root: string;
  capabilities: EngineCapability[];
}

export function unknownCapabilities(root: string): EngineCapabilities {
  return {
    engine: 'Wan2GP',
    root,
    capabilities: [
      { id: 'video', label: 'Video generation', status: 'unknown' },
      { id: 'image', label: 'Image generation', status: 'unknown' },
      { id: 'text', label: 'Text / prompt generation', status: 'unknown' },
      { id: 'deepy', label: 'Deepy', status: 'unknown' },
      { id: 'audio', label: 'Audio / TTS', status: 'unknown' },
    ],
  };
}
