<script setup lang="ts">
import { ref } from "vue";
import { FlagNamed } from "../../../../../../../../../hooks/flag.svelte";
import type { InputPhoneFlagProps } from "../data/types";
import { getIconClass } from "../../../../../../../../../hooks/icon.svelte";
import type { CountryPhoneOption } from "../../../../../../../molecules/popover/data/country-phone";
import CountryPhonePopover from "../../../../../../../molecules/popover/ui/CountryPhonePopover.vue";
import Label from "../../../../../../label/ui/Label.vue";
import { StyleComponent } from "../../../../../../../../data/style.component";

const props = defineProps<InputPhoneFlagProps>();
const style = StyleComponent.input.phone;
const selectedFlag = ref(props.flag[0] ?? FlagNamed.Ao);

function handleCountrySelect(country: CountryPhoneOption) {
    selectedFlag.value = country.flag;
    props.onCountrySelect?.(country);
}
</script>

<template>
    <div :class="style.field">
        <Label v-if="props.labelProps" v-bind="props.labelProps" />
        <div :class="style.form">
            <CountryPhonePopover :flag="selectedFlag" :locale="props.locale" @select="handleCountrySelect" />
            <i v-if="props.inputProps.icon" :class="getIconClass(props.inputProps.icon)" aria-hidden="true" />
            <input
                :id="props.inputProps.id"
                type="tel"
                :required="props.inputProps.required"
                :disabled="props.inputProps.disabled"
                :placeholder="props.inputProps.placeholder"
                :value="props.inputProps.value"
                data-name="label"
            />
        </div>
    </div>
</template>
