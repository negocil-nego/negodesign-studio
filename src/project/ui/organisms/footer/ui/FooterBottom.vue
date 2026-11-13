<script setup lang="ts">
import { StyleComponent } from "../../../../data/style.component";
import type { FooterBottomProps } from "../data/types";

withDefaults(defineProps<FooterBottomProps>(), {
  paymentBadges: () => [],
  bottomLinks: () => [],
  copyrightText: "© 2026 Negodesign Studio. All rights reserved.",
});

const S = StyleComponent.footer;
</script>

<template>
  <div :class="S.bottomBar">
    <div :class="S.bottomInner">
      <span>{{ copyrightText }}</span>

      <div v-if="bottomLinks.length > 0" :class="S.bottomLinks">
        <a
          v-for="link in bottomLinks"
          :key="link.label"
          :href="link.href ?? '#'"
          :class="S.linkItem"
        >
          {{ link.label }}
        </a>
      </div>

      <div v-if="paymentBadges.length > 0" :class="S.badgesRow">
        <span v-for="badge in paymentBadges" :key="badge" :class="S.badgePill">
          {{ badge }}
        </span>
      </div>
    </div>
  </div>
</template>
