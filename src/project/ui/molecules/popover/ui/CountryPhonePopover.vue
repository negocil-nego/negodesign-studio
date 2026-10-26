<script setup lang="ts">
import { computed, ref, useId } from "vue";
import Popover from "../../../atoms/popover/ui/Popover.vue";
import { getFlagUrl, type FlagNamed } from "../../../../../hooks/flag.svelte";
import { getCountryPhoneOptions, type CountryPhoneOption } from "../data/country-phone";

type Props = {
    flag: FlagNamed;
    locale?: string;
    onselect?: (country: CountryPhoneOption) => void;
};

const props = withDefaults(defineProps<Props>(), { locale: "en" });
const popoverId = useId();
const query = ref("");
const countries = computed(() => getCountryPhoneOptions(props.locale));
const selected = computed(
    () => countries.value.find((country) => country.flag === props.flag) ?? countries.value[0],
);
const filtered = computed(() => {
    const term = query.value.trim().toLocaleLowerCase(props.locale).replace(/^\+/, "");
    if (!term) return countries.value;

    return countries.value.filter(
        (country) =>
            country.name.toLocaleLowerCase(props.locale).includes(term) ||
            country.iso2.toLowerCase().includes(term) ||
            country.dialCode.startsWith(term),
    );
});

function choose(country: CountryPhoneOption) {
    props.onselect?.(country);
    query.value = "";
}
</script>

<template>
    <Popover :id="popoverId" role="listbox" aria-label="Select country calling code">
        <template #trigger>
            <button type="button" :popovertarget="popoverId" aria-haspopup="listbox">
                <img :src="getFlagUrl(selected.flag)" alt="" />
                <span>+{{ selected.dialCode }}</span>
                <span aria-hidden="true">▾</span>
            </button>
        </template>
        <template #panel>
            <input
                v-model="query"
                type="search"
                aria-label="Search country or code"
                placeholder="Search country or code"
            />
            <ul>
                <li v-for="country in filtered" :key="country.iso2">
                    <button
                        type="button"
                        role="option"
                        :aria-selected="country.flag === selected.flag"
                        :popovertarget="popoverId"
                        popovertargetaction="hide"
                        @click="choose(country)"
                    >
                        <img :src="getFlagUrl(country.flag)" alt="" />
                        <span>{{ country.name }}</span>
                        <span>+{{ country.dialCode }}</span>
                    </button>
                </li>
                <li v-if="!filtered.length">No country found</li>
            </ul>
        </template>
    </Popover>
</template>
