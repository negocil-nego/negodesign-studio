// @ts-nocheck
import CardButton from "../../../shared/card-button/ui/CardButton";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton";
import CardDescription from "../../../shared/card-description/ui/CardDescription";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton";
import CardImage from "../../../shared/card-image/ui/CardImage";
import CardLogo from "../../../shared/card-logo/ui/CardLogo";
import CardPrice from "../../../shared/card-price/ui/CardPrice";
import CardRating from "../../../shared/card-rating/ui/CardRating";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo";
import CardTitle from "../../../shared/card-title/ui/CardTitle";
import CardVideo from "../../../shared/card-video/ui/CardVideo";
import type { CardMediaBaseProps } from "../../data/types";

export default function CardMedia02({
    logoProps,
    titleProps,
    videoProps,
    imageProps,
    priceProps,
    ratingProps,
    buttonProps,
    cartButtonProps,
    descriptionProps,
    favoriteButtonProps,
    thumbnailVideoProps,
}: CardMediaBaseProps) {
    return (
        <aside className="w-87.5 flex flex-col gap-4 p-6 rounded-2xl bg-white dark:bg-blender-card border border-blender-border shadow-sm transition-colors">
            <div className="flex items-start justify-between gap-3">
                <div className="flex items-center gap-3">
                    {logoProps && <CardLogo {...logoProps} />}
                    {(titleProps || ratingProps || priceProps) && (
                        <div className="flex flex-col gap-1">
                            {titleProps && <CardTitle {...titleProps} />}
                            {ratingProps ? <CardRating {...ratingProps} /> : <CardRating rating={0} />}
                            {priceProps && <CardPrice {...priceProps} />}
                        </div>
                    )}
                </div>
                {(cartButtonProps || favoriteButtonProps) && (
                    <div className="flex flex-row gap-2 shrink-0">
                        {cartButtonProps && <CardCartButton {...cartButtonProps} />}
                        {favoriteButtonProps && <CardFavoriteButton {...favoriteButtonProps} />}
                    </div>
                )}
            </div>

            <div className="flex flex-col gap-3">
                <div className="overflow-hidden rounded-md border border-gray-100 h-full w-full">
                    {thumbnailVideoProps ? (
                        <CardThumbnailVideo {...thumbnailVideoProps} visibleButtonPlay />
                    ) : videoProps ? (
                        <CardVideo {...videoProps} />
                    ) : imageProps ? (
                        <CardImage {...imageProps} />
                    ) : null}
                </div>
                <div className="my-1">
                    {descriptionProps && <CardDescription {...descriptionProps} />}
                </div>
            </div>

            {buttonProps && (
                <div className="mt-1">
                    <CardButton {...buttonProps} />
                </div>
            )}
        </aside>
    );
}
