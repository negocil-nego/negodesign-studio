<script setup lang="ts">
import type { SectionFormProps } from "../data/types";
import Button from "../../../../atoms/button/variant/default/ui/Button.vue";
import Text from "../../../../atoms/text/ui/Text.vue";
import Form from "../../../../molecules/form/ui/form.vue";
withDefaults(defineProps<SectionFormProps>(), { isScreen: false, buttonsLoginProps: () => [] });
</script>
<template><section :class="`w-full ${isScreen ? 'min-h-full flex items-center justify-center' : ''} ${className ?? ''}`"><div class="w-full max-w-md"><div class="flex flex-col items-center gap-5"><div class="flex flex-col items-center gap-2"><Text v-if="titleProps" v-bind="titleProps" /><Text v-if="subtitleProps" v-bind="subtitleProps" /></div><Text v-if="descriptionProps" v-bind="descriptionProps" /></div><div v-if="formProps" class="mt-5"><Form v-bind="formProps" /></div><div v-if="buttonsLoginProps?.length" class="mt-4 flex items-center justify-center gap-3"><Button v-for="(button, i) in buttonsLoginProps" :key="i" v-bind="button" /></div><div v-if="buttonSubmitProps" class="mt-4"><Button v-bind="buttonSubmitProps" /></div></div></section></template>
