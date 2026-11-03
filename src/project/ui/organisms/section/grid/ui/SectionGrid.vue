<script setup lang="ts">
import type { SectionGridProps } from "../data/types";
const props = withDefaults(defineProps<Omit<SectionGridProps, "children">>(), { className: "" });
</script>
<template>
    <section :class="`w-full min-w-0 py-6 ${props.className}`.trim()">
        <header v-if="props.titleProps || props.descriptionProps" class="mb-6">
            <h2 v-if="props.titleProps" class="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl">{{
                props.titleProps.text }}</h2>
            <p v-if="props.descriptionProps" class="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">
                {{ props.descriptionProps.text }}</p>
        </header>
        <div
            class="grid grid-cols-1 gap-2 sm:grid-cols-2 md:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 [&>*]:w-full [&>*]:min-w-0"
        >
            <slot />
        </div>
    </section>
</template>
