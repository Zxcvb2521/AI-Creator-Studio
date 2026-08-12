export type MediaKind = 'image' | 'video' | 'audio' | 'voice';

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
  metadata: Record<string, unknown>;
}

export interface JobSnapshot {
  id: string;
  state: 'queued' | 'running' | 'completed' | 'failed' | 'cancelled';
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

export interface WanGPAdapter {
  capabilities(): Promise<{ media: MediaKind[]; deepy: DeepyCapability }>;
  generate(request: GenerationRequest): Promise<JobSnapshot>;
  getJob(jobId: string): Promise<JobSnapshot>;
  cancelJob(jobId: string): Promise<void>;
}
