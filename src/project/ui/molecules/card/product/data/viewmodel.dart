import 'package:flutter/material.dart';
import '../shared/card-image/ui/CardImage.dart';
import '../shared/card-thumbnail-video/ui/CardThumbnailVideo.dart';
import '../shared/card-video/data/types.dart';
import '../shared/card-video/ui/CardVideo.dart';
import 'types.dart';

class CardProductViewModel {
  final CardProductBaseProps props;
  const CardProductViewModel(this.props);

  bool get hasMinFiles => props.minFileProps != null && props.minFileProps!.isNotEmpty;

  Widget buildMedia() {
    if (props.thumbnailVideoProps != null) return CardThumbnailVideo(props.thumbnailVideoProps!);
    if (props.videoProps != null) {
      final v = props.videoProps!;
      return CardVideo(CardVideoProps(src: v.src, poster: v.poster, preload: v.preload, controls: false));
    }
    if (props.imageProps != null) return CardImage(props.imageProps!);
    return const SizedBox.shrink();
  }
}

