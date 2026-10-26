// @ts-nocheck
import { getIconClass, IconNamed } from "../../../../../hooks/icon.svelte";
import { StyleComponent } from "../../../../data/style.component";
import type { IconProps } from "../data/types";

export default function Icon({ name, className, isLoading = false }: IconProps) {
    const iconClass = getIconClass(IconNamed[name]);

    if (isLoading) {
        return <span className={StyleComponent.icon.spinner}></span>;
    }
    return <i className={`${iconClass} ${className} ${StyleComponent.icon.default}`} aria-hidden="true" />;
}
