export interface ModelSchema {
  modelType: string;
  schema: Record<string, unknown>;
  source: 'wan2gp-api' | 'unavailable';
  error?: string;
}

export function normalizeModelSchema(modelType: string, raw: unknown): ModelSchema {
  if (!raw || typeof raw !== 'object') return { modelType, schema: {}, source: 'unavailable', error: 'WanGP returned no schema' };
  return { modelType, schema: raw as Record<string, unknown>, source: 'wan2gp-api' };
}
