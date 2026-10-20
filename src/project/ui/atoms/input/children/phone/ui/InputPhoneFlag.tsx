// @ts-nocheck
import { useState } from "react";
import { getFlagUrl } from "../../../../../../../hooks/flag.svelte";
import Input from "../../../core/ui/Input";
import InputLabel from "../../label/ui/InputLabel";
import CountryPhonePopover from "../../../../../molecules/popover/ui/CountryPhonePopover";
import type { InputPhoneFlagProps } from "../data/types";

export default function InputPhoneFlag({ labelProps, inputProps, flag, locale, onCountrySelect }: InputPhoneFlagProps) {
    const phoneProps = { ...inputProps, type: "tel" };
    const [selectedFlag, setSelectedFlag] = useState(flag[0] ?? "ao.svg");
    const [selectedDialCode, setSelectedDialCode] = useState("");

    function selectCountry(country) {
        setSelectedFlag(country.flag);
        setSelectedDialCode(country.dialCode);
        onCountrySelect?.(country);
    }

    return (
        <div className="input-phone-flag">
            <CountryPhonePopover flag={selectedFlag} locale={locale} onselect={selectCountry} />
            {flag.slice(1).map((selectedFlagItem) => <img key={selectedFlagItem} src={getFlagUrl(selectedFlagItem)} alt={selectedFlagItem} />)}
            {labelProps ? (
                <InputLabel labelProps={labelProps} inputProps={{ ...phoneProps, value: selectedDialCode ? `+${selectedDialCode} ${inputProps.value ?? ""}` : inputProps.value }} />
            ) : (
                <Input {...phoneProps} value={selectedDialCode ? `+${selectedDialCode} ${inputProps.value ?? ""}` : inputProps.value} />
            )}
        </div>
    );
}
