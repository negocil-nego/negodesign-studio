export interface PaginationNavProps {
  totalPages?: number;
  currentPage?: number;
  size?: "sm" | "md" | "lg";
  disabledFirst?: boolean;
  onPageChange?: (page: number) => void;
}
