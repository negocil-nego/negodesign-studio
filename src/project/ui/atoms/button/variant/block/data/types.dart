import '../../../shared/base/data/types.dart';

class BlockButtonProps extends ButtonBaseProps {
  final String variant;

  const BlockButtonProps({
    required super.label,
    super.onPressed,
    super.disabled = false,
    this.variant = 'primary',
  });
}
