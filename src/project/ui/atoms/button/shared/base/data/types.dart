import '../../../../icon/data/types.dart';

class ButtonBaseProps {
  final IconProps? icon;
  final String? text;
  final String? className;
  final bool isLoading;
  final bool isSkeleton;
  final bool isDisabled;
  final void Function()? onClick;

  const ButtonBaseProps({
    this.icon,
    this.text,
    this.className,
    this.isLoading = false,
    this.isSkeleton = false,
    this.isDisabled = false,
    this.onClick,
  });
}
