// @ts-nocheck
import { useId, useState } from "react";
import { getFlagUrl } from "../../../../../hooks/flag.svelte";
import { getCountryPhoneOptions } from "../data/country-phone";

export default function CountryPhonePopover({ flag, locale = "en", onselect }) {
    const [query, setQuery] = useState("");
    const popoverId = useId();
    const countries = getCountryPhoneOptions(locale);
    const selected = countries.find((country) => country.flag === flag) ?? countries[0];
    const term = query.trim().toLocaleLowerCase(locale).replace(/^\+/, "");
    const filtered = countries.filter((country) => !term || country.name.toLocaleLowerCase(locale).includes(term) || country.iso2.toLowerCase().includes(term) || country.dialCode.startsWith(term));
    return (
        <div className="popover">
            <button type="button" popoverTarget={popoverId} aria-haspopup="listbox">
                <img src={getFlagUrl(selected.flag)} alt="" /> +{selected.dialCode} <span aria-hidden="true">▾</span>
            </button>
            <div id={popoverId} popover="auto" role="listbox" aria-label="Select country calling code">
                <input autoFocus type="search" value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Search country or code" />
                <ul>{filtered.map((country) => <li key={country.iso2}><button type="button" role="option" popoverTarget={popoverId} popoverTargetAction="hide" onClick={() => { onselect?.(country); setQuery(""); }}><img src={getFlagUrl(country.flag)} alt="" /><span>{country.name}</span><span>+{country.dialCode}</span></button></li>)}{!filtered.length && <li>No country found</li>}</ul>
            </div>
        </div>
    );
}
