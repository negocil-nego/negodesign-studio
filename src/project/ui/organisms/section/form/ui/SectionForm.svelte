<script lang="ts">
  import Button from "../../../../atoms/button/variant/default/ui/Button.svelte";
  import Text from "../../../../atoms/text/ui/Text.svelte";
  import Form from "../../../../molecules/form/ui/form.svelte";
  import type { SectionFormProps } from "../data/types";

  let {
    isScreen,
    className,
    titleProps,
    subtitleProps,
    descriptionProps,
    formProps,
    buttonSubmitProps,
    buttonsLoginProps,
  }: SectionFormProps = $props();
</script>

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
    {#if formProps}
      <div>
        <Form {...formProps} />
      </div>
    {/if}
    {#if buttonsLoginProps}
      <div class="flex justify-center items-center gap-3 mt-2 md:mt-5">
        {#each buttonsLoginProps as btn, i (i)}
          <Button {...btn} />
        {/each}
      </div>
    {/if}
    {#if buttonSubmitProps}
      <div class="mt-2 md:mt-5">
        <Button {...buttonSubmitProps} />
      </div>
    {/if}
  </div>
</section>
