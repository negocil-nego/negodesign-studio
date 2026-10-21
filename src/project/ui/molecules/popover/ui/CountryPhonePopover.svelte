<script lang="ts">
  import { getFlagUrl, type FlagNamed } from "../../../../../hooks/flag.svelte";
  import Popover from "../../../atoms/popover/ui/Popover.svelte";
  import {
    getCountryPhoneOptions,
    type CountryPhoneOption,
  } from "../data/country-phone";

  type Props = {
    flag: FlagNamed;
    locale?: string;
    onselect?: (country: CountryPhoneOption) => void;
  };

  let { flag, locale = "en", onselect }: Props = $props();
  const popoverId = $props.id();
  let query = $state("");
  const countries = $derived(getCountryPhoneOptions(locale));
  const selected = $derived(
    countries.find((country) => country.flag === flag) ?? countries[0],
  );

  const filtered = $derived.by(() => {
    const term = query.trim().toLocaleLowerCase(locale).replace(/^\+/, "");
    if (!term) return countries;

    return countries.filter(
      (country) =>
        country.name.toLocaleLowerCase(locale).includes(term) ||
        country.iso2.toLowerCase().includes(term) ||
        country.dialCode.startsWith(term),
    );
  });

  function choose(country: CountryPhoneOption) {
    onselect?.(country);
    query = "";
  }
</script>

<Popover id={popoverId} role="listbox" ariaLabel="Select country calling code">
  {#snippet trigger()}
    <button type="button" popovertarget={popoverId} aria-haspopup="listbox">
      <img src={getFlagUrl(selected.flag)} alt="" />
      <span>+{selected.dialCode}</span>
      <span aria-hidden="true">▾</span>
    </button>
  {/snippet}

  {#snippet panel()}
    <input
      type="search"
      bind:value={query}
      aria-label="Search country or code"
      placeholder="Search country or code"
    />
    <ul>
      {#each filtered as country (country.iso2)}
        <li>
          <button
            type="button"
            role="option"
            aria-selected={country.flag === selected.flag}
            popovertarget={popoverId}
            popovertargetaction="hide"
            onclick={() => choose(country)}
          >
            <img src={getFlagUrl(country.flag)} alt="" />
            <span>{country.name}</span>
            <span>+{country.dialCode}</span>
          </button>
        </li>
      {:else}
        <li>No country found</li>
      {/each}
    </ul>
  {/snippet}
</Popover>

