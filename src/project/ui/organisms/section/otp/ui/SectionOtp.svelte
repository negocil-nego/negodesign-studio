<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import InputOtp from "../../../../atoms/input/children/otp/ui/InputOtp.svelte";
  import Text from "../../../../atoms/text/ui/Text.svelte";
  import type { SectionOtpProps } from "../data/types";

  let {
    isScreen,
    className,
    titleProps,
    subtitleProps,
    descriptionProps,
    inputOtpProps,
    buttonSubmitProps,
  }: SectionOtpProps = $props();
</script>

<section class="w-full {className}" class:h-screen={isScreen}>
  <div>
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
      <div>
        <InputOtp {...inputOtpProps} />
      </div>
    {/if}
    {#if buttonSubmitProps}
      <div>
        <Button {...buttonSubmitProps} />
      </div>
    {/if}
  </div>
</section>
