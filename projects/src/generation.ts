import type { GenerationResult, JobSnapshot } from '../../engine/wan-gp-adapter/src/types';
import type { ProjectAsset, ProjectDocument } from './model';
import { addAsset } from './store';

export function resultToAsset(result: GenerationResult, name = 'Generated asset'): ProjectAsset {
  const first = result.files[0] ?? '';
  return {
    id: crypto.randomUUID(),
    kind: result.kind,
    name,
    path: first,
    duration: result.duration,
    createdAt: new Date().toISOString(),
    metadata: result.metadata,
  };
}

export function completedJobToProject(project: ProjectDocument, job: JobSnapshot, name?: string): ProjectDocument {
  if (job.state !== 'completed' || !job.result) {
    throw new Error('Only a completed job with a result can be added to a project.');
  }
  return addAsset(project, resultToAsset(job.result, name));
}
