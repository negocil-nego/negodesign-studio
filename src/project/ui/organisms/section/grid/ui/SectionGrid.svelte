<script lang="ts">
  import type { SectionGridProps } from "../data/types";

  let { titleProps, descriptionProps, className = "", children }: SectionGridProps = $props();
</script>

<section class={`w-full min-w-0 py-6 ${className}`.trim()}>
  {#if titleProps || descriptionProps}
    <header class="mb-6">
      {#if titleProps}
        <h2 class="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl">
          {titleProps.text}
        </h2>
      {/if}
      {#if descriptionProps}
        <p class="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">
          {descriptionProps.text}
        </p>
      {/if}
    </header>
  {/if}

  <div class="grid grid-cols-1 gap-2 sm:grid-cols-2 md:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 [&>*]:w-full [&>*]:min-w-0">
    {@render children()}
  </div>
</section>
