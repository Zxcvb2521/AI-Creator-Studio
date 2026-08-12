import type { EngineCapabilities, JobSnapshot, GenerationRequest } from '../../engine/wan-gp-adapter/src/types';

export async function engineStatus(): Promise<unknown> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke('engine_status');
}

export async function engineStart(): Promise<string> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<string>('start_engine');
}

export async function engineStop(): Promise<string> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<string>('stop_engine');
}

export type { EngineCapabilities, JobSnapshot, GenerationRequest };
