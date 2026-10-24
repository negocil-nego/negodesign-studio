import '../../shared/card-actions-button/data/types.dart';
import '../../shared/card-button/data/types.dart';
import '../../shared/card-cart-button/data/types.dart';
import '../../shared/card-description/data/types.dart';
import '../../shared/card-favorite-button/data/types.dart';
import '../../shared/card-image/data/types.dart';
import '../../shared/card-logo/data/types.dart';
import '../../shared/card-rating/data/types.dart';
import '../../shared/card-tags/data/types.dart';
import '../../shared/card-thumbnail-video/data/types.dart';
import '../../shared/card-title/data/types.dart';
import '../../shared/card-video/data/types.dart';

class CardProfileBaseProps {
  final CardLogoProps? logoProps;
  final CardTitleProps? titleProps;
  final CardVideoProps? videoProps;
  final CardImageProps? imageProps;
  final CardRatingProps? ratingProps;
  final CardButtonProps? buttonProps;
  final CardCartButtonProps? cartButtonProps;
  final CardDescriptionProps? descriptionProps;
  final CardFavoriteButtonProps? favoriteButtonProps;
  final CardThumbnailVideoProps? thumbnailVideoProps;
  final CardTagsProps? tagsProps;
  final CardActionsButtonProps? actionsProps;

  const CardProfileBaseProps({
    this.logoProps,
    this.titleProps,
    this.videoProps,
    this.imageProps,
    this.ratingProps,
    this.buttonProps,
    this.cartButtonProps,
    this.descriptionProps,
    this.favoriteButtonProps,
    this.thumbnailVideoProps,
    this.tagsProps,
    this.actionsProps,
  });
}

class CardProfileProps extends CardProfileBaseProps {
  final String variant;

  const CardProfileProps({
    super.logoProps,
    super.titleProps,
    super.videoProps,
    super.imageProps,
    super.ratingProps,
    super.buttonProps,
    super.cartButtonProps,
    super.descriptionProps,
    super.favoriteButtonProps,
    super.thumbnailVideoProps,
    super.tagsProps,
    super.actionsProps,
    this.variant = '01',
  });
}
