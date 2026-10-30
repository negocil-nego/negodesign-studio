import type { FormEvent } from "react";
import type { FormProps, FormIt } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

function Field({ field }: { field: FormIt }) {
  const id = field.id ?? field.inputProps?.id;
  const label = field.labelProps?.label ?? field.label;
  const component = field.component ?? field.type;
  const common = { id, name: field.name ?? field.inputProps?.name, required: field.required ?? field.inputProps?.required, disabled: field.disabled ?? field.inputProps?.disabled, placeholder: field.placeholder ?? field.inputProps?.placeholder, defaultValue: field.value ?? field.inputProps?.value };
  if (component === "checkbox" || component === "toggle") return <label className="flex items-center gap-2 text-sm"><input type="checkbox" defaultChecked={field.checked} disabled={field.disabled} required={field.required} onChange={e => field.onCheckedChange?.(e.target.checked)} />{label}</label>;
  if (component === "radio") return <fieldset><legend>{label}</legend>{field.options?.map((option, i) => <label key={i} className="mr-3 inline-flex gap-2"><input type="radio" name={common.name ?? id} value={option.value} disabled={option.disabled} />{option.label}</label>)}</fieldset>;
  if (component === "select") return <label className="block space-y-1 text-sm">{label}<select {...common} className={StyleComponent.select.trigger}><option value="">{field.placeholder ?? "Seleccione"}</option>{field.options?.map(o => <option key={o.value} value={o.value} disabled={o.disabled}>{o.label}</option>)}</select></label>;
  if (component === "textarea") return <label className="block space-y-1 text-sm">{label}<textarea {...common} rows={field.rows ?? 4} className={StyleComponent.input.form} /></label>;
  const type = field.inputProps?.type ?? (field.type === "email" || field.type === "password" || field.type === "number" || field.type === "tel" ? field.type : "text");
  return <label className="block space-y-1 text-sm">{label}<input {...common} type={type} className={StyleComponent.input.form} /></label>;
}

export default function Form({ style = "grid", className = "", fields = [], onSubmit }: FormProps) {
  const submit = (event: FormEvent) => { event.preventDefault(); onSubmit?.(); };
  return <form className={`w-full grid grid-cols-1 ${style === "grid" ? "md:grid-cols-2" : ""} gap-4 ${className}`} onSubmit={submit}>{fields.map((field, i) => <div key={field.id ?? `${field.component ?? field.type}-${i}`}><Field field={field} /></div>)}</form>;
}
