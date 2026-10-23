import type { IconProps } from "../../../../icon/data/types";

export type ButtonBaseProps = {
    icon?: IconProps;
    text?: string;
    className?: string;
    isLoading?: boolean;
    isSkeleton?: boolean;
    isDisabled?: boolean;
    onClick?: () => void;
};