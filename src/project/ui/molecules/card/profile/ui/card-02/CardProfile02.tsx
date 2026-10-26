// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
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
    const style = StyleComponent.card.profile.v02;
    return (
        <aside className={style.root}>
            <div className={style.cover}>
                <div className={StyleComponent.card.profile.fill}>
                    {thumbnailVideoProps ? (
                        <CardThumbnailVideo {...thumbnailVideoProps} />
                    ) : videoProps ? (
                        <CardVideo {...videoProps} controls={false} />
                    ) : imageProps ? (
                        <CardImage {...imageProps} />
                    ) : null}
                </div>

                <div className={style.actions}>
                    {cartButtonProps && <CardCartButton {...cartButtonProps} />}
                    {favoriteButtonProps && <CardFavoriteButton {...favoriteButtonProps} />}
                </div>
            </div>

            <div className={style.body}>
                <div className="relative">
                    {logoProps && (
                        <div className={style.logoFrame}>
                            <CardLogo {...logoProps} className={style.logoImage} />
                        </div>
                    )}
                </div>

                {titleProps && <CardTitle {...titleProps} className={style.infoTitle} />}

                {ratingProps && <CardRating {...ratingProps} />}

                {tagsProps && <CardTags {...tagsProps} />}

                {actionsProps && <CardActionsButton {...actionsProps} />}

                {descriptionProps && <CardDescription {...descriptionProps} />}

                {buttonProps && <CardButton {...buttonProps} />}
            </div>
        </aside>
    );
}
