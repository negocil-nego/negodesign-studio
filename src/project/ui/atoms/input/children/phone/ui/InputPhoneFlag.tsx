// @ts-nocheck
import { useState } from "react";
import type { InputPhoneFlagProps } from "../data/types";
import type { CountryPhoneOption } from "../../../../../molecules/popover/data/country-phone";
import CountryPhonePopover from "../../../../../molecules/popover/ui/CountryPhonePopover";
import Label from "../../../../label/ui/Label";

export default function InputPhoneFlag({ labelProps, inputProps, flag, locale, onCountrySelect }: InputPhoneFlagProps) {
    const [selectedFlag, setSelectedFlag] = useState(flag[0] ?? "ao.svg");
    const inputValue = inputProps.value;

    function selectCountry(country: CountryPhoneOption) {
        setSelectedFlag(country.flag);
        onCountrySelect?.(country);
    }

    return (
        <div className="input-phone-field">
            {labelProps && <Label {...labelProps} />}
            <div className="input-form input-phone-flag">
                <CountryPhonePopover flag={selectedFlag} locale={locale} onselect={selectCountry} />
                {inputProps.icon && <img src={`/icons/${inputProps.icon}`} alt="" />}
                <input
                    id={inputProps.id}
                    type="tel"
                    required={inputProps.required}
                    disabled={inputProps.disabled}
                    placeholder={inputProps.placeholder}
                    defaultValue={inputValue}
                    data-name="label"
                />
            </div>
        </div>
    );
}



