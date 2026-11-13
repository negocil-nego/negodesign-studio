<script lang="ts">
  import Button from "../../../atoms/button/variant/default/ui/Button.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import type { FooterProps } from "../data/types";
  import FooterBottom from "./FooterBottom.svelte";
  import FooterMain from "./FooterMain.svelte";

  let {
    variant = "01",
    logoProps,
    logoText = "Negodesign Store",
    logoSubtitle,
    descriptionProps,
    openingHours = [],
    contactItems = [],
    columns = [],
    newsletterProps,
    socialLinks = [],
    paymentBadges = [],
    bottomLinks = [],
    copyrightText = "© 2026 Negodesign Studio. All rights reserved.",
    className = "",
  }: FooterProps = $props();

  const S = StyleComponent.footer;
  let email = $state("");

  function submitNewsletter(event: SubmitEvent) {
    event.preventDefault();
    newsletterProps?.onSubmit?.(email);
    email = "";
  }
</script>

<footer class={`${S.root} ${className}`}>
  {#if variant === "02" && newsletterProps?.bannerTitle}
    <div class={S.bannerWrap}>
      <div class={S.bannerInner}>
        <h3 class={S.bannerTitle}>{newsletterProps.bannerTitle}</h3>
        <form class={S.bannerForm} onsubmit={submitNewsletter}>
          <input
            type="email"
            bind:value={email}
            placeholder={newsletterProps.placeholder ?? "Drop your email here"}
            class={S.input}
          />
          <Button
            text={newsletterProps.buttonProps?.text ?? "Subscribe"}
            {...newsletterProps.buttonProps}
          />
        </form>
      </div>
    </div>
  {/if}

  <FooterMain
    {variant}
    {logoProps}
    {logoText}
    {logoSubtitle}
    {descriptionProps}
    {openingHours}
    {contactItems}
    {columns}
    {newsletterProps}
    {socialLinks}
    bind:email
    onSubmit={submitNewsletter}
  />

  <FooterBottom {paymentBadges} {bottomLinks} {copyrightText} />
</footer>
