import type { ProjectDocument } from '../../../../projects/src/model';
import { moveTimelineItem, removeTimelineItem } from '../../../../projects/src/timeline';

interface Props {
  project: ProjectDocument;
  onChange: (project: ProjectDocument) => void;
}

const PX_PER_SECOND = 70;

export function Timeline({ project, onChange }: Props) {
  return (
    <section className="timeline-panel">
      <div className="timeline-head">
        <strong>Timeline</strong>
        <span>{project.tracks.reduce((n, t) => n + t.items.length, 0)} items</span>
      </div>
      <div className="timeline-ruler">
        {[0, 5, 10, 15, 20, 25, 30].map(second => <span key={second} style={{ left: second * PX_PER_SECOND }}>{second}s</span>)}
      </div>
      {project.tracks.map(track => (
        <div className="track" key={track.id}>
          <div className="track-label">{track.name}</div>
          <div className="track-lane">
            {track.items.map(item => {
              const asset = project.assets.find(a => a.id === item.assetId);
              return (
                <button
                  className="timeline-item"
                  key={item.id}
                  title={asset?.name ?? item.assetId}
                  style={{ left: item.start * PX_PER_SECOND, width: Math.max(70, item.duration * PX_PER_SECOND) }}
                  onDoubleClick={() => onChange(removeTimelineItem(project, item.id))}
                  onContextMenu={event => {
                    event.preventDefault();
                    onChange(moveTimelineItem(project, item.id, item.start + 1));
                  }}
                >
                  {asset?.name ?? 'Asset'}
                </button>
              );
            })}
          </div>
        </div>
      ))}
      <small className="timeline-help">Двойной клик — удалить · ПКМ — сдвинуть на 1 сек.</small>
    </section>
  );
}
