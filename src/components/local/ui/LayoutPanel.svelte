<script lang="ts">
  type LayoutProp = { label: string; value: string; type?: 'text'|'select'|'number'; options?: string[] };
  type LayoutSection = { title: string; props: LayoutProp[] };

  let sections = $state<LayoutSection[]>([
    { title: 'Anchor Pane Constraints', props: [
        { label: 'Bottom Anchor', value: '', type: 'number' },
        { label: 'Left Anchor', value: '', type: 'number' },
        { label: 'Right Anchor', value: '', type: 'number' },
        { label: 'Top Anchor', value: '', type: 'number' },
      ]
    },
    { title: 'Dimension', props: [
        { label: 'Min Width', value: 'USE_COMPUTED', type: 'text' },
        { label: 'Pref Width', value: 'USE_COMPUTED', type: 'text' },
        { label: 'Max Width', value: 'USE_COMPUTED', type: 'text' },
        { label: 'Min Height', value: 'USE_COMPUTED', type: 'text' },
        { label: 'Pref Height', value: 'USE_COMPUTED', type: 'text' },
        { label: 'Max Height', value: 'USE_COMPUTED', type: 'text' },
      ]
    },
    { title: 'Position', props: [
        { label: 'Layout X', value: '0', type: 'number' },
        { label: 'Layout Y', value: '0', type: 'number' },
        { label: 'Translate X', value: '0', type: 'number' },
        { label: 'Translate Y', value: '0', type: 'number' },
        { label: 'Translate Z', value: '0', type: 'number' },
      ]
    },
    { title: 'Margin & Padding', props: [
        { label: 'Top', value: '0', type: 'number' },
        { label: 'Right', value: '0', type: 'number' },
        { label: 'Bottom', value: '0', type: 'number' },
        { label: 'Left', value: '0', type: 'number' },
      ]
    },
  ]);
</script>

<div class="flex flex-col h-full overflow-y-auto">
  {#each sections as section}
    <div class="border-b border-blender-border/40">
      <div class="px-3 py-1 bg-blender-base/60 text-right">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">{section.title}</span>
      </div>
      {#each section.props as prop, i (i)}
        {@const inputId = `layout-${section.title}-${i}`}
        <div class="flex items-center gap-2 px-2 py-1 hover:bg-blender-hover/40 transition-colors">
          <label
            for={inputId}
            class="text-[10px] text-zinc-400 flex-1 truncate cursor-pointer"
            title={prop.label}
          >{prop.label}</label>
          <input
            id={inputId}
            bind:value={prop.value}
            type={prop.type === 'number' ? 'number' : 'text'}
            class="w-[90px] bg-blender-base border border-blender-border/60 rounded px-1.5 py-0.5
                   text-[10px] text-zinc-300 text-right focus:outline-none focus:border-blue-500/50 transition-colors"
          />
        </div>
      {/each}
    </div>
  {/each}
</div>
