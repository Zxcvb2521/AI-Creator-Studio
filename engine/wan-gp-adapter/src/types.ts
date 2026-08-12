export type MediaKind = 'image' | 'video' | 'audio' | 'voice';
export type JobState = 'queued' | 'running' | 'completed' | 'failed' | 'cancelled';

export interface GenerationRequest {
  kind: MediaKind;
  prompt: string;
  negativePrompt?: string;
  model?: string;
  settings?: Record<string, unknown>;
}

export interface GenerationResult {
  jobId: string;
  kind: MediaKind;
  files: string[];
  duration?: number;
  metadata: Record<string, unknown>;
}

export interface JobSnapshot {
  id: string;
  state: JobState;
  progress?: number;
  message?: string;
  result?: GenerationResult;
  error?: string;
}

export interface DeepyCapability {
  available: boolean;
  version?: string;
  entrypoint?: string;
}

export interface EngineCapabilities {
  media: MediaKind[];
  deepy: DeepyCapability;
  engine: string;
  version?: string;
}

export interface WanGPAdapter {
  capabilities(): Promise<EngineCapabilities>;
  generate(request: GenerationRequest): Promise<JobSnapshot>;
  getJob(jobId: string): Promise<JobSnapshot>;
  cancelJob(jobId: string): Promise<void>;
}
