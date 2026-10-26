import type { FormIt } from "../../../molecules/form/data/types";

export type LabelProps = FormIt & {
    label: string;
    for: string;
};