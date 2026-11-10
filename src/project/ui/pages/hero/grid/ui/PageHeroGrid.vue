<script setup lang="ts">
import Menu from "../../../../organisms/menu/simple/ui/Menu.vue";
import Button from "../../../../atoms/button/variant/default/ui/Button.vue";
import Text from "../../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../../../data/style.component";
import {
  HERO_GRID_COLS,
  HERO_GRID_POS,
  HERO_GRID_ROWS,
  type PageHeroGridCardProps,
  type PageHeroGridProps,
} from "../data/types";

withDefaults(defineProps<PageHeroGridProps>(), {
  backgroundColor: "#17161c",
  cards: () => [],
});

const g = StyleComponent.pages.hero.grid;
const getCardTone = (card: PageHeroGridCardProps) =>
  card.sectionCardProps?.tone === "light"
    ? StyleComponent.pages.hero.gridLight
    : StyleComponent.pages.hero.grid;
</script>

<template>
  <section :class="g.root" :style="{ backgroundColor }" aria-label="Hero em mosaico">
    <aside v-if="menuProps" :class="g.navigation">
      <Menu v-bind="menuProps" :style="menuProps.style ?? 'ghost-inverse'" />
    </aside>
    <div :class="g.tiles">
      <article
        v-for="card in cards"
        :key="card.id"
        :class="`${g.card} ${HERO_GRID_COLS[card.colSpan ?? 12]} ${HERO_GRID_ROWS[card.rowSpan ?? 2]} ${card.className ?? ''}`"
        :style="{ backgroundColor: card.backgroundColor ?? '#28262e' }"
        :aria-label="card.sectionCardProps?.subtitleProps?.text ?? card.id"
      >
        <img
          v-if="card.imageUrl"
          :src="card.imageUrl"
          :alt="card.imageAlt ?? ''"
          loading="lazy"
          :class="`${card.imageMode === 'contain' ? g.imageContain : g.imageCover} ${card.imageClassName ?? ''}`"
        />
        <div v-if="card.showOverlay" :class="g.overlay" aria-hidden="true" />
        <section
          v-if="card.sectionCardProps"
          :class="`${getCardTone(card).content} ${HERO_GRID_POS[card.sectionCardProps.contentPosition ?? 'end']}`"
        >
          <div v-if="card.sectionCardProps.titleProps" :class="getCardTone(card).eyebrow">
            <Text v-bind="card.sectionCardProps.titleProps" :className="getCardTone(card).eyebrowText" />
          </div>
          <div v-if="card.sectionCardProps.subtitleProps" role="heading" aria-level="2">
            <Text v-bind="card.sectionCardProps.subtitleProps" :className="getCardTone(card).titleText" />
          </div>
          <Text
            v-if="card.sectionCardProps.descriptionProps"
            v-bind="card.sectionCardProps.descriptionProps"
            :className="getCardTone(card).descriptionText"
          />
          <div
            v-if="card.sectionCardProps.buttonsProps?.length"
            class="relative z-20 mt-4 flex flex-wrap justify-start gap-3"
          >
            <Button
              v-for="(b, i) in card.sectionCardProps.buttonsProps"
              :key="i"
              v-bind="b"
              :className="b.className ?? getCardTone(card).action"
            />
          </div>
        </section>
      </article>
    </div>
  </section>
</template>
