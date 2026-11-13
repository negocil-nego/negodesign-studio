<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { FooterBottomProps } from "../data/types";

  let {
    paymentBadges = [],
    bottomLinks = [],
    copyrightText = "© 2026 Negodesign Studio. All rights reserved.",
  }: FooterBottomProps = $props();

  const S = StyleComponent.footer;
</script>

<div class={S.bottomBar}>
  <div class={S.bottomInner}>
    <span>{copyrightText}</span>

    {#if bottomLinks.length > 0}
      <div class={S.bottomLinks}>
        {#each bottomLinks as link (link.label)}
          <a href={link.href ?? "#"} class={S.linkItem}>{link.label}</a>
        {/each}
      </div>
    {/if}

    {#if paymentBadges.length > 0}
      <div class={S.badgesRow}>
        {#each paymentBadges as badge (badge)}
          <span class={S.badgePill}>{badge}</span>
        {/each}
      </div>
    {/if}
  </div>
</div>
