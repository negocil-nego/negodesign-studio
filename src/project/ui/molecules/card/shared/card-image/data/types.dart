class CardImageProps {
  final String src;
  final String? alt;
  final String? srcset;
  final String? sizes;
  final String loading;
  final String? fetchpriority;

  const CardImageProps({
    required this.src,
    this.alt,
    this.srcset,
    this.sizes,
    this.loading = 'lazy',
    this.fetchpriority,
  });
}
