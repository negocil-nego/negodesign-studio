<script setup lang="ts">
import type { MenuTextProps } from "../data/types";
import Icon from "../../../../../../atoms/icon/ui/Icon.vue";
import Text from "../../../../../../atoms/text/ui/Text.vue";
defineProps<MenuTextProps>();
</script>
<template><Icon v-if="icon" v-bind="icon" /><Text v-if="textProps" v-bind="textProps" /></template>
