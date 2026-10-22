<script setup lang="ts">
import { ref, watch } from "vue";
import type { TextareaProps } from "../data/types";

const { id, name, value, placeholder, rows = 4, cols, disabled, required, readonly, maxlength } = defineProps<TextareaProps>();
const content = ref(value ?? "");
watch(() => value, (v) => { content.value = v ?? ""; });
</script>

<template>
    <div class="input-form">
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
        />
    </div>
</template>
