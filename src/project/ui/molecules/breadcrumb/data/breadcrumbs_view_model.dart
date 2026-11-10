import 'package:flutter/foundation.dart';
import 'types.dart';

class BreadcrumbsViewModel extends ChangeNotifier {
  BreadcrumbsProps _props;

  BreadcrumbsViewModel(this._props);

  BreadcrumbsProps get props => _props;
  List<BreadcrumbItem> get items => _props.items;

  void didUpdate(BreadcrumbsProps nextProps) {
    _props = nextProps;
    notifyListeners();
  }

  void selectIndex(int index) {
    _props.onSelect?.call(index);
  }
}
