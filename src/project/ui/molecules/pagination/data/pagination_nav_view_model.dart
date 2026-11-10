import 'package:flutter/foundation.dart';
import 'types.dart';

class PaginationNavViewModel extends ChangeNotifier {
  PaginationNavProps _props;
  late int _current;

  PaginationNavViewModel(this._props) {
    _current = _props.currentPage.clamp(1, _props.totalPages);
  }

  PaginationNavProps get props => _props;
  int get current => _current;
  List<int> get pages => List<int>.generate(_props.totalPages, (index) => index + 1);

  void didUpdate(PaginationNavProps nextProps) {
    _props = nextProps;
    _current = nextProps.currentPage.clamp(1, nextProps.totalPages);
    notifyListeners();
  }

  void goTo(int page) {
    final next = page.clamp(1, _props.totalPages);
    if (next == _current) return;
    _current = next;
    _props.onPageChange?.call(_current);
    notifyListeners();
  }
}
