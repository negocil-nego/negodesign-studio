import 'package:flutter/foundation.dart';
import 'types.dart';

class ListGroupsViewModel extends ChangeNotifier {
  ListGroupsProps _props;
  String? _activeId;

  ListGroupsViewModel(this._props) {
    _activeId = _props.activeId;
  }

  ListGroupsProps get props => _props;
  String? get activeId => _activeId;

  bool isItemActive(ListGroupEntry item) =>
      _activeId != null ? _activeId == item.id : item.active;

  void didUpdate(ListGroupsProps nextProps) {
    _props = nextProps;
    _activeId = nextProps.activeId;
    notifyListeners();
  }

  void select(String id) {
    _activeId = id;
    _props.onSelect?.call(id);
    notifyListeners();
  }
}
