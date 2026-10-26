// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
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
        <aside className={StyleComponent.card.media.v02.root}>
            <div className={StyleComponent.card.media.v02.header}>
                <div className={StyleComponent.card.media.v02.headerGroup}>
                    {logoProps && <CardLogo {...logoProps} />}
                    {(titleProps || ratingProps || priceProps) && (
                        <div className={StyleComponent.card.media.v02.titleGroup}>
                            {titleProps && <CardTitle {...titleProps} />}
                            {ratingProps ? <CardRating {...ratingProps} /> : <CardRating rating={0} />}
                            {priceProps && <CardPrice {...priceProps} />}
                        </div>
                    )}
                </div>
                {(cartButtonProps || favoriteButtonProps) && (
                    <div className={StyleComponent.card.media.v02.actions}>
                        {cartButtonProps && <CardCartButton {...cartButtonProps} />}
                        {favoriteButtonProps && <CardFavoriteButton {...favoriteButtonProps} />}
                    </div>
                )}
            </div>

            <div className={StyleComponent.card.media.v02.body}>
                <div className={`${StyleComponent.card.media.frame} ${StyleComponent.card.media.fill}`}>
                    {thumbnailVideoProps ? (
                        <CardThumbnailVideo {...thumbnailVideoProps} visibleButtonPlay />
                    ) : videoProps ? (
                        <CardVideo {...videoProps} />
                    ) : imageProps ? (
                        <CardImage {...imageProps} />
                    ) : null}
                </div>
                <div className={StyleComponent.card.media.v02.description}>
                    {descriptionProps && <CardDescription {...descriptionProps} />}
                </div>
            </div>

            {buttonProps && (
                <div className={StyleComponent.card.media.v02.footer}>
                    <CardButton {...buttonProps} />
                </div>
            )}
        </aside>
    );
}
