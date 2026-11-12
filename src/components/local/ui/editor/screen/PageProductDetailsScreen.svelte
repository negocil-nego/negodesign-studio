<script lang="ts">
  import { productDetailsTemplates } from "../../../data/product-details-templates";
  import PageProductDetails from "../../../../../project/ui/pages/product-details/ui/PageProductDetails.svelte";
  import ComponentScreenGroup from "../shared/ComponentScreenGroup.svelte";
  import { templateScreenNavigation } from "../shared/template-screen-navigation";
  import SourceCode from "../terminal/SourceCode.svelte";
  import Terminal from "../terminal/Terminal.svelte";

  import pageProductDetailsSvelte from "../../../../../project/ui/pages/product-details/ui/PageProductDetails.svelte?raw";
  import pageProductDetailsReact from "../../../../../project/ui/pages/product-details/ui/PageProductDetails.tsx?raw";
  import pageProductDetailsVue from "../../../../../project/ui/pages/product-details/ui/PageProductDetails.vue?raw";

  const group = templateScreenNavigation.find(
    (item) => item.type === "product-details",
  )!;

  const templates = Object.keys(productDetailsTemplates).map((name) => ({
    id: `page-product-details-${name
      .replace("PageProductDetails · ", "")
      .toLowerCase()
      .replace(/\s+/g, "-")}`,
    name,
  }));
</script>

<div class="space-y-5">
  <ComponentScreenGroup
    label={group.label}
    badge={group.badge}
    description={group.description}
    count={group.count}
  />

  {#each templates as template (template.id)}
    <Terminal
      id={template.id}
      componentId="page-product-details"
      name={template.name}
    >
      {#snippet preview()}
        <article
          class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-blender-border bg-blender-base"
        >
          <div
            class="relative max-h-[980px] overflow-y-auto bg-blender-base"
          >
            <div
              class="origin-top-left scale-75"
              style="width: 133.333%; height: 133.333%;"
            >
              <PageProductDetails
                {...productDetailsTemplates[template.name]}
              />
            </div>
            <div
              class="pointer-events-none absolute inset-x-0 bottom-0 h-8 bg-linear-to-t from-blender-base/60 to-transparent"
            ></div>
          </div>
        </article>
      {/snippet}
      {#snippet svelte()}
        <SourceCode
          path="src/project/ui/pages/product-details/ui/PageProductDetails.svelte"
          source={pageProductDetailsSvelte}
        />
      {/snippet}
      {#snippet react()}
        <SourceCode
          path="src/project/ui/pages/product-details/ui/PageProductDetails.tsx"
          source={pageProductDetailsReact}
        />
      {/snippet}
      {#snippet vue()}
        <SourceCode
          path="src/project/ui/pages/product-details/ui/PageProductDetails.vue"
          source={pageProductDetailsVue}
        />
      {/snippet}
    </Terminal>
  {/each}
</div>
