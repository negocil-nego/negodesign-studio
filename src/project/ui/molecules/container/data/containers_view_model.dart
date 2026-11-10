import 'package:flutter/foundation.dart';
import 'types.dart';

class ContainersViewModel extends ChangeNotifier {
  ContainersProps _props;

  ContainersViewModel(this._props);

  ContainersProps get props => _props;

  void didUpdate(ContainersProps nextProps) {
    _props = nextProps;
    notifyListeners();
  }

  void triggerAction() {
    _props.onAction?.call();
  }
}
