<script lang="ts">
  import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import UnderlineNav from "../../../../../molecules/underline-nav/ui/UnderlineNav.svelte";
  import type { ProductTabsProps } from "../data/types";

  let {
    underlineNavProps,
    titleProps,
    descriptionProps,
    paragraphs = [],
    specs = [],
    highlights = [],
    tabContents = [],
    specsStyle = "inline",
    className = "",
  }: ProductTabsProps = $props();
  const defaultNavItems = [
    { value: "description", label: "Description" },
    { value: "reviews", label: "Reviews (150)" },
    { value: "support", label: "Support" },
  ];
  const navItems = $derived(underlineNavProps?.items ?? defaultNavItems);
  let activeTab = $state("");
  const resolvedTab = $derived(
    activeTab || underlineNavProps?.value || navItems[0]?.value || "description",
  );
  const activeContent = $derived(
    tabContents.find((t) => t.value === resolvedTab),
  );

  const currentTitle = $derived(activeContent?.titleProps ?? titleProps);
  const currentDescription = $derived(activeContent?.descriptionProps ?? descriptionProps);
  const currentParagraphs = $derived(activeContent?.paragraphs ?? paragraphs);
  const currentSpecs = $derived(activeContent?.specs ?? specs);
  const currentHighlights = $derived(activeContent?.highlights ?? highlights);
</script>

<div class={`space-y-5 ${className}`}>
  <UnderlineNav
    items={navItems}
    value={resolvedTab}
    onValueChange={(val) => {
      activeTab = val;
      underlineNavProps?.onValueChange?.(val);
    }}
  />

  <div class="space-y-4 pt-1">
    {#if currentTitle}
      <CardTitle {...currentTitle} className={currentTitle.className ?? "text-sm font-semibold text-zinc-100"} />
    {/if}

    {#if currentSpecs.length > 0}
      {#if specsStyle === "table"}
        <div class="divide-y divide-blender-border/60 rounded-xl border border-blender-border bg-blender-surface/40">
          {#each currentSpecs as spec (spec.label)}
            <div class="grid grid-cols-2 gap-4 px-4 py-2.5 text-xs">
              <span class="text-zinc-400">{spec.label}</span>
              <span class="font-medium text-zinc-200">{spec.value}</span>
            </div>
          {/each}
        </div>
      {:else}
        <dl class="grid gap-2 text-xs sm:grid-cols-2">
          {#each currentSpecs as spec (`${spec.label}-${spec.value}`)}
            <div class="flex items-baseline gap-2">
              <dt class="font-medium text-zinc-400">{spec.label}:</dt>
              <dd class="text-zinc-200">{spec.value}</dd>
            </div>
          {/each}
        </dl>
      {/if}
    {/if}

    {#if currentDescription}
      <CardDescription
        {...currentDescription}
        className={currentDescription.className ?? "text-xs leading-relaxed text-zinc-300"}
      />
    {/if}

    {#if currentParagraphs.length > 0}
      <div class="space-y-3 text-xs leading-relaxed text-zinc-400">
        {#each currentParagraphs as paragraph (paragraph)}
          <p>{paragraph}</p>
        {/each}
      </div>
    {/if}

    {#if currentHighlights.length > 0}
      <ul class="list-disc space-y-1.5 pl-5 text-xs text-zinc-300">
        {#each currentHighlights as item (item)}
          <li>{item}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>
