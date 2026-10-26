// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
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

export default function CardProduct01({
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
    const style = StyleComponent.card.product.v01;
    return (
        <aside className={style.root}>
            <div className={style.media}>
                <div className={`${StyleComponent.card.product.frame} ${StyleComponent.card.product.fill}`}>
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

                {priceProps && (
                    <div className={style.priceBadge}>
                        <CardPrice {...priceProps} />
                    </div>
                )}
            </div>

            <div className={style.body}>
                <div className={style.header}>
                    {logoProps && <CardLogo {...logoProps} />}
                    {titleProps && (
                        <div className={style.titleGroup}>
                            <CardTitle {...titleProps} />
                            {ratingProps ? <CardRating {...ratingProps} /> : <CardRating rating={0} />}
                        </div>
                    )}
                </div>

                {minFileProps?.length ? (
                    <div className={style.thumbs}>
                        {minFileProps.map((minFile, index) => (
                            <CardMinFile key={index} {...minFile} />
                        ))}
                    </div>
                ) : null}

                {descriptionProps && <CardDescription {...descriptionProps} />}

                {buttonProps && (
                    <div className={style.footer}>
                        <CardButton {...buttonProps} />
                    </div>
                )}
            </div>
        </aside>
    );
}
