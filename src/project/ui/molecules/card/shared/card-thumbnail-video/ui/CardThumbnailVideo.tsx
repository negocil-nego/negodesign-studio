// @ts-nocheck
import { useState } from "react";
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardThumbnailVideoProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon";
import CardImage from "../../card-image/ui/CardImage";
import CardVideo from "../../card-video/ui/CardVideo";

export default function CardThumbnailVideo({ videoProps, imageProps, visibleButtonPlay = false }: CardThumbnailVideoProps) {
    const [isHoverImage, setIsHoverImage] = useState(false);

    return (
        <div
            className={StyleComponent.card.cardThumbnailVideo.container}
            role="presentation"
            onMouseEnter={() => setIsHoverImage(true)}
            onMouseLeave={() => setIsHoverImage(false)}
        >
            {/* Imagem pré-carregada (loading eager) — define a altura e fica sempre por baixo */}
            <CardImage {...imageProps} loading="eager" />

            {/* Vídeo montado desde o início com preload="auto": ao passar o rato já está pré-carregado e a reproduzir, pelo que o hover apenas o torna visível. */}
            <div className={`${StyleComponent.card.cardThumbnailVideo.video} ${!isHoverImage ? "invisible" : ""}`}>
                <CardVideo {...videoProps} preload="auto" />
            </div>

            {visibleButtonPlay && (
                <div className={`${StyleComponent.card.cardThumbnailVideo.playButton} ${isHoverImage ? "invisible" : ""}`}>
                    <Icon name="Play" className={StyleComponent.card.cardThumbnailVideo.playIcon} />
                </div>
            )}
        </div>
    );
}
