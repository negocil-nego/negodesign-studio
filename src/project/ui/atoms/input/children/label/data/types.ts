import type { LabelProps } from "../../../../label/data/types";
import type { InputProps } from "../../../core/data/types";

export type InputLabelProps = {
    labelProps: LabelProps;
    inputProps: InputProps;
};

export type InputWithoutLabelProps = {
    labelProps?: LabelProps;
    inputProps: InputProps;
};