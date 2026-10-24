class IconProps {
  final String name;
  final bool isLoading;
  final String? className;

  const IconProps({
    required this.name,
    this.isLoading = false,
    this.className,
  });
}
