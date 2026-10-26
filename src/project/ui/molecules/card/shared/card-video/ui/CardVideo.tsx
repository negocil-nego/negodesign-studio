// @ts-nocheck
import { useEffect, useRef } from "react";
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardVideoProps } from "../data/types";

export default function CardVideo({ src, poster, preload = "metadata", controls = false }: CardVideoProps) {
    const videoRef = useRef<HTMLVideoElement>(null);

    function playVideo() {
        const video = videoRef.current;
        if (!video) return;
        // O WebKit ignora por vezes o atributo `muted`; forçar a propriedade é o que permite o autoplay
        video.muted = true;
        video.play().catch(() => {});
    }

    useEffect(playVideo, []);

    return (
        <video
            ref={videoRef}
            src={src}
            poster={poster}
            preload={preload}
            controls={controls}
            autoPlay
            muted
            loop
            playsInline
            onCanPlay={playVideo}
            className={StyleComponent.card.media.default}
        />
    );
}
