<script setup lang="ts">
import Icon from "../../../../icon/ui/Icon.vue";
import type { ButtonBaseProps } from "../data/types";

withDefaults(defineProps<ButtonBaseProps>(), { isDisabled: false });
</script>

<template>
    <button :class="className" @click="onClick" :disabled="isDisabled || isLoading">
        <Icon v-if="icon" v-bind="icon" />
        <span v-if="text">{{ text }}</span>
    </button>
</template>
