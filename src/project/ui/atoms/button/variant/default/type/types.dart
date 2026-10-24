import '../../../shared/base/data/types.dart';

class ButtonProps extends ButtonBaseProps {
  final String variant;

  const ButtonProps({
    super.icon,
    super.text,
    super.className,
    super.isLoading,
    super.isSkeleton,
    super.isDisabled,
    super.onClick,
    this.variant = 'solid',
  });
}
