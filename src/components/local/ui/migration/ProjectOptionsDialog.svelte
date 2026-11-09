<script lang="ts">
  import { goto } from "$app/navigation";
  import { LocalStyleComponent } from "../../data/style.component";
  import MigrationSection from "./MigrationSection.svelte";

  let { open = false, onclose }: { open?: boolean; onclose: () => void } =
    $props();

  function go(path: string) {
    onclose();
    void goto(path);
  }
</script>

{#if open}
  <div
    class={LocalStyleComponent.migration.overlay}
    role="presentation"
    onclick={(event) => event.target === event.currentTarget && onclose()}
    onkeydown={(event) => event.key === "Escape" && onclose()}
  >
    <div
      class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-xl border border-blender-border bg-white p-6 text-slate-900 shadow-2xl dark:bg-blender-card dark:text-zinc-100"
      role="dialog"
      aria-modal="true"
      aria-labelledby="project-options-title"
    >
      <!-- Header -->
      <div class="mb-6 flex items-start justify-between gap-4">
        <div>
          <p
            class="text-xs font-semibold uppercase tracking-wider text-blue-500"
          >
            Projecto
          </p>
          <h2 id="project-options-title" class="mt-1 text-xl font-semibold">
            Como quer começar?
          </h2>
          <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
            Escolha uma opção para criar ou migrar o seu projecto.
          </p>
        </div>
        <button
          type="button"
          class="rounded p-2 text-zinc-500 hover:bg-blender-hover"
          aria-label="Fechar"
          onclick={onclose}>✕</button
        >
      </div>

      <!-- Frontend -->
      <div class="mb-6">
        <h3 class={LocalStyleComponent.migration.sectionTitle}>Frontend</h3>
        <div class="grid gap-3 sm:grid-cols-2">
          <button
            type="button"
            class={LocalStyleComponent.migration.optionCard}
            onclick={() => go("/main/template")}
          >
            <span
              class={LocalStyleComponent.migration.optionIcon}
              aria-hidden="true">▦</span
            >
            <span class={LocalStyleComponent.migration.optionTitle}
              >Layouts</span
            >
            <span class={LocalStyleComponent.migration.optionHint}
              >Conheça os layouts prontos da plataforma.</span
            >
          </button>
          <button
            type="button"
            class={LocalStyleComponent.migration.optionCard}
            onclick={() => go("/main/project")}
          >
            <span
              class={LocalStyleComponent.migration.optionIcon}
              aria-hidden="true">◫</span
            >
            <span class={LocalStyleComponent.migration.optionTitle}
              >Componentes</span
            >
            <span class={LocalStyleComponent.migration.optionHint}
              >Explore os componentes disponíveis.</span
            >
          </button>
        </div>
      </div>

      <!-- Migração de Projecto (Banco de dados + ApiDocs) -->
      <MigrationSection {onclose} />
    </div>
  </div>
{/if}
