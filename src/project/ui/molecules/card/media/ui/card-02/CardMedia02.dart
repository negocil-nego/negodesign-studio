import 'package:flutter/material.dart';
import '../../data/types.dart';
import '../../data/viewmodel.dart';
import '../../../shared/card-button/ui/CardButton.dart';
import '../../../shared/card-cart-button/ui/CardCartButton.dart';
import '../../../shared/card-description/ui/CardDescription.dart';
import '../../../shared/card-favorite-button/ui/CardFavoriteButton.dart';
import '../../../shared/card-logo/ui/CardLogo.dart';
import '../../../shared/card-price/ui/CardPrice.dart';
import '../../../shared/card-rating/ui/CardRating.dart';
import '../../../shared/card-rating/data/types.dart';
import '../../../shared/card-title/ui/CardTitle.dart';

class CardMedia02 extends StatelessWidget {
  final CardMediaBaseProps props;
  const CardMedia02(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final vm = CardMediaViewModel(props);
    return Container(
      width: 350,
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color(0xFFE5E7EB)),
        boxShadow: const [BoxShadow(color: Color(0x14000000), blurRadius: 6, offset: Offset(0, 2))],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: Row(
                  children: [
                    if (props.logoProps != null) CardLogo(props.logoProps!),
                    const SizedBox(width: 12),
                    if (vm.hasHeaderDetails)
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            if (props.titleProps != null) CardTitle(props.titleProps!),
                            props.ratingProps != null ? CardRating(props.ratingProps!) : const CardRating(CardRatingProps(rating: 0)),
                            if (props.priceProps != null) CardPrice(props.priceProps!),
                          ],
                        ),
                      ),
                  ],
                ),
              ),
              if (vm.hasActionButtons)
                Row(children: [
                  if (props.cartButtonProps != null) CardCartButton(props.cartButtonProps!),
                  const SizedBox(width: 8),
                  if (props.favoriteButtonProps != null) CardFavoriteButton(props.favoriteButtonProps!),
                ]),
            ],
          ),
          const SizedBox(height: 16),
          ClipRRect(borderRadius: BorderRadius.circular(6), child: vm.buildMedia()),
          if (props.descriptionProps != null) Padding(padding: const EdgeInsets.symmetric(vertical: 4), child: CardDescription(props.descriptionProps!)),
          if (props.buttonProps != null) Padding(padding: const EdgeInsets.only(top: 4), child: CardButton(props.buttonProps!)),
        ],
      ),
    );
  }
}
