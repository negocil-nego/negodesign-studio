<script lang="ts">
  type Props = {
    showOnStartup: boolean;
  };

  let { showOnStartup = $bindable() }: Props = $props();
</script>

<footer class="blender-footer">
  <label class="flex items-center gap-2 cursor-pointer hover:text-zinc-200 transition-colors">
    <input
      type="checkbox"
      bind:checked={showOnStartup}
      class="w-3.5 h-3.5 rounded border-zinc-750 bg-zinc-800 text-blue-600 focus:ring-0 cursor-pointer"
    />
    <span>Mostrar tela ao iniciar o NegoDesign</span>
  </label>

  <div class="flex items-center gap-3">
    <a href="/page/docs" class="hover:text-blue-400 transition-colors">Documentação</a>
    <span class="text-zinc-700">•</span>
    <a href="/page/help" class="hover:text-blue-400 transition-colors">Ajuda</a>
    <span class="text-zinc-700">•</span>
    <a href="https://github.com" target="_blank" rel="noopener noreferrer" class="hover:text-blue-400 transition-colors">GitHub</a>
    <span class="text-zinc-700">•</span>
    <span class="text-zinc-400 font-mono text-[10px]">MIT License</span>
  </div>
</footer>
