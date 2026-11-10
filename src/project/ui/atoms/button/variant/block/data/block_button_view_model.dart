import 'package:flutter/foundation.dart';
import 'types.dart';

class BlockButtonViewModel extends ChangeNotifier {
  BlockButtonProps _props;

  BlockButtonViewModel(this._props);

  BlockButtonProps get props => _props;
  bool get isOutline => _props.variant == 'outline';
  bool get isSecondary => _props.variant == 'secondary';

  void didUpdate(BlockButtonProps nextProps) {
    _props = nextProps;
    notifyListeners();
  }

  void trigger() {
    if (!_props.disabled) {
      _props.onPressed?.call();
    }
  }
}
