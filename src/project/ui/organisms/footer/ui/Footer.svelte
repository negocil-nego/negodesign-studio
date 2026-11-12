<script lang="ts">
  import Button from "../../../atoms/button/variant/default/ui/Button.svelte";
  import Icon from "../../../atoms/icon/ui/Icon.svelte";
  import Image from "../../../atoms/image/ui/Image.svelte";
  import Text from "../../../atoms/text/ui/Text.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import type { FooterProps } from "../data/types";

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

  <div class={S.mainWrap}>
    <div class={S.brandCol}>
      <div class={S.brandRow}>
        {#if logoProps}
          <Image {...logoProps} className={logoProps.className ?? "size-8 rounded-lg object-cover"} />
        {/if}
        <div>
          <span class={S.brandTitle}>{logoText}</span>
          {#if logoSubtitle}
            <p class={S.brandSubtitle}>{logoSubtitle}</p>
          {/if}
        </div>
      </div>

      {#if descriptionProps}
        <p class={S.description}>
          <Text {...descriptionProps} />
        </p>
      {/if}

      {#if openingHours.length > 0}
        <div class="space-y-1 text-xs text-zinc-400">
          {#each openingHours as line (line)}
            <p>{line}</p>
          {/each}
        </div>
      {/if}

      {#if contactItems.length > 0}
        <ul class={S.contactList}>
          {#each contactItems as item (item.text)}
            <li class={S.contactItem}>
              {#if item.icon}
                <Icon name={item.icon} className="size-4 text-blue-400 shrink-0" />
              {/if}
              <span>{item.text}</span>
            </li>
          {/each}
        </ul>
      {/if}

      {#if socialLinks.length > 0}
        <div class={S.socialRow}>
          {#each socialLinks as social (social.icon)}
            <a
              href={social.href ?? "#"}
              aria-label={social.label ?? social.icon}
              class={S.socialBtn}
            >
              <Icon name={social.icon} className="size-4" />
            </a>
          {/each}
        </div>
      {/if}
    </div>

    {#each columns as col (col.title)}
      <div class={S.linksCol}>
        <h4 class={S.colTitle}>{col.title}</h4>
        <ul class={S.linkList}>
          {#each col.links as link (link.label)}
            <li>
              <a href={link.href ?? "#"} class={S.linkItem}>{link.label}</a>
            </li>
          {/each}
        </ul>
      </div>
    {/each}

    {#if newsletterProps && variant !== "02"}
      <div class={S.newsletterCol}>
        <h4 class={S.colTitle}>{newsletterProps.title ?? "Newsletter"}</h4>
        {#if newsletterProps.description}
          <p class={S.description}>{newsletterProps.description}</p>
        {/if}
        <form class="flex flex-col gap-2.5 sm:flex-row" onsubmit={submitNewsletter}>
          <input
            type="email"
            bind:value={email}
            placeholder={newsletterProps.placeholder ?? "Your email address"}
            class={S.input}
          />
          <button type="submit" class={S.submitBtn}>
            {newsletterProps.buttonProps?.text ?? "Subscribe"}
          </button>
        </form>
      </div>
    {/if}
  </div>

  <div class={S.bottomBar}>
    <div class={S.bottomInner}>
      <span>{copyrightText}</span>

      {#if bottomLinks.length > 0}
        <div class={S.bottomLinks}>
          {#each bottomLinks as link (link.label)}
            <a href={link.href ?? "#"} class={S.linkItem}>{link.label}</a>
          {/each}
        </div>
      {/if}

      {#if paymentBadges.length > 0}
        <div class={S.badgesRow}>
          {#each paymentBadges as badge (badge)}
            <span class={S.badgePill}>{badge}</span>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</footer>
