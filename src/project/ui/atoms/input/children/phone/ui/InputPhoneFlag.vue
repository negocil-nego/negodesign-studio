<script setup lang="ts">
import { computed, ref } from "vue";
import type { InputPhoneFlagProps } from "../data/types";
import type { CountryPhoneOption } from "../../../../../molecules/popover/data/country-phone";
import CountryPhonePopover from "../../../../../molecules/popover/ui/CountryPhonePopover.vue";
import Label from "../../../../label/ui/Label.vue";
import type { FlagNamed } from "../../../../../../../hooks/flag.svelte";

const props = defineProps<InputPhoneFlagProps>();
const emit = defineEmits<{ countrySelect: [country: CountryPhoneOption] }>();
const selectedFlag = ref<FlagNamed>(props.flag[0] ?? "ao.svg");
const inputValue = computed(() => props.inputProps.value);

function selectCountry(country: CountryPhoneOption) {
  selectedFlag.value = country.flag;
  emit("countrySelect", country);
  props.onCountrySelect?.(country);
}
</script>

<template>
  <div class="input-phone-field">
    <Label v-if="props.labelProps" v-bind="props.labelProps" />
    <div class="input-form input-phone-flag">
      <CountryPhonePopover :flag="selectedFlag" :locale="props.locale" @select="selectCountry" />
      <img v-if="props.inputProps.icon" :src="`/icons/${props.inputProps.icon}`" alt="" />
      <input
        :id="props.inputProps.id"
        type="tel"
        :required="props.inputProps.required"
        :disabled="props.inputProps.disabled"
        :placeholder="props.inputProps.placeholder"
        :value="inputValue"
        data-name="label"
      />
    </div>
  </div>
</template>


