import 'package:flutter/material.dart';
import 'types.dart';

class SectionGridCarouselViewModel extends ChangeNotifier {
  SectionGridCarouselProps props;
  final PageController pageController = PageController();
  int currentPage = 0;

  SectionGridCarouselViewModel(this.props);

  void didUpdate(SectionGridCarouselProps newProps) {
    props = newProps;
  }

  int get columns => props.columns.clamp(1, 1000).toInt();
  int get rows => props.rows.clamp(1, 1000).toInt();
  int get pageSize => rows * columns;

  List<List<Widget>> get pages {
    final result = <List<Widget>>[];
    for (var i = 0; i < props.children.length; i += pageSize) {
      result.add(props.children.skip(i).take(pageSize).toList());
    }
    return result;
  }

  bool get canGoPrevious => currentPage > 0;
  bool get canGoNext => currentPage < pages.length - 1;

  void onPageChanged(int page) {
    currentPage = page;
    notifyListeners();
  }

  void previousPage() {
    if (canGoPrevious) {
      pageController.previousPage(duration: const Duration(milliseconds: 300), curve: Curves.easeInOut);
    }
  }

  void nextPage() {
    if (canGoNext) {
      pageController.nextPage(duration: const Duration(milliseconds: 300), curve: Curves.easeInOut);
    }
  }

  @override
  void dispose() {
    pageController.dispose();
    super.dispose();
  }
}

