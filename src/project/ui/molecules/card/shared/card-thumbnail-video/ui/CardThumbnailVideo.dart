import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import '../../card-image/ui/CardImage.dart';
import '../../card-video/ui/CardVideo.dart';

class CardThumbnailVideo extends StatefulWidget {
  final CardThumbnailVideoProps props;
  const CardThumbnailVideo(this.props, {super.key});

  @override
  State<CardThumbnailVideo> createState() => _CardThumbnailVideoState();
}

class _CardThumbnailVideoState extends State<CardThumbnailVideo> {
  final CardThumbnailVideoViewModel _vm = CardThumbnailVideoViewModel();

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final props = widget.props;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: (_) => _vm.setHover(true),
        onExit: (_) => _vm.setHover(false),
        child: Stack(
          children: [
            CardImage(props.imageProps),
            Positioned.fill(
              child: Visibility(visible: _vm.isHoverImage, child: CardVideo(props.videoProps)),
            ),
            if (props.visibleButtonPlay)
              Positioned.fill(
                child: Visibility(
                  visible: !_vm.isHoverImage,
                  child: Center(
                    child: Container(
                      width: 48,
                      height: 48,
                      decoration: const BoxDecoration(color: Color(0xCCFFFFFF), shape: BoxShape.circle),
                      child: const Icon(Icons.play_arrow, size: 32),
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}
