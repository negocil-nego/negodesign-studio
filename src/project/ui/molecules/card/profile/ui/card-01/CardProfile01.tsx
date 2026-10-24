// @ts-nocheck
import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton";
import CardButton from "../../../shared/card-button/ui/CardButton";
import CardCartButton from "../../../shared/card-cart-button/ui/CardCartButton";
import CardDescription from "../../../shared/card-description/ui/CardDescription";
import CardFavoriteButton from "../../../shared/card-favorite-button/ui/CardFavoriteButton";
import CardImage from "../../../shared/card-image/ui/CardImage";
import CardLogo from "../../../shared/card-logo/ui/CardLogo";
import CardTags from "../../../shared/card-tags/ui/CardTags";
import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo";
import CardTitle from "../../../shared/card-title/ui/CardTitle";
import CardVideo from "../../../shared/card-video/ui/CardVideo";
import type { CardProfileBaseProps } from "../../data/types";

export default function CardProfile01({
    logoProps,
    titleProps,
    videoProps,
    imageProps,
    buttonProps,
    cartButtonProps,
    descriptionProps,
    favoriteButtonProps,
    thumbnailVideoProps,
    tagsProps,
    actionsProps,
}: CardProfileBaseProps) {
    return (
        <aside className="w-72 rounded-3xl bg-white dark:bg-blender-card border border-blender-border transition-colors overflow-hidden shadow-xl">
            <div className="relative h-32 w-full">
                <div className="h-40 w-full rounded-md object-cover">
                    {thumbnailVideoProps ? (
                        <CardThumbnailVideo {...thumbnailVideoProps} />
                    ) : videoProps ? (
                        <CardVideo {...videoProps} controls={false} />
                    ) : imageProps ? (
                        <CardImage {...imageProps} />
                    ) : null}
                </div>

                <div className="absolute top-2 flex justify-between w-full gap-2 z-10 p-2">
                    {cartButtonProps && <CardCartButton {...cartButtonProps} />}
                    {favoriteButtonProps && <CardFavoriteButton {...favoriteButtonProps} />}
                </div>

                <div className="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 z-10">
                    {logoProps && (
                        <div className="rounded-full overflow-hidden border-4 border-white bg-white shadow-md">
                            <CardLogo {...logoProps} className="w-20 h-20 rounded-full" />
                        </div>
                    )}
                </div>
            </div>

            <div className="px-6 pt-14 pb-6 flex flex-col gap-4">
                <div className="flex flex-col items-center gap-1 z-20 border-white text-white bg-black/5 p-2 font-bold">
                    {titleProps && <CardTitle {...titleProps} />}
                </div>

                {tagsProps && <CardTags {...tagsProps} />}

                {actionsProps && <CardActionsButton {...actionsProps} />}

                <div className="flex flex-col gap-3">
                    {descriptionProps && <CardDescription {...descriptionProps} />}
                </div>

                {buttonProps && (
                    <div className="mt-2">
                        <CardButton {...buttonProps} />
                    </div>
                )}
            </div>
        </aside>
    );
}
