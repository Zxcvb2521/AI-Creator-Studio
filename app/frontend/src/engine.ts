import type { } from '@tauri-apps/api/core';

export type EngineCapability = {
  id: string;
  label?: string;
  description?: string;
  available?: boolean;
};

export type EngineCapabilities = {
  engine?: string;
  ready?: boolean;
  capabilities: EngineCapability[];
};

export type JobSnapshot = {
  id: string;
  state: 'queued' | 'running' | 'completed' | 'failed' | 'cancelled';
  message: string;
  result?: unknown;
};

export type GenerationRequest = {
  model_type: string;
  settings: Record<string, unknown>;
};

export async function engineStatus(): Promise<EngineCapabilities> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<EngineCapabilities>('engine_status');
}

export async function engineStart(): Promise<string> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<string>('start_engine');
}

export async function engineStop(): Promise<string> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<string>('stop_engine');
}
