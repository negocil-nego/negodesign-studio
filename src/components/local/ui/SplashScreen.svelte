<script lang="ts">
  import SplashHeader from "./splash/SplashHeader.svelte";
  import SplashActions from "./splash/SplashActions.svelte";
  import SplashEcosystem from "./splash/SplashEcosystem.svelte";
  import SplashFooter from "./splash/SplashFooter.svelte";
  import { StyleComponent } from "../data/style.component";

  let showOnStartup = $state(true);
</script>

<!-- Card SplashScreen responsivo estilo Blender que ocupa o espaço disponível na janela -->
<div
  class="{StyleComponent.layout.card} w-full max-w-5xl xl:max-w-6xl 2xl:max-w-7xl min-h-[75vh] lg:min-h-[82vh] flex flex-col justify-between transition-all duration-200"
>
  <SplashHeader />

  <div class="p-5 md:p-6 lg:p-8 flex-1 grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8 bg-blender-surface">
    <SplashActions />
    <SplashEcosystem />
  </div>

  <SplashFooter bind:showOnStartup />
</div>
