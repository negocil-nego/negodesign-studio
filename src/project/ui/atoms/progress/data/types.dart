class ProgressBarSegment {
  final double value;
  final String variant;
  final bool striped;
  final bool animated;

  const ProgressBarSegment({
    required this.value,
    this.variant = 'primary',
    this.striped = false,
    this.animated = false,
  });
}

class ProgressProps {
  final double value;
  final String variant;
  final bool striped;
  final bool animated;
  final List<ProgressBarSegment> bars;

  const ProgressProps({
    this.value = 25,
    this.variant = 'primary',
    this.striped = false,
    this.animated = false,
    this.bars = const [],
  });
}
