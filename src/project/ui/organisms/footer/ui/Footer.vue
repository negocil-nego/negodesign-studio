<script setup lang="ts">
import { ref } from "vue";
import Button from "../../../atoms/button/variant/default/ui/Button.vue";
import { StyleComponent } from "../../../../data/style.component";
import type { FooterProps } from "../data/types";
import FooterBottom from "./FooterBottom.vue";
import FooterMain from "./FooterMain.vue";

const props = withDefaults(defineProps<FooterProps>(), {
  variant: "01",
  logoText: "Negodesign Store",
  openingHours: () => [],
  contactItems: () => [],
  columns: () => [],
  socialLinks: () => [],
  paymentBadges: () => [],
  bottomLinks: () => [],
  copyrightText: "© 2026 Negodesign Studio. All rights reserved.",
  className: "",
});

const S = StyleComponent.footer;
const email = ref("");

function submitNewsletter() {
  props.newsletterProps?.onSubmit?.(email.value);
  email.value = "";
}

function onEmailChange(value: string) {
  email.value = value;
}
</script>

<template>
  <footer :class="`${S.root} ${className}`">
    <div v-if="variant === '02' && newsletterProps?.bannerTitle" :class="S.bannerWrap">
      <div :class="S.bannerInner">
        <h3 :class="S.bannerTitle">{{ newsletterProps.bannerTitle }}</h3>
        <form :class="S.bannerForm" @submit.prevent="submitNewsletter">
          <input
            v-model="email"
            type="email"
            :placeholder="newsletterProps.placeholder ?? 'Drop your email here'"
            :class="S.input"
          />
          <Button
            type="submit"
            :text="newsletterProps.buttonProps?.text ?? 'Subscribe'"
            v-bind="newsletterProps.buttonProps"
          />
        </form>
      </div>
    </div>

    <FooterMain
      :variant="variant"
      :logoProps="logoProps"
      :logoText="logoText"
      :logoSubtitle="logoSubtitle"
      :descriptionProps="descriptionProps"
      :openingHours="openingHours"
      :contactItems="contactItems"
      :columns="columns"
      :newsletterProps="newsletterProps"
      :socialLinks="socialLinks"
      :email="email"
      :onEmailChange="onEmailChange"
      :onSubmit="submitNewsletter"
    />

    <FooterBottom :paymentBadges="paymentBadges" :bottomLinks="bottomLinks" :copyrightText="copyrightText" />
  </footer>
</template>
