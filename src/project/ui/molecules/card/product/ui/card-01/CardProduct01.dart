import 'package:flutter/material.dart';
import '../../data/types.dart';
import '../../../shared/card-button/ui/CardButton.dart';
import '../../../shared/card-cart-button/ui/CardCartButton.dart';
import '../../../shared/card-description/ui/CardDescription.dart';
import '../../../shared/card-favorite-button/ui/CardFavoriteButton.dart';
import '../../../shared/card-image/ui/CardImage.dart';
import '../../../shared/card-logo/ui/CardLogo.dart';
import '../../../shared/card-min-file/ui/CardMinFile.dart';
import '../../../shared/card-price/ui/CardPrice.dart';
import '../../../shared/card-rating/ui/CardRating.dart';
import '../../../shared/card-rating/data/types.dart';
import '../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.dart';
import '../../../shared/card-title/ui/CardTitle.dart';
import '../../../shared/card-video/ui/CardVideo.dart';
import '../../../shared/card-video/data/types.dart';

class CardProduct01 extends StatelessWidget {
  final CardProductBaseProps props;

  const CardProduct01(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 288,
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color(0xFFE5E7EB)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            height: 224,
            child: Stack(
              children: [
                Positioned.fill(child: _buildMedia()),
                Positioned(
                  top: 8,
                  left: 8,
                  right: 8,
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      if (props.cartButtonProps != null)
                        CardCartButton(props.cartButtonProps!),
                      if (props.favoriteButtonProps != null)
                        CardFavoriteButton(props.favoriteButtonProps!),
                    ],
                  ),
                ),
                if (props.priceProps != null)
                  Positioned(
                    bottom: 8,
                    left: 8,
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 12,
                        vertical: 4,
                      ),
                      decoration: BoxDecoration(
                        color: const Color(0xE6FFFFFF),
                        borderRadius: BorderRadius.circular(999),
                      ),
                      child: CardPrice(props.priceProps!),
                    ),
                  ),
              ],
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    if (props.logoProps != null) CardLogo(props.logoProps!),
                    const SizedBox(width: 12),
                    if (props.titleProps != null)
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            CardTitle(props.titleProps!),
                            props.ratingProps != null
                                ? CardRating(props.ratingProps!)
                                : const CardRating(CardRatingProps(rating: 0)),
                          ],
                        ),
                      ),
                  ],
                ),
                if (props.minFileProps != null && props.minFileProps!.isNotEmpty) ...[
                  const SizedBox(height: 12),
                  SingleChildScrollView(
                    scrollDirection: Axis.horizontal,
                    child: Row(
                      children: props.minFileProps!
                          .map(
                            (minFile) => Padding(
                              padding: const EdgeInsets.only(right: 8),
                              child: CardMinFile(minFile),
                            ),
                          )
                          .toList(),
                    ),
                  ),
                ],
                if (props.descriptionProps != null) ...[
                  const SizedBox(height: 12),
                  CardDescription(props.descriptionProps!),
                ],
                if (props.buttonProps != null) ...[
                  const SizedBox(height: 4),
                  CardButton(props.buttonProps!),
                ],
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
