// @ts-nocheck
import { getIconClass, IconNamed } from "../../../../../hooks/icon.svelte";
import type { IconProps } from "../data/types";

export default function Icon({ name }: IconProps) {
    return <i className={`${getIconClass(IconNamed[name])} text-base opacity-75`} aria-hidden="true" />;
}
