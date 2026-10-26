// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
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
    const style = StyleComponent.card.profile.v01;
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

                <div className={style.logo}>
                    {logoProps && (
                        <div className={style.logoFrame}>
                            <CardLogo {...logoProps} className={style.logoImage} />
                        </div>
                    )}
                </div>
            </div>

            <div className={style.body}>
                <div className={style.titleGroup}>
                    {titleProps && <CardTitle {...titleProps} />}
                </div>

                {tagsProps && <CardTags {...tagsProps} />}

                {actionsProps && <CardActionsButton {...actionsProps} />}

                <div className={style.descriptionGroup}>
                    {descriptionProps && <CardDescription {...descriptionProps} />}
                </div>

                {buttonProps && (
                    <div className={style.footer}>
                        <CardButton {...buttonProps} />
                    </div>
                )}
            </div>
        </aside>
    );
}
