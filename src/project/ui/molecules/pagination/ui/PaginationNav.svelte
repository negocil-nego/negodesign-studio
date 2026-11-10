<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { PaginationNavProps } from "../data/types";

  let {
    totalPages = 5,
    currentPage = $bindable(1),
    size = "md",
    disabledFirst = false,
    onPageChange,
  }: PaginationNavProps = $props();

  const pages = $derived(Array.from({ length: totalPages }, (_, idx) => idx + 1));

  function setPage(next: number) {
    if (next < 1 || next > totalPages) return;
    currentPage = next;
    onPageChange?.(currentPage);
  }
</script>

<nav aria-label="Pagination" class={StyleComponent.pagination.nav}>
  <ul class={`${StyleComponent.pagination.list} ${StyleComponent.pagination.size[size]}`}>
    <li class={StyleComponent.pagination.item}>
      <button
        type="button"
        disabled={disabledFirst || currentPage <= 1}
        onclick={() => setPage(currentPage - 1)}
        class={StyleComponent.pagination.link}
        aria-label="Previous"
      >
        &laquo;
      </button>
    </li>
    {#each pages as page (page)}
      {@const active = page === currentPage}
      <li class={`${StyleComponent.pagination.item} ${active ? StyleComponent.pagination.itemActive : ""}`}>
        <button
          type="button"
          onclick={() => setPage(page)}
          class={`${StyleComponent.pagination.link} ${active ? StyleComponent.pagination.linkActive : ""}`}
        >
          {page}
        </button>
      </li>
    {/each}
    <li class={StyleComponent.pagination.item}>
      <button
        type="button"
        disabled={currentPage >= totalPages}
        onclick={() => setPage(currentPage + 1)}
        class={StyleComponent.pagination.link}
        aria-label="Next"
      >
        &raquo;
      </button>
    </li>
  </ul>
</nav>
