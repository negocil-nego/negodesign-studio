import 'package:flutter/material.dart';

class CardThumbnailVideoViewModel extends ChangeNotifier {
  bool isHoverImage = false;

  void setHover(bool hover) {
    if (isHoverImage != hover) {
      isHoverImage = hover;
      notifyListeners();
    }
  }
}

