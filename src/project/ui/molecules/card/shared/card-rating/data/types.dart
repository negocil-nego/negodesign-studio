class CardRatingProps {
  final num rating;
  final bool isLoading;

  const CardRatingProps({
    required this.rating,
    this.isLoading = false,
  });
}
