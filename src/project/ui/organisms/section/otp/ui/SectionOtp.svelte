<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import InputOtp from "../../../../atoms/input/children/otp/ui/InputOtp.svelte";
  import Text from "../../../../atoms/text/ui/Text.svelte";
  import type { SectionOtpProps } from "../data/types";

  let {
    isScreen,
    layout = "simple",
    className,
    brandTitleProps,
    brandSubtitleProps,
    brandDescriptionProps,
    titleProps,
    subtitleProps,
    descriptionProps,
    inputOtpProps,
    buttonSubmitProps,
    buttonsLoginProps,
  }: SectionOtpProps = $props();
</script>

<div
  class="w-full {layout === 'dual' ? 'grid grid-cols-1 md:grid-cols-2' : ''}"
  class:h-full={isScreen}
  class:min-h-full={isScreen}
>
  {#if layout === "dual"}
    <section class="flex flex-col justify-between bg-blue-600 px-8 py-10 text-white">
      <span class="flex h-9 w-9 items-center justify-center rounded-lg bg-white/15 font-semibold">N</span>
      <div class="my-8">
        {#if brandSubtitleProps}
          <div class="text-xs font-semibold uppercase tracking-wider text-blue-100">
            <Text {...brandSubtitleProps} />
          </div>
        {/if}
        {#if brandTitleProps}
          <div class="mt-2 text-xl font-semibold">
            <Text {...brandTitleProps} />
          </div>
        {/if}
        {#if brandDescriptionProps}
          <div class="mt-2 text-sm leading-relaxed text-blue-100">
            <Text {...brandDescriptionProps} />
          </div>
        {/if}
      </div>
      <span class="text-[10px] text-blue-100">© 2026 Negodesign</span>
    </section>
  {/if}

  <section
    class="w-full {className}"
    class:h-full={isScreen}
    class:min-h-full={isScreen}
    class:flex={isScreen}
    class:items-center={isScreen}
    class:justify-center={isScreen}
  >
    <div class="w-full max-w-md">
      <div class="flex flex-col gap-5 items-center">
        <div class="flex flex-col gap-2 items-center">
          {#if titleProps}
            <Text {...titleProps} />
          {/if}
          {#if subtitleProps}
            <Text {...subtitleProps} />
          {/if}
        </div>
        {#if descriptionProps}
          <Text {...descriptionProps} />
        {/if}
      </div>
      {#if inputOtpProps}
        <div class="mt-6 flex justify-center">
          <InputOtp {...inputOtpProps} />
        </div>
      {/if}
      {#if buttonSubmitProps}
        <div class="mt-5 flex justify-center">
          <Button {...buttonSubmitProps} />
        </div>
      {/if}
      {#if buttonsLoginProps}
        <div class="mt-4 flex justify-center gap-3">
          {#each buttonsLoginProps as btn, i (i)}
            <Button {...btn} />
          {/each}
        </div>
      {/if}
    </div>
  </section>
</div>
