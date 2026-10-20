<script lang="ts">
  type PropRow = {
    label: string;
    value: string;
    type?: "text" | "color" | "number";
  };
  type PropSection = { title: string; rows: PropRow[] };

  let sections = $state<PropSection[]>([
    { title: "Identity", rows: [{ label: "fx:id", value: "", type: "text" }] },
    {
      title: "Style",
      rows: [
        { label: "Style Class", value: "", type: "text" },
        { label: "Style", value: "", type: "text" },
      ],
    },
    {
      title: "DragDrop",
      rows: [
        { label: "On Drag Detected", value: "", type: "text" },
        { label: "On Drag Done", value: "", type: "text" },
        { label: "On Drag Dropped", value: "", type: "text" },
        { label: "On Drag Entered", value: "", type: "text" },
        { label: "On Drag Exited", value: "", type: "text" },
        { label: "On Drag Over", value: "", type: "text" },
      ],
    },
    {
      title: "Keyboard",
      rows: [
        { label: "On Key Pressed", value: "", type: "text" },
        { label: "On Key Released", value: "", type: "text" },
        { label: "On Key Typed", value: "", type: "text" },
      ],
    },
    {
      title: "Mouse",
      rows: [
        { label: "On Mouse Clicked", value: "", type: "text" },
        { label: "On Mouse Entered", value: "", type: "text" },
        { label: "On Mouse Exited", value: "", type: "text" },
      ],
    },
  ]);
</script>

<div class="flex flex-col h-full overflow-y-auto">
  {#each sections as section}
    <div class="border-b border-blender-border/40">
      <div class="px-3 py-1 bg-blender-base/60 text-right">
        <span
          class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500"
          >{section.title}</span
        >
      </div>
      {#each section.rows as row, i (i)}
        <div
          class="flex items-center gap-2 px-2 py-1 hover:bg-blender-hover/40 transition-colors group"
        >
          <!-- svelte-ignore a11y_label_has_associated_control -->
          <label
            class="text-[10px] text-blue-400 w-32.51 shrink-0 truncate"
            title={row.label}
          >
            {row.label}
          </label>
          <div class="flex items-center gap-1 flex-1 min-w-0">
            <span class="text-zinc-600 text-[10px]">#</span>
            <input
              bind:value={row.value}
              placeholder=""
              class="flex-1 min-w-0 bg-blender-base border border-blender-border/60 rounded px-1.5 py-0.5
                     text-[10px] text-zinc-300 focus:outline-none focus:border-blue-500/50 transition-colors"
            />
          </div>
        </div>
      {/each}
    </div>
  {/each}
</div>
