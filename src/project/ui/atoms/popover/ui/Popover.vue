<script setup lang="ts">
defineProps<{ id: string; role?: string; ariaLabel?: string }>();
</script>

<template>
    <div class="popover w-full">
        <slot name="trigger" />
        <div :id="id" popover="auto" :role="role ?? 'dialog'" :aria-label="ariaLabel">
            <slot name="panel" />
        </div>
    </div>
</template>
