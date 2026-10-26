<script setup lang="ts">
import { StyleComponent } from "../../../../data/style.component";

defineProps<{ id: string; role?: string; ariaLabel?: string }>();
</script>

<template>
    <div :class="StyleComponent.popover.root">
        <slot name="trigger" />
        <div :id="id" popover="auto" :role="role ?? 'dialog'" :aria-label="ariaLabel">
            <slot name="panel" />
        </div>
    </div>
</template>
