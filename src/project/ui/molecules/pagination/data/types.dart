import 'package:flutter/foundation.dart';

class PaginationNavProps {
  final int totalPages;
  final int currentPage;
  final String size;
  final ValueChanged<int>? onPageChange;

  const PaginationNavProps({
    this.totalPages = 5,
    this.currentPage = 1,
    this.size = 'md',
    this.onPageChange,
  });
}
