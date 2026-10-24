// @ts-nocheck
import { useState } from "react";
import type { CardThumbnailVideoProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon";
import CardImage from "../../card-image/ui/CardImage";
import CardVideo from "../../card-video/ui/CardVideo";

export default function CardThumbnailVideo({ videoProps, imageProps, visibleButtonPlay = false }: CardThumbnailVideoProps) {
    const [isHoverImage, setIsHoverImage] = useState(false);

    return (
        <div
            className="relative cursor-pointer"
            role="presentation"
            onMouseEnter={() => setIsHoverImage(true)}
            onMouseLeave={() => setIsHoverImage(false)}
        >
            {/* Imagem pré-carregada (loading eager) — define a altura e fica sempre por baixo */}
            <CardImage {...imageProps} loading="eager" />

            {/* Vídeo montado desde o início: o hover apenas o torna visível */}
            <div className={`absolute inset-0${!isHoverImage ? " invisible" : ""}`}>
                <CardVideo {...videoProps} preload="auto" />
            </div>

            {visibleButtonPlay && (
                <div className={`absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white/80 size-12 rounded-full flex items-center justify-center z-50${isHoverImage ? " invisible" : ""}`}>
                    <Icon name="Play" className="text-7xl" />
                </div>
            )}
        </div>
    );
}
