import 'package:flutter/material.dart';
import 'types.dart';

class InputOtpViewModel extends ChangeNotifier {
  InputOtpProps props;
  late List<TextEditingController> controllers;
  late List<FocusNode> focusNodes;

  InputOtpViewModel(this.props) {
    _createFields();
  }

  int get length => props.length.clamp(1, 12).toInt();
  double get groupWidth => length * 44.0 + (length - 1) * 8.0;
  bool get isCentered => props.position == 'center';

  bool shouldShowSeparator(int index) =>
      props.separatorEvery > 0 && index > 0 && index % props.separatorEvery == 0;

  void _createFields() {
    final digits = props.value.replaceAll(RegExp(r'\D'), '').split('');
    controllers = List.generate(
      length,
      (i) => TextEditingController(text: i < digits.length ? digits[i] : ''),
    );
    focusNodes = List.generate(length, (_) => FocusNode());
  }

  void didUpdate(InputOtpProps oldProps, InputOtpProps newProps) {
    props = newProps;
    if (oldProps.length != newProps.length) {
      _disposeFields();
      _createFields();
      notifyListeners();
    } else if (oldProps.value != newProps.value) {
      final digits = newProps.value.replaceAll(RegExp(r'\D'), '').split('');
      for (var i = 0; i < length; i++) {
        controllers[i].text = i < digits.length ? digits[i] : '';
      }
      notifyListeners();
    }
  }

  void updateDigit(int index, String digit) {
    controllers[index].text = digit;
    props.onValueChange?.call(controllers.map((c) => c.text).join());
    if (digit.isNotEmpty && index < length - 1) {
      focusNodes[index + 1].requestFocus();
    }
    notifyListeners();
  }

  void _disposeFields() {
    for (final c in controllers) {
      c.dispose();
    }
    for (final n in focusNodes) {
      n.dispose();
    }
  }

  @override
  void dispose() {
    _disposeFields();
    super.dispose();
  }
}

