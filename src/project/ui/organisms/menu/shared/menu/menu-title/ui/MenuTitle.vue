<script setup lang="ts">
import { computed } from "vue";
import type { MenuTitleProps } from "../data/types";
import { StyleComponent } from "../../../../../../../../components/local/data/style.component";
import Text from "../../../../../../atoms/text/ui/Text.vue";
const props = withDefaults(defineProps<MenuTitleProps>(), { variant: "item" });
const classes = computed(() => props.variant === "featured" ? StyleComponent.menu.title.featured : StyleComponent.menu.title.item);
</script>
<template><div :class="classes.root"><template v-if="title"><a v-if="href" :class="classes.titleLink" :href="href"><Text v-bind="title" /></a><div v-else :class="classes.title"><Text v-bind="title" /></div></template><div v-if="description" :class="classes.description"><Text v-bind="description" /></div><template v-if="link"><a v-if="variant === 'featured' && href" :class="classes.link" :href="href"><Text v-bind="link" /><span aria-hidden="true">→</span></a><div v-else :class="classes.link"><Text v-bind="link" /></div></template></div></template>
