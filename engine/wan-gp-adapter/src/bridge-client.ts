import type { EngineCapabilities, GenerationRequest, JobSnapshot, WanGPAdapter } from './types';

/** Client for the Studio bridge already prototyped against WanGP's headless session API. */
export class WanGPBridgeClient implements WanGPAdapter {
  constructor(private readonly baseUrl = 'http://127.0.0.1:18765') {}

  private async request<T>(path: string, init?: RequestInit): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, {
      ...init,
      headers: { 'Content-Type': 'application/json', ...(init?.headers ?? {}) },
    });
    if (!response.ok) throw new Error(`WanGP bridge: ${response.status} ${response.statusText}`);
    return await response.json() as T;
  }

  async capabilities(): Promise<EngineCapabilities> {
    const [health, models, deepy] = await Promise.all([
      this.request<{ engine?: string }>('/health'),
      this.request<{ models?: Array<Record<string, unknown>> }>('/models'),
      this.request<{ deepy?: Record<string, unknown> }>('/deepy'),
    ]);
    const metadata = models.models ?? [];
    const text = JSON.stringify(metadata).toLowerCase();
    return {
      engine: health.engine ?? 'WanGP',
      image: /image|t2i/.test(text),
      video: /video|t2v|wan|ltx/.test(text),
      audio: /audio|ace|music/.test(text),
      voice: /tts|voice|speech/.test(text),
      deepy: Boolean(deepy.deepy?.available ?? true),
      models: metadata.map(m => String(m.model_type ?? m.name ?? '')).filter(Boolean),
    };
  }

  async generate(request: GenerationRequest): Promise<JobSnapshot> {
    if (request.kind !== 'image' && request.kind !== 'video') {
      throw new Error(`WanGP bridge image/video path is ready; ${request.kind} will use its dedicated audio/TTS adapter.`);
    }
    const path = request.kind === 'video' ? '/generate/video' : '/generate/image';
    const created = await this.request<{ job_id: string }>(path, {
      method: 'POST',
      body: JSON.stringify({ prompt: request.prompt ?? '', settings: { ...(request.parameters ?? {}), ...(request.model ? { model_type: request.model } : {}) } }),
    });
    return this.getJob(created.job_id);
  }

  async getJob(id: string): Promise<JobSnapshot> {
    const job = await this.request<Record<string, unknown> & { id: string; status: string; progress?: number; error?: string }>(`/jobs/${encodeURIComponent(id)}`);
    const state = job.status === 'completed' ? 'completed' : job.status === 'failed' ? 'failed' : job.status === 'cancelled' ? 'cancelled' : job.status === 'queued' ? 'queued' : 'running';
    return { id: job.id, state, progress: job.progress, message: String(job.phase ?? ''), result: job.result as JobSnapshot['result'], error: job.error };
  }

  async cancelJob(id: string): Promise<void> {
    await this.request(`/jobs/${encodeURIComponent(id)}/cancel`, { method: 'POST', body: '{}' });
  }
}
