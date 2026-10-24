class CardVideoProps {
  final String src;
  final String? poster;
  final String preload;
  final bool controls;

  const CardVideoProps({
    required this.src,
    this.poster,
    this.preload = 'metadata',
    this.controls = false,
  });
}
