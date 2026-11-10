<script setup lang="ts">
import Menu from "../../../../organisms/menu/simple/ui/Menu.vue";
import Button from "../../../../atoms/button/variant/default/ui/Button.vue";
import Text from "../../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../../../data/style.component";
import type { PageHeroDualProps } from "../data/types";

defineProps<PageHeroDualProps>();
const s = StyleComponent.pages.hero.dual;
</script>

<template>
  <header :class="s.root">
    <div :class="s.backdrop" aria-hidden="true" />
    <aside v-if="menuProps" :class="s.navigation">
      <Menu v-bind="menuProps" :style="menuProps.style ?? 'ghost'" />
    </aside>
    <div :class="s.layout">
      <template v-for="(section, index) in [leftSectionHereProps, rightSectionHereProps]" :key="index">
        <div v-if="section" :class="s.section">
          <section v-if="section.textSectionProps" :class="s.content">
            <div v-if="section.textSectionProps.titleProps" :class="s.eyebrow">
              <Text v-bind="section.textSectionProps.titleProps" :className="s.eyebrowText" />
            </div>
            <div v-if="section.textSectionProps.subtitleProps" role="heading" aria-level="1">
              <Text v-bind="section.textSectionProps.subtitleProps" :className="s.titleText" />
            </div>
            <Text
              v-if="section.textSectionProps.descriptionProps"
              v-bind="section.textSectionProps.descriptionProps"
              :className="s.descriptionText"
            />
            <div v-if="section.textSectionProps.buttonsProps?.length" class="relative z-20 mt-6 flex flex-wrap gap-3">
              <Button v-for="(b, i) in section.textSectionProps.buttonsProps" :key="i" v-bind="b" />
            </div>
            <img
              v-if="section.textSectionProps.imageUrl"
              :src="section.textSectionProps.imageUrl"
              :alt="section.textSectionProps.imageAlt ?? ''"
              :class="s.media"
            />
          </section>
        </div>
      </template>
    </div>
  </header>
</template>
