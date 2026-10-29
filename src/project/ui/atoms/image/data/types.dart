class ImageProps {
  final String? src;
  final String? alt;
  final bool isLoading;
  final void Function()? onClick;
  final String? className;

  const ImageProps({
    this.src,
    this.alt,
    this.isLoading = false,
    this.onClick,
    this.className,
  });
}
