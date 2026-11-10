import 'package:flutter/material.dart';
import 'types.dart';

class TextareaViewModel {
  late final TextEditingController controller;

  TextareaViewModel(TextareaProps props) {
    controller = TextEditingController(text: props.value ?? '');
  }

  void didUpdate(TextareaProps oldProps, TextareaProps newProps) {
    if (oldProps.value != newProps.value && (newProps.value ?? '') != controller.text) {
      controller.text = newProps.value ?? '';
    }
  }

  void dispose() {
    controller.dispose();
  }
}

