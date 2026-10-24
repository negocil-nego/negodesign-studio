class AvatarProps {
  final String? src;
  final String? alt;
  final bool isLoading;
  final void Function()? onClick;
  final String? className;

  const AvatarProps({
    this.src,
    this.alt,
    this.isLoading = false,
    this.onClick,
    this.className,
  });
}
