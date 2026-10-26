// @ts-nocheck
import { useId, useMemo, useState } from "react";
import Popover from "../../../atoms/popover/ui/Popover";
import { getFlagUrl } from "../../../../../hooks/flag.svelte";
import { getCountryPhoneOptions } from "../data/country-phone";

export default function CountryPhonePopover({ flag, locale = "en", onselect }) {
    const popoverId = useId();
    const [query, setQuery] = useState("");
    const countries = useMemo(() => getCountryPhoneOptions(locale), [locale]);
    const selected = countries.find((country) => country.flag === flag) ?? countries[0];
    const term = query.trim().toLocaleLowerCase(locale).replace(/^\+/, "");
    const filtered = term
        ? countries.filter(
            (country) =>
                country.name.toLocaleLowerCase(locale).includes(term) ||
                country.iso2.toLowerCase().includes(term) ||
                country.dialCode.startsWith(term),
        )
        : countries;

    function choose(country) {
        onselect?.(country);
        setQuery("");
    }

    return (
        <Popover
            id={popoverId}
            role="listbox"
            ariaLabel="Select country calling code"
            trigger={
                <button type="button" popoverTarget={popoverId} aria-haspopup="listbox">
                    <img src={getFlagUrl(selected.flag)} alt="" />
                    <span>+{selected.dialCode}</span>
                    <span aria-hidden="true">▾</span>
                </button>
            }
            panel={
                <>
                    <input
                        type="search"
                        value={query}
                        onChange={(event) => setQuery(event.target.value)}
                        aria-label="Search country or code"
                        placeholder="Search country or code"
                    />
                    <ul>
                        {filtered.map((country) => (
                            <li key={country.iso2}>
                                <button
                                    type="button"
                                    role="option"
                                    aria-selected={country.flag === selected.flag}
                                    popoverTarget={popoverId}
                                    popoverTargetAction="hide"
                                    onClick={() => choose(country)}
                                >
                                    <img src={getFlagUrl(country.flag)} alt="" />
                                    <span>{country.name}</span>
                                    <span>+{country.dialCode}</span>
                                </button>
                            </li>
                        ))}
                        {!filtered.length && <li>No country found</li>}
                    </ul>
                </>
            }
        />
    );
}
