<script lang="ts">
  import CardProfile from "../../../../../../project/ui/molecules/card/profile/ui/CardProfile.svelte";
  import svelteSource from "../../../../../../project/ui/molecules/card/profile/ui/CardProfile.svelte?raw";
  import reactSource from "../../../../../../project/ui/molecules/card/profile/ui/CardProfile.tsx?raw";
  import vueSource from "../../../../../../project/ui/molecules/card/profile/ui/CardProfile.vue?raw";
  import flutterSource from "../../../../../../project/ui/molecules/card/profile/ui/CardProfile.dart?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";

  import type { CardTag } from "../../../../../../project/ui/molecules/card/shared/card-tags/data/types";
  import type { CardActionButton } from "../../../../../../project/ui/molecules/card/shared/card-actions-button/data/types";

  const cardTags: CardTag[] = [
    { icon: "Star", title: "Novo" },
    { icon: "Location", title: "Luanda" },
  ];

  const cardActions: CardActionButton[] = [
    { icon: "Link", label: "Partilhar" },
    { icon: "MoreHorizontal", label: "Mais" },
  ];
</script>

<Terminal id="card-profile">
  {#snippet preview()}
    <div class="w-full max-w-xs">
      <CardProfile
        variant="01"
        logoProps={{ src: "/img/programmer-01.jpg", alt: "Perfil" }}
        imageProps={{ src: "/img/programmer-02.jpg", alt: "Capa" }}
        titleProps={{ title: "Ana Silva" }}
        tagsProps={{ tags: cardTags }}
        actionsProps={{ actions: cardActions }}
        descriptionProps={{ description: "Designer de produto em Luanda." }}
        buttonProps={{ text: "Seguir" }}
        cartButtonProps={{}}
        favoriteButtonProps={{}}
      />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/project/ui/molecules/card/profile/ui/CardProfile.svelte"
      source={svelteSource}
    />
  {/snippet}

  {#snippet react()}
    <SourceCode
      path="src/project/ui/molecules/card/profile/ui/CardProfile.tsx"
      source={reactSource}
    />
  {/snippet}

  {#snippet vue()}
    <SourceCode
      path="src/project/ui/molecules/card/profile/ui/CardProfile.vue"
      source={vueSource}
    />
  {/snippet}

  {#snippet flutter()}
    <SourceCode
      path="src/project/ui/molecules/card/profile/ui/CardProfile.dart"
      source={flutterSource}
    />
  {/snippet}
</Terminal>
