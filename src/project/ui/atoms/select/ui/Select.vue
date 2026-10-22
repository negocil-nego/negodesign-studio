<script setup lang="ts">
import { ref, computed, useId } from "vue";
import Icon from "../../icon/ui/Icon.vue";
import type { SelectProps, SelectOption } from "../data/types";

const props = defineProps<SelectProps>();
const emit = defineEmits<{ select: [option: SelectOption] }>();

const popoverId = useId();
const query = ref("");
const selectedValue = ref(props.value ?? "");

const selected = computed(() => props.options.find((opt) => opt.value === selectedValue.value || opt.label === selectedValue.value));
const filtered = computed(() => {
    const term = query.value.trim().toLowerCase();
    return props.options.filter((opt) => !term || opt.label.toLowerCase().includes(term));
});

function choose(option: SelectOption) {
    selectedValue.value = option.value;
    emit("select", option);
    query.value = "";
}
</script>

<template>
    <div class="relative w-full">
        <div class="popover w-full">
            <button type="button" :id="props.id" :disabled="props.disabled" :popovertarget="popoverId"
                aria-haspopup="listbox"
                class="input-form w-full cursor-pointer flex justify-between items-center text-left">
                <div v-if="props.label" class="text-[10px] uppercase font-semibold text-zinc-500 select-none w-full">
                    {{ props.label }}
                </div>
                <div v-else class="text-sm truncate w-full">
                    {{ selected ? selected.label : (props.placeholder ?? "Selecione...") }}
                </div>
                <div>
                    <Icon name="Home" />
                </div>
            </button>

            <div :id="popoverId" popover="auto" role="listbox" :aria-label="props.label ?? 'Select option'"
                class="mt-1.5 p-2 w-full min-w-60 bg-blender-card text-zinc-200 border border-blender-border rounded-lg shadow-xl z-50 outline-none">
                <input v-model="query" type="search" placeholder="Search..."
                    class="w-full px-3 py-1.5 mb-1.5 text-sm bg-blender-surface border border-blender-border rounded text-inherit outline-none focus:border-blue-500" />
                <ul class="max-h-56 overflow-y-auto flex flex-col gap-0.5">
                    <li v-for="option in filtered" :key="option.value">
                        <button type="button" role="option" :disabled="option.disabled" :popovertarget="popoverId"
                            popovertargetaction="hide" @click="choose(option)"
                            class="w-full flex items-center justify-between px-3 py-2 rounded text-sm text-zinc-300 hover:text-slate-900 hover:bg-blender-hover active:bg-blender-active transition-colors cursor-pointer text-left"
                            :class="selected?.value === option.value ? 'bg-blender-hover text-slate-900 font-medium' : ''">
                            <div class="flex items-center gap-2.5 truncate">
                                <span v-if="option.icon" class="text-base">{{ option.icon }}</span>
                                <span class="truncate">{{ option.label }}</span>
                            </div>
                            <span v-if="selected?.value === option.value || selected?.label === option.label"
                                class="text-blue-500 font-bold text-xs">✓</span>
                        </button>
                    </li>
                    <li v-if="!filtered.length" class="px-3 py-2 text-xs text-zinc-500 text-center select-none">
                        No options found
                    </li>
                </ul>
            </div>
        </div>
    </div>
</template>