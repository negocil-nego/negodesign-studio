<script setup lang="ts">
import { computed, ref, watch, useAttrs } from "vue";
import type { TextareaHTMLAttributes } from "vue";
import type { TextareaProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";
import Label from "../../label/ui/label.vue";

type Props = TextareaProps & Omit<TextareaHTMLAttributes, keyof TextareaProps | "value">;
defineOptions({ inheritAttrs: false });
const props = defineProps<Props>();
const attrs = useAttrs();
const content = ref(props.value ?? "");
const textareaAttrs = computed(() => Object.fromEntries(
    [...Object.entries(props), ...Object.entries(attrs)].filter(([key]) => !["id", "name", "value", "placeholder", "rows", "cols", "disabled", "required", "readonly", "maxlength", "labelProps", "class"].includes(key)),
));
watch(() => props.value, (value) => { content.value = value ?? ""; });
</script>

<template>
    <div :class="StyleComponent.textarea.form">
        <Label v-if="props.labelProps" v-bind="props.labelProps" />
        <textarea
            v-bind="textareaAttrs"
            :id="props.id"
            :name="props.name"
            v-model="content"
            :placeholder="props.placeholder"
            :rows="props.rows ?? 4"
            :cols="props.cols"
            :disabled="props.disabled"
            :required="props.required"
            :readonly="props.readonly"
            :maxlength="props.maxlength"
            data-name="textarea"
            :class="[StyleComponent.textarea.default, attrs.class]"
        />
    </div>
</template>