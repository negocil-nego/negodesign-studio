<script lang="ts">
  import DataTable from "../../../../../project/ui/organisms/data-table/ui/DataTable.svelte";
  import type { OpenApiEntity } from "../../../data/migration";

  let { entity }: { entity: OpenApiEntity } = $props();

  const tableProps = $derived({
    title: entity.name,
    columns: entity.fields.map((field) => ({ text: field.name })),
    rows: entity.fields.map((field) => ({ text: field.type })),
  });
</script>

<div class="rounded-lg bg-blender-base p-4">
  <DataTable variant="03" {...tableProps} />
</div>
