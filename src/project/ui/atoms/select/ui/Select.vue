<script setup lang="ts">
import { ref, computed, useId } from "vue";
import Icon from "../../icon/ui/Icon.vue";
import Popover from "../../popover/ui/Popover.vue";
import { StyleComponent } from "../../../../data/style.component";
import type { SelectProps, SelectOption } from "../data/types";

const props = defineProps<SelectProps>();

const popoverId = useId();
const query = ref("");
const value = ref(props.value ?? "");

const selected = computed(() => props.options.find((opt) => opt.value === value.value || opt.label === value.value));
const filtered = computed(() => {
    const term = query.value.trim().toLowerCase();
    if (!term) return props.options;
    return props.options.filter((opt) => opt.label.toLowerCase().includes(term));
});

function choose(option: SelectOption) {
    value.value = option.label;
    props.onselect?.(option);
    query.value = "";
}
</script>

<template>
    <div :class="StyleComponent.select.root">
        <Popover :id="popoverId" role="listbox" :aria-label="props.label ?? 'Select option'">
            <template #trigger>
                <button
                    :id="props.id"
                    :disabled="props.disabled"
                    type="button"
                    :popovertarget="popoverId"
                    aria-haspopup="listbox"
                    :class="StyleComponent.select.trigger"
                >
                    <div v-if="props.label" :class="StyleComponent.select.triggerLabel">
                        {{ props.label }}
                    </div>
                    <div v-else :class="StyleComponent.select.triggerValue">
                        {{ selected ? selected.label : (props.placeholder ?? "Selecione...") }}
                    </div>
                    <div>
                        <Icon name="ArrowDown" />
                    </div>
                </button>
            </template>
            <template #panel>
                <div :class="StyleComponent.select.panel">
                    <input
                        type="search"
                        v-model="query"
                        placeholder="Search..."
                        :class="StyleComponent.select.search"
                    />
                    <ul :class="StyleComponent.select.list">
                        <li v-for="option in filtered" :key="option.value">
                            <button
                                type="button"
                                role="option"
                                :aria-selected="option.value === value || option.label === value"
                                :disabled="option.disabled"
                                :popovertarget="popoverId"
                                popovertargetaction="hide"
                                @click="choose(option)"
                                :class="[StyleComponent.select.option, selected?.value === option.value ? StyleComponent.select.optionSelected : '']"
                            >
                                <div :class="StyleComponent.select.optionContent">
                                    <span v-if="option.icon" :class="StyleComponent.select.optionIcon">{{ option.icon }}</span>
                                    <span class="truncate">{{ option.label }}</span>
                                </div>
                                <span v-if="selected?.value === option.value || selected?.label === option.label" :class="StyleComponent.select.optionCheck">✓</span>
                            </button>
                        </li>
                        <li v-if="!filtered.length" :class="StyleComponent.select.optionEmpty">
                            No options found
                        </li>
                    </ul>
                </div>
            </template>
        </Popover>
    </div>
</template>
