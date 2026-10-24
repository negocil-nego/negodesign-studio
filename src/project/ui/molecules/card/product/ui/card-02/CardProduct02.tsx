// @ts-nocheck
import CardButton from "../../../shared/card-button/ui/CardButton";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton";
import CardDescription from "../../../shared/card-description/ui/CardDescription";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton";
import CardImage from "../../../shared/card-image/ui/CardImage";
import CardLogo from "../../../shared/card-logo/ui/CardLogo";
import CardMinFile from "../../../shared/card-min-file/ui/CardMinFile";
import CardPrice from "../../../shared/card-price/ui/CardPrice";
import CardRating from "../../../shared/card-rating/ui/CardRating";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo";
import CardTitle from "../../../shared/card-title/ui/CardTitle";
import CardVideo from "../../../shared/card-video/ui/CardVideo";
import type { CardProductBaseProps } from "../../data/types";

export default function CardProduct02({
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
    minFileProps,
}: CardProductBaseProps) {
    return (
        <aside className="w-100 flex gap-4 p-5 rounded-2xl bg-white dark:bg-blender-card border border-blender-border shadow-sm transition-colors">
            <div className="flex flex-col gap-2 w-44 shrink-0">
                <div className="relative h-40 w-full">
                    <div className="overflow-hidden rounded-md border border-gray-100 h-full w-full">
                        {thumbnailVideoProps ? (
                            <CardThumbnailVideo {...thumbnailVideoProps} />
                        ) : videoProps ? (
                            <CardVideo {...videoProps} controls={false} />
                        ) : imageProps ? (
                            <CardImage {...imageProps} />
                        ) : null}
                    </div>

                    <div className="absolute top-2 inset-x-0 flex justify-between gap-2 z-10 p-2">
                        {cartButtonProps && <CardCartButton {...cartButtonProps} />}
                        {favoriteButtonProps && <CardFavoriteButton {...favoriteButtonProps} />}
                    </div>
                </div>

                {minFileProps?.length ? (
                    <div className="flex gap-2 overflow-x-auto">
                        {minFileProps.map((minFile, index) => (
                            <CardMinFile key={index} {...minFile} />
                        ))}
                    </div>
                ) : null}
            </div>

            <div className="flex flex-col gap-3 flex-1 min-w-0">
                <div className="flex items-center gap-3">
                    {logoProps && <CardLogo {...logoProps} />}
                    {titleProps && (
                        <div className="flex flex-col gap-1 min-w-0">
                            <CardTitle {...titleProps} />
                            {ratingProps ? <CardRating {...ratingProps} /> : <CardRating rating={0} />}
                        </div>
                    )}
                </div>

                {priceProps && <CardPrice {...priceProps} />}

                {descriptionProps && <CardDescription {...descriptionProps} />}

                {buttonProps && (
                    <div className="mt-auto">
                        <CardButton {...buttonProps} />
                    </div>
                )}
            </div>
        </aside>
    );
}
