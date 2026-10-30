<script setup lang="ts">
import { StyleComponent } from "../../../../../../../../components/local/data/style.component";
import type { MenuTextProps } from "../../menu-text/data/types";
import Icon from "../../../../../../atoms/icon/ui/Icon.vue";
import Text from "../../../../../../atoms/text/ui/Text.vue";
defineProps<MenuTextProps>();
</script>
<template><div :class="StyleComponent.menu.text.root"><Icon v-if="icon" :name="icon.name" :isLoading="icon.isLoading" :className="`${StyleComponent.menu.text.icon} ${icon.className ?? ''}`" /><div v-if="textProps" :class="StyleComponent.menu.text.label"><Text v-bind="textProps" /></div></div></template>
