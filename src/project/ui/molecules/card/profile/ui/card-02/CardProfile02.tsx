// @ts-nocheck
import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton";
import CardButton from "../../../shared/card-button/ui/CardButton";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton";
import CardDescription from "../../../shared/card-description/ui/CardDescription";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton";
import CardImage from "../../../shared/card-image/ui/CardImage";
import CardLogo from "../../../shared/card-logo/ui/CardLogo";
import CardRating from "../../../shared/card-rating/ui/CardRating";
import CardTags from "../../../shared/card-tags/ui/CardTags";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo";
import CardTitle from "../../../shared/card-title/ui/CardTitle";
import CardVideo from "../../../shared/card-video/ui/CardVideo";
import type { CardProfileBaseProps } from "../../data/types";

export default function CardProfile02({
    logoProps,
    titleProps,
    videoProps,
    imageProps,
    ratingProps,
    buttonProps,
    cartButtonProps,
    descriptionProps,
    favoriteButtonProps,
    thumbnailVideoProps,
    tagsProps,
    actionsProps,
}: CardProfileBaseProps) {
    return (
        <aside className="w-72 rounded-3xl bg-white dark:bg-blender-card border border-blender-border transition-colors overflow-hidden shadow-sm">
            <div className="h-44 w-full relative">
                <div className="h-40 w-full rounded-md object-cover">
                    {thumbnailVideoProps ? (
                        <CardThumbnailVideo {...thumbnailVideoProps} />
                    ) : videoProps ? (
                        <CardVideo {...videoProps} controls={false} />
                    ) : imageProps ? (
                        <CardImage {...imageProps} />
                    ) : null}
                </div>

                <div className="absolute top-2 flex justify-end w-full gap-2 z-10 p-2">
                    {cartButtonProps && <CardCartButton {...cartButtonProps} />}
                    {favoriteButtonProps && <CardFavoriteButton {...favoriteButtonProps} />}
                </div>
            </div>

            <div className="px-6 pb-6 flex flex-col gap-4">
                <div className="relative">
                    {logoProps && (
                        <div className="w-24 h-24 rounded-full border-4 border-white overflow-hidden bg-white shadow-md absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-10">
                            <CardLogo {...logoProps} className="w-full h-full object-cover" />
                        </div>
                    )}
                </div>

                {titleProps && <CardTitle {...titleProps} className="text-2xl font-bold text-gray-900 mt-8" />}

                {ratingProps && <CardRating {...ratingProps} />}

                {tagsProps && <CardTags {...tagsProps} />}

                {actionsProps && <CardActionsButton {...actionsProps} />}

                {descriptionProps && <CardDescription {...descriptionProps} />}

                {buttonProps && <CardButton {...buttonProps} />}
            </div>
        </aside>
    );
}
