<script setup lang="ts">
import { ref, computed, useId, useAttrs } from "vue";
import type { ButtonHTMLAttributes } from "vue";
import Icon from "../../icon/ui/Icon.vue";
import Popover from "../../popover/ui/Popover.vue";
import { StyleComponent } from "../../../../data/style.component";
import type { SelectProps, SelectOption } from "../data/types";

type Props = SelectProps & Omit<ButtonHTMLAttributes, keyof SelectProps | "type">;
defineOptions({ inheritAttrs: false });
const props = defineProps<Props>();
const attrs = useAttrs();
const popoverId = useId();
const query = ref("");
const value = ref(props.value ?? "");
const selected = computed(() => props.options.find((opt) => opt.value === value.value || opt.label === value.value));
const filtered = computed(() => {
    const term = query.value.trim().toLowerCase();
    return term ? props.options.filter((opt) => opt.label.toLowerCase().includes(term)) : props.options;
});
const triggerAttrs = computed(() => Object.fromEntries(
    [...Object.entries(props), ...Object.entries(attrs)].filter(([key]) => !["id", "name", "label", "options", "value", "placeholder", "disabled", "required", "onOptionSelect", "onClick", "class"].includes(key)),
));

function choose(option: SelectOption) {
    value.value = option.value;
    props.onOptionSelect?.(option);
    query.value = "";
}
</script>

<template>
    <div :class="StyleComponent.select.root">
        <Popover :id="popoverId" role="listbox" :aria-label="props.label ?? 'Select option'">
            <template #trigger>
                <button
                    v-bind="triggerAttrs"
                    :id="props.id"
                    :disabled="props.disabled"
                    :name="props.name"
                    type="button"
                    :popovertarget="popoverId"
                    aria-haspopup="listbox"
                    @click="props.onClick"
                    :aria-required="props.required"
                    :class="[StyleComponent.select.trigger, attrs.class]"
                >
                    <div v-if="props.label" :class="StyleComponent.select.triggerLabel">{{ props.label }}</div>
                    <div v-else :class="StyleComponent.select.triggerValue">{{ selected ? selected.label : (props.placeholder ?? 'Selecione...') }}</div>
                    <div><Icon name="ArrowDown" /></div>
                </button>
            </template>
            <template #panel>
                <div :class="StyleComponent.select.panel">
                    <input type="search" v-model="query" placeholder="Search..." :class="StyleComponent.select.search" />
                    <ul :class="StyleComponent.select.list">
                        <li v-for="option in filtered" :key="option.value">
                            <button type="button" role="option" :aria-selected="option.value === value || option.label === value" :disabled="option.disabled" :popovertarget="popoverId" popovertargetaction="hide" @click="choose(option)" :class="[StyleComponent.select.option, selected?.value === option.value ? StyleComponent.select.optionSelected : '']">
                                <div :class="StyleComponent.select.optionContent">
                                    <span v-if="option.icon" :class="StyleComponent.select.optionIcon">{{ option.icon }}</span>
                                    <span class="truncate">{{ option.label }}</span>
                                </div>
                                <span v-if="selected?.value === option.value || selected?.label === option.label" :class="StyleComponent.select.optionCheck">✓</span>
                            </button>
                        </li>
                        <li v-if="!filtered.length" :class="StyleComponent.select.optionEmpty">No options found</li>
                    </ul>
                </div>
            </template>
        </Popover>
        <input type="hidden" :name="props.name" :value="selected?.value ?? ''" />
    </div>
</template>