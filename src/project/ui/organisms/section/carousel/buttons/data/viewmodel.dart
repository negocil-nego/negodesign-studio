import 'package:flutter/material.dart';

class SectionCarouselButtonsViewModel {
  final ScrollController controller = ScrollController();

  void scroll(int direction) {
    if (!controller.hasClients) return;
    controller.animateTo(
      (controller.offset + direction * controller.position.viewportDimension * 0.8)
          .clamp(0, controller.position.maxScrollExtent)
          .toDouble(),
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  double cardWidth(double maxWidth) => (maxWidth - 60) / 4;

  void dispose() {
    controller.dispose();
  }
}

