import type { IconProps } from "../../../../icon/data/types";

export type IconButtonProps = {
    icon: IconProps;    
    isLoading?: boolean;
    onClick?: () => void;
}