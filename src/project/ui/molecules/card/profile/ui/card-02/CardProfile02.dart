import 'package:flutter/material.dart';
import '../../data/types.dart';
import '../../../shared/card-actions-button/ui/CardActionsButton.dart';
import '../../../shared/card-button/ui/CardButton.dart';
import '../../../shared/card-cart-button/ui/CardCartButton.dart';
import '../../../shared/card-description/ui/CardDescription.dart';
import '../../../shared/card-favorite-button/ui/CardFavoriteButton.dart';
import '../../../shared/card-image/ui/CardImage.dart';
import '../../../shared/card-logo/ui/CardLogo.dart';
import '../../../shared/card-rating/ui/CardRating.dart';
import '../../../shared/card-tags/ui/CardTags.dart';
import '../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.dart';
import '../../../shared/card-title/ui/CardTitle.dart';
import '../../../shared/card-video/ui/CardVideo.dart';
import '../../../shared/card-video/data/types.dart';

class CardProfile02 extends StatelessWidget {
  final CardProfileBaseProps props;

  const CardProfile02(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 288,
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.circular(24),
        border: Border.all(color: const Color(0xFFE5E7EB)),
        boxShadow: const [
          BoxShadow(color: Color(0x14000000), blurRadius: 6, offset: Offset(0, 2)),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            height: 176,
            child: Stack(
              clipBehavior: Clip.none,
              children: [
                Positioned.fill(child: _buildMedia()),
                Positioned(
                  top: 8,
                  left: 8,
                  right: 8,
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      if (props.cartButtonProps != null)
                        CardCartButton(props.cartButtonProps!),
                      const SizedBox(width: 8),
                      if (props.favoriteButtonProps != null)
                        CardFavoriteButton(props.favoriteButtonProps!),
                    ],
                  ),
                ),
              ],
            ),
          ),
          Padding(
            padding: const EdgeInsets.fromLTRB(24, 0, 24, 24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                SizedBox(
                  height: 4,
                  child: Stack(
                    clipBehavior: Clip.none,
                    children: [
                      if (props.logoProps != null)
                        Positioned(
                          top: -48,
                          left: 0,
                          right: 0,
                          child: Center(
                            child: Container(
                              width: 96,
                              height: 96,
                              clipBehavior: Clip.antiAlias,
                              decoration: BoxDecoration(
                                color: Colors.white,
                                shape: BoxShape.circle,
                                border: Border.all(color: Colors.white, width: 4),
                                boxShadow: const [
                                  BoxShadow(color: Color(0x26000000), blurRadius: 6),
                                ],
                              ),
                              child: CardLogo(props.logoProps!),
                            ),
                          ),
                        ),
                    ],
                  ),
                ),
                const SizedBox(height: 32),
                if (props.titleProps != null) CardTitle(props.titleProps!),
                if (props.ratingProps != null) CardRating(props.ratingProps!),
                if (props.tagsProps != null) CardTags(props.tagsProps!),
                if (props.actionsProps != null)
                  CardActionsButton(props.actionsProps!),
                if (props.descriptionProps != null)
                  CardDescription(props.descriptionProps!),
                if (props.buttonProps != null) CardButton(props.buttonProps!),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildMedia() {
    if (props.thumbnailVideoProps != null) {
      return CardThumbnailVideo(props.thumbnailVideoProps!);
    }
    if (props.videoProps != null) {
      final video = props.videoProps!;
      return CardVideo(
        CardVideoProps(
          src: video.src,
          poster: video.poster,
          preload: video.preload,
          controls: false,
        ),
      );
    }
    if (props.imageProps != null) return CardImage(props.imageProps!);
    return const SizedBox.shrink();
  }
}
