import { useMemo } from 'react';

type Props = { schema: Record<string, unknown>; values: Record<string, unknown>; onChange: (values: Record<string, unknown>) => void };

export function SchemaForm({ schema, values, onChange }: Props) {
  const fields = useMemo(() => {
    const properties = schema.properties;
    if (!properties || typeof properties !== 'object') return [];
    return Object.entries(properties as Record<string, any>);
  }, [schema]);

  return <div className="schema-form">{fields.map(([name, field]) => {
    const type = field?.type;
    const title = field?.title ?? name;
    const value = values[name] ?? field?.default ?? '';
    if (type === 'boolean') return <label className="schema-field" key={name}><span>{title}</span><input type="checkbox" checked={Boolean(value)} onChange={e => onChange({ ...values, [name]: e.target.checked })} /></label>;
    if (type === 'integer' || type === 'number') return <label className="schema-field" key={name}><span>{title}</span><input type="number" value={String(value)} min={field?.minimum} max={field?.maximum} step={type === 'integer' ? 1 : 'any'} onChange={e => onChange({ ...values, [name]: type === 'integer' ? Number.parseInt(e.target.value, 10) : Number.parseFloat(e.target.value) })} /></label>;
    if (Array.isArray(field?.enum)) return <label className="schema-field" key={name}><span>{title}</span><select value={String(value)} onChange={e => onChange({ ...values, [name]: e.target.value })}>{field.enum.map((option: unknown) => <option key={String(option)} value={String(option)}>{String(option)}</option>)}</select></label>;
    return <label className="schema-field" key={name}><span>{title}</span><input value={String(value)} onChange={e => onChange({ ...values, [name]: e.target.value })} /></label>;
  })}</div>;
}
