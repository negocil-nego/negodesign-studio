import '../../shared/card-button/data/types.dart';
import '../../shared/card-cart-button/data/types.dart';
import '../../shared/card-description/data/types.dart';
import '../../shared/card-favorite-button/data/types.dart';
import '../../shared/card-image/data/types.dart';
import '../../shared/card-logo/data/types.dart';
import '../../shared/card-price/data/types.dart';
import '../../shared/card-rating/data/types.dart';
import '../../shared/card-thumbnail-video/data/types.dart';
import '../../shared/card-title/data/types.dart';
import '../../shared/card-video/data/types.dart';

class CardMediaBaseProps {
  final CardLogoProps? logoProps;
  final CardTitleProps? titleProps;
  final CardVideoProps? videoProps;
  final CardImageProps? imageProps;
  final CardPriceProps? priceProps;
  final CardRatingProps? ratingProps;
  final CardButtonProps? buttonProps;
  final CardCartButtonProps? cartButtonProps;
  final CardDescriptionProps? descriptionProps;
  final CardFavoriteButtonProps? favoriteButtonProps;
  final CardThumbnailVideoProps? thumbnailVideoProps;

  const CardMediaBaseProps({
    this.logoProps,
    this.titleProps,
    this.videoProps,
    this.imageProps,
    this.priceProps,
    this.ratingProps,
    this.buttonProps,
    this.cartButtonProps,
    this.descriptionProps,
    this.favoriteButtonProps,
    this.thumbnailVideoProps,
  });
}

class CardMediaProps extends CardMediaBaseProps {
  final String variant;

  const CardMediaProps({
    super.logoProps,
    super.titleProps,
    super.videoProps,
    super.imageProps,
    super.priceProps,
    super.ratingProps,
    super.buttonProps,
    super.cartButtonProps,
    super.descriptionProps,
    super.favoriteButtonProps,
    super.thumbnailVideoProps,
    this.variant = '01',
  });
}
