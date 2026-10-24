// @ts-nocheck
import IconButton from "../../../../../atoms/button/shared/icon/ui/IconButton";
import type { CardFavoriteButtonProps } from "../data/types";

export default function CardFavoriteButton({ ...restProps }: CardFavoriteButtonProps) {
    return <IconButton icon={{ name: "Favorite" }} {...restProps} />;
}
