<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  import type { ToggleProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";

  type Props = ToggleProps & Omit<HTMLButtonAttributes, keyof ToggleProps | "type" | "class"> & { class?: string };
  let {
    id, name, value, checked = $bindable(false), disabled, label,
    onCheckedChange, onclick: onNativeClick, class: buttonClass, ...rest
  }: Props = $props();

  function handleClick(event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
    if (disabled) return;
    checked = !checked;
    onNativeClick?.(event);
    onCheckedChange?.(checked);
  }
</script>

<label class={`${StyleComponent.toggle.root} ${disabled ? "opacity-50 cursor-not-allowed" : ""}`} for={id}>
  <button
    {...rest}
    type="button"
    {id}
    {name}
    {value}
    role="switch"
    aria-checked={checked}
    {disabled}
    onclick={handleClick}
    class={`${StyleComponent.toggle.track} ${checked ? StyleComponent.toggle.on : ""} ${buttonClass ?? ""}`}
  >
    <span class={`${StyleComponent.toggle.thumb} ${checked ? StyleComponent.toggle.thumbOn : ""}`}></span>
  </button>
  {#if label}<span class={StyleComponent.toggle.label}>{typeof label === "string" ? label : label.label}</span>{/if}
</label>
