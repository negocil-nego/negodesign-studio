<script setup lang="ts">
import Menu from "../../../../organisms/menu/simple/ui/Menu.vue";
import Button from "../../../../atoms/button/variant/default/ui/Button.vue";
import Text from "../../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../../../../components/local/data/style.component";
import type { PageHeroSimpleProps } from "../data/types";
const props = defineProps<PageHeroSimpleProps>();
const hero = props.sectionHeroProps ?? {};
const light = hero.style === "light";
const styles = light ? StyleComponent.pages.hero.light : StyleComponent.pages.hero.simple;
</script>
<template><header :class="styles.root" :style="{ backgroundColor: light ? '#eef5f6' : hero.backgroundColor ?? '#10212b' }">
  <div v-if="light" :class="StyleComponent.pages.hero.light.backdrop" aria-hidden="true" />
  <div v-else-if="hero.backgroundMediaUrl" :class="StyleComponent.pages.hero.simple.background" :style="{ backgroundImage: `url('${hero.backgroundMediaUrl}')` }" aria-hidden="true" />
  <template v-if="!light"><div :class="StyleComponent.pages.hero.simple.overlay" aria-hidden="true" /><div :class="StyleComponent.pages.hero.simple.shade" :style="{ opacity: Math.min(1, Math.max(0, hero.backgroundOverlayOpacity ?? 0.55)) }" aria-hidden="true" /></template>
  <aside v-if="menuProps" :class="styles.navigation"><Menu v-bind="menuProps" :style="light ? 'ghost' : 'ghost-inverse'" /></aside>
  <section v-if="hero.textSectionProps" :class="light ? StyleComponent.pages.hero.light.content : StyleComponent.pages.hero.simple.content">
    <div v-if="hero.textSectionProps.titleProps" :class="light ? StyleComponent.pages.hero.light.eyebrow : StyleComponent.pages.hero.simple.eyebrow"><Text v-bind="hero.textSectionProps.titleProps" :className="light ? StyleComponent.pages.hero.light.eyebrowText : StyleComponent.pages.hero.simple.eyebrowText" /></div>
    <div v-if="hero.textSectionProps.subtitleProps" role="heading" aria-level="1"><Text v-bind="hero.textSectionProps.subtitleProps" :className="light ? StyleComponent.pages.hero.light.titleText : StyleComponent.pages.hero.simple.titleText" /></div>
    <Text v-if="hero.textSectionProps.descriptionProps" v-bind="hero.textSectionProps.descriptionProps" :className="light ? StyleComponent.pages.hero.light.descriptionText : StyleComponent.pages.hero.simple.descriptionText" />
    <div v-if="hero.textSectionProps.buttonsProps?.length" class="relative z-20 mt-6 flex flex-wrap justify-center gap-3"><Button v-for="(button, index) in hero.textSectionProps.buttonsProps" :key="index" v-bind="button" :className="button.className ?? (light ? StyleComponent.pages.hero.light.action : undefined)" /></div>
    <img v-if="hero.textSectionProps.imageUrl" :src="hero.textSectionProps.imageUrl" :alt="hero.textSectionProps.imageAlt ?? ''" :class="light ? StyleComponent.pages.hero.light.illustration : StyleComponent.pages.hero.dual.media" />
  </section>
</header></template>
