<script setup lang="ts">
import { ref, watch } from "vue";
import type { TextareaProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";
import Label from "../../label/ui/label.svelte";

const { id, name, value, placeholder, rows = 4, cols, disabled, required, readonly, maxlength, labelProps } = defineProps<TextareaProps>();
const content = ref(value ?? "");
watch(() => value, (v) => { content.value = v ?? ""; });
</script>

<template>
    <div :class="StyleComponent.textarea.form">
        <Label v-if="labelProps" {...labelProps} />
        <textarea
            :id="id"
            :name="name"
            v-model="content"
            :placeholder="placeholder"
            :rows="rows"
            :cols="cols"
            :disabled="disabled"
            :required="required"
            :readonly="readonly"
            :maxlength="maxlength"
            data-name="textarea"
            :class="StyleComponent.textarea.default"
        />
    </div>
</template>
