class SectionGridViewModel {
  const SectionGridViewModel();

  int columnsForWidth(double width) {
    if (width >= 2560) return 5;
    if (width >= 1536) return 4;
    if (width >= 768) return 3;
    if (width >= 640) return 2;
    return 1;
  }
}

