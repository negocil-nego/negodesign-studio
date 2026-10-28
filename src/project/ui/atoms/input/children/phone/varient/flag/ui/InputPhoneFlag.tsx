// @ts-nocheck
import { useState } from "react";
import { FlagNamed } from "../../../../../../../../../hooks/flag.svelte";
import type { InputPhoneFlagProps } from "../data/types";
import { getIconClass } from "../../../../../../../../../hooks/icon.svelte";
import type { CountryPhoneOption } from "../../../../../../../molecules/popover/data/country-phone";
import CountryPhonePopover from "../../../../../../../molecules/popover/ui/CountryPhonePopover";
import Label from "../../../../../../label/ui/Label";
import { StyleComponent } from "../../../../../../../../data/style.component";

export default function InputPhoneFlag({ labelProps, inputProps, flag, locale, onCountrySelect }: InputPhoneFlagProps) {
    const style = StyleComponent.input.phone;
    const [selectedFlag, setSelectedFlag] = useState(flag[0] ?? FlagNamed.Ao);

    function handleCountrySelect(country: CountryPhoneOption) {
        setSelectedFlag(country.flag);
        onCountrySelect?.(country);
    }

    return (
        <div className={style.field}>
            {labelProps && <Label {...labelProps} />}
            <div className={style.form}>
                <CountryPhonePopover flag={selectedFlag} locale={locale} onselect={handleCountrySelect} />
                {inputProps.icon && <i className={getIconClass(inputProps.icon)} aria-hidden="true" />}
                <input
                    id={inputProps.id}
                    type="tel"
                    required={inputProps.required}
                    disabled={inputProps.disabled}
                    placeholder={inputProps.placeholder}
                    value={inputProps.value}
                    data-name="label"
                />
            </div>
        </div>
    );
}
