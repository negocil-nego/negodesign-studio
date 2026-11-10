import 'package:flutter/material.dart';
import '../shared/card-image/ui/CardImage.dart';
import '../shared/card-thumbnail-video/ui/CardThumbnailVideo.dart';
import '../shared/card-video/data/types.dart';
import '../shared/card-video/ui/CardVideo.dart';
import 'types.dart';

class CardProfileViewModel {
  final CardProfileBaseProps props;
  const CardProfileViewModel(this.props);

  bool get hasAboutSection => props.descriptionProps != null || props.tagsProps != null;

  Widget buildMedia({bool gradientFallback = false}) {
    if (props.thumbnailVideoProps != null) return CardThumbnailVideo(props.thumbnailVideoProps!);
    if (props.videoProps != null) {
      final v = props.videoProps!;
      return CardVideo(CardVideoProps(src: v.src, poster: v.poster, preload: v.preload, controls: false));
    }
    if (props.imageProps != null) return CardImage(props.imageProps!);
    if (gradientFallback) {
      return Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFFFBCFE8), Color(0xFFFDE68A), Color(0xFFFED7AA)],
          ),
        ),
      );
    }
    return const SizedBox.shrink();
  }
}

