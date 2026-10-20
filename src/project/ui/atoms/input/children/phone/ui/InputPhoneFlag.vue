<script setup lang="ts">
import { computed, ref } from "vue";
import { getFlagUrl } from "../../../../../../../hooks/flag.svelte";
import Input from "../../../core/ui/Input.vue";
import InputLabel from "../../label/ui/InputLabel.vue";
import CountryPhonePopover from "../../../../../molecules/popover/ui/CountryPhonePopover.vue";
import type { CountryPhoneOption } from "../../../../../molecules/popover/data/country-phone";
import type { InputPhoneFlagProps } from "../data/types";
import type { FlagNamed } from "../../../../../../../hooks/flag.svelte";

const props = defineProps<InputPhoneFlagProps>();
const emit = defineEmits<{ countrySelect: [country: CountryPhoneOption] }>();
const selectedFlag = ref<FlagNamed>(props.flag[0] ?? "ao.svg");
const selectedDialCode = ref("");
const phoneProps = computed(() => ({ ...props.inputProps, type: "tel" as const, value: selectedDialCode.value ? `+${selectedDialCode.value} ${props.inputProps.value ?? ""}` : props.inputProps.value }));
function selectCountry(country: CountryPhoneOption) { selectedFlag.value = country.flag; selectedDialCode.value = country.dialCode; emit("countrySelect", country); props.onCountrySelect?.(country); }
</script>

<template>
  <div class="input-phone-flag">
    <CountryPhonePopover :flag="selectedFlag" :locale="props.locale" @select="selectCountry" />
    <img v-for="selectedFlagItem in props.flag.slice(1)" :key="selectedFlagItem" :src="getFlagUrl(selectedFlagItem)" :alt="selectedFlagItem" />
    <InputLabel v-if="props.labelProps" :labelProps="props.labelProps" :inputProps="phoneProps" />
    <Input v-else v-bind="phoneProps" />
  </div>
</template>

