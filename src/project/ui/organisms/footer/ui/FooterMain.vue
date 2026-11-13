<script setup lang="ts">
import Icon from "../../../atoms/icon/ui/Icon.vue";
import Image from "../../../atoms/image/ui/Image.vue";
import Text from "../../../atoms/text/ui/Text.vue";
import { StyleComponent } from "../../../../data/style.component";
import type { FooterMainProps } from "../data/types";

withDefaults(defineProps<FooterMainProps>(), {
  variant: "01",
  logoText: "Negodesign Store",
  openingHours: () => [],
  contactItems: () => [],
  columns: () => [],
  socialLinks: () => [],
  email: "",
});

const S = StyleComponent.footer;
</script>

<template>
  <div :class="S.mainWrap">
    <div :class="S.brandCol">
      <div :class="S.brandRow">
        <Image
          v-if="logoProps"
          v-bind="logoProps"
          :className="logoProps.className ?? 'size-8 rounded-lg object-cover'"
        />
        <div>
          <span :class="S.brandTitle">{{ logoText }}</span>
          <p v-if="logoSubtitle" :class="S.brandSubtitle">{{ logoSubtitle }}</p>
        </div>
      </div>

      <p v-if="descriptionProps" :class="S.description">
        <Text v-bind="descriptionProps" />
      </p>

      <div v-if="openingHours.length > 0" class="space-y-1 text-xs text-zinc-400">
        <p v-for="line in openingHours" :key="line">{{ line }}</p>
      </div>

      <ul v-if="contactItems.length > 0" :class="S.contactList">
        <li v-for="item in contactItems" :key="item.text" :class="S.contactItem">
          <Icon v-if="item.icon" :name="item.icon" className="size-4 text-blue-400 shrink-0" />
          <span>{{ item.text }}</span>
        </li>
      </ul>

      <div v-if="socialLinks.length > 0" :class="S.socialRow">
        <a
          v-for="social in socialLinks"
          :key="social.icon"
          :href="social.href ?? '#'"
          :aria-label="social.label ?? social.icon"
          :class="S.socialBtn"
        >
          <Icon :name="social.icon" className="size-4" />
        </a>
      </div>
    </div>

    <div v-for="col in columns" :key="col.title" :class="S.linksCol">
      <h4 :class="S.colTitle">{{ col.title }}</h4>
      <ul :class="S.linkList">
        <li v-for="link in col.links" :key="link.label">
          <a :href="link.href ?? '#'" :class="S.linkItem">{{ link.label }}</a>
        </li>
      </ul>
    </div>

    <div v-if="newsletterProps && variant !== '02'" :class="S.newsletterCol">
      <h4 :class="S.colTitle">{{ newsletterProps.title ?? "Newsletter" }}</h4>
      <p v-if="newsletterProps.description" :class="S.description">
        {{ newsletterProps.description }}
      </p>
      <form class="flex flex-col gap-2.5 sm:flex-row" @submit.prevent="onSubmit?.($event)">
        <input
          :value="email"
          type="email"
          :placeholder="newsletterProps.placeholder ?? 'Your email address'"
          :class="S.input"
          @input="onEmailChange?.(($event.target as HTMLInputElement).value)"
        />
        <button type="submit" :class="S.submitBtn">
          {{ newsletterProps.buttonProps?.text ?? "Subscribe" }}
        </button>
      </form>
    </div>
  </div>
</template>
