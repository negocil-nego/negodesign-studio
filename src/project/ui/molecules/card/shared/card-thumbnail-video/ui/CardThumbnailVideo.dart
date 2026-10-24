import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../card-image/ui/CardImage.dart';
import '../../card-video/ui/CardVideo.dart';

class CardThumbnailVideo extends StatefulWidget {
  final CardThumbnailVideoProps props;

  const CardThumbnailVideo(this.props, {super.key});

  @override
  State<CardThumbnailVideo> createState() => _CardThumbnailVideoState();
}

class _CardThumbnailVideoState extends State<CardThumbnailVideo> {
  bool _isHoverImage = false;

  @override
  Widget build(BuildContext context) {
    final props = widget.props;
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _isHoverImage = true),
      onExit: (_) => setState(() => _isHoverImage = false),
      child: Stack(
        children: [
          // Imagem por baixo — define a altura do bloco
          CardImage(props.imageProps),
          // Vídeo por cima, visível apenas durante o hover
          Positioned.fill(
            child: Visibility(
              visible: _isHoverImage,
              child: CardVideo(props.videoProps),
            ),
          ),
          if (props.visibleButtonPlay)
            Positioned.fill(
              child: Visibility(
                visible: !_isHoverImage,
                child: Center(
                  child: Container(
                    width: 48,
                    height: 48,
                    decoration: const BoxDecoration(
                      color: Color(0xCCFFFFFF),
                      shape: BoxShape.circle,
                    ),
                    child: const Icon(Icons.play_arrow, size: 32),
                  ),
                ),
              ),
            ),
        ],
      ),
    );
  }
}
