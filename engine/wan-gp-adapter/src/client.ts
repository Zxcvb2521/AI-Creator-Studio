import type { EngineCapabilities, GenerationRequest, JobSnapshot, WanGPAdapter } from './types';

export interface WanGPTransport {
  request<T>(method: string, path: string, body?: unknown): Promise<T>;
}

export class HttpWanGPTransport implements WanGPTransport {
  constructor(private readonly baseUrl = 'http://127.0.0.1:7860') {}

  async request<T>(method: string, path: string, body?: unknown): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    if (!response.ok) {
      throw new Error(`WanGP request failed: ${response.status} ${response.statusText}`);
    }
    return await response.json() as T;
  }
}

export class WanGPClient implements WanGPAdapter {
  constructor(private readonly transport: WanGPTransport) {}

  capabilities(): Promise<EngineCapabilities> {
    return this.transport.request<EngineCapabilities>('GET', '/api/studio/capabilities');
  }

  generate(request: GenerationRequest): Promise<JobSnapshot> {
    return this.transport.request<JobSnapshot>('POST', '/api/studio/jobs', request);
  }

  getJob(id: string): Promise<JobSnapshot> {
    return this.transport.request<JobSnapshot>('GET', `/api/studio/jobs/${encodeURIComponent(id)}`);
  }

  async cancelJob(id: string): Promise<void> {
    await this.transport.request('POST', `/api/studio/jobs/${encodeURIComponent(id)}/cancel`);
  }
}
