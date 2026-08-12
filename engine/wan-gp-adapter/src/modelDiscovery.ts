export type ModelAvailability = 'available' | 'unavailable' | 'unknown';

export interface WanModel {
  id: string;
  label: string;
  kind: string;
  availability: ModelAvailability;
  metadata?: Record<string, unknown>;
}

export interface ModelCatalog {
  engine: 'Wan2GP';
  models: WanModel[];
  source: 'wan2gp-api' | 'fallback';
  error?: string;
}

export function normalizeModelCatalog(raw: unknown): ModelCatalog {
  if (!Array.isArray(raw)) return { engine: 'Wan2GP', models: [], source: 'fallback', error: 'WanGP returned no model list' };
  const models: WanModel[] = raw.map((item: any, index) => ({
    id: String(item?.id ?? item?.name ?? `model-${index}`),
    label: String(item?.label ?? item?.title ?? item?.name ?? item?.id ?? `Model ${index + 1}`),
    kind: String(item?.kind ?? item?.type ?? item?.task ?? 'unknown'),
    availability: item?.available === false ? 'unavailable' : 'available',
    metadata: item && typeof item === 'object' ? item : undefined,
  }));
  return { engine: 'Wan2GP', models, source: 'wan2gp-api' };
}
