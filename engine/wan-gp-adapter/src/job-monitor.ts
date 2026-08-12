import type { JobSnapshot } from './types';
import { WanGPBridgeClient } from './bridge-client';

export type JobListener = (job: JobSnapshot) => void;

export class JobMonitor {
  private timers = new Map<string, ReturnType<typeof setInterval>>();

  constructor(private readonly client: WanGPBridgeClient) {}

  watch(id: string, listener: JobListener, intervalMs = 500): void {
    this.stop(id);
    const poll = async () => {
      try {
        const job = await this.client.getJob(id);
        listener(job);
        if (['completed', 'failed', 'cancelled'].includes(job.state)) this.stop(id);
      } catch (error) {
        listener({ id, state: 'failed', error: error instanceof Error ? error.message : String(error) });
        this.stop(id);
      }
    };
    void poll();
    this.timers.set(id, setInterval(() => void poll(), intervalMs));
  }

  stop(id: string): void {
    const timer = this.timers.get(id);
    if (timer) clearInterval(timer);
    this.timers.delete(id);
  }

  stopAll(): void {
    for (const id of this.timers.keys()) this.stop(id);
  }
}
