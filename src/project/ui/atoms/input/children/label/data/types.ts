import type { FormIt } from "../../../../../molecules/form/data/types";
import type { LabelProps } from "../../../../label/data/types";
import type { InputProps } from "../../../core/data/types";

export type InputLabelProps = FormIt & {
    labelProps: LabelProps;
    inputProps: InputProps;
};

export type InputWithoutLabelProps = FormIt & {
    labelProps?: LabelProps;
    inputProps: InputProps;
};