import '../../../../icon/data/types.dart';

class IconButtonProps {
  final String? className;
  final IconProps icon;
  final bool isLoading;
  final void Function()? onClick;

  const IconButtonProps({
    required this.icon,
    this.className,
    this.isLoading = false,
    this.onClick,
  });
}
