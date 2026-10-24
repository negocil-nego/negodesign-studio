import type { IconProps } from "../../../../icon/data/types";

export type IconButtonProps = {
    className?: string;
    icon: IconProps;    
    isLoading?: boolean;
    onClick?: () => void;
}