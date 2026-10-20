<script setup lang="ts">
import { computed, ref, useId } from "vue";
import { getFlagUrl, type FlagNamed } from "../../../../../hooks/flag.svelte";
import { getCountryPhoneOptions, type CountryPhoneOption } from "../data/country-phone";

const props = withDefaults(defineProps<{ flag: FlagNamed; locale?: string }>(), { locale: "en" });
const emit = defineEmits<{ select: [country: CountryPhoneOption] }>();
const popoverId = useId();
const query = ref("");
const countries = computed(() => getCountryPhoneOptions(props.locale));
const selected = computed(() => countries.value.find((country) => country.flag === props.flag) ?? countries.value[0]);
const filtered = computed(() => {
  const term = query.value.trim().toLocaleLowerCase(props.locale).replace(/^\+/, "");
  return countries.value.filter((country) => !term || country.name.toLocaleLowerCase(props.locale).includes(term) || country.iso2.toLowerCase().includes(term) || country.dialCode.startsWith(term));
});

</script>

<template>
  <div class="popover">
    <button type="button" :popovertarget="popoverId" aria-haspopup="listbox"><img :src="getFlagUrl(selected.flag)" alt="" /><span>+{{ selected.dialCode }}</span><span aria-hidden="true">▾</span></button>
    <div :id="popoverId" popover="auto" role="listbox" aria-label="Select country calling code">
      <input v-model="query" autofocus type="search" placeholder="Search country or code" />
      <ul><li v-for="country in filtered" :key="country.iso2"><button type="button" role="option" :popovertarget="popoverId" popovertargetaction="hide" @click="emit('select', country); query = ''"><img :src="getFlagUrl(country.flag)" alt="" /><span>{{ country.name }}</span><span>+{{ country.dialCode }}</span></button></li><li v-if="!filtered.length">No country found</li></ul>
    </div>
  </div>
</template>
