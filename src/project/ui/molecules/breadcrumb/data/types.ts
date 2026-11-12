export interface BreadcrumbItem {
  label: string;
  href?: string;
  active?: boolean;
}

export interface BreadcrumbsProps {
  items?: BreadcrumbItem[];
  onSelect?: (item: BreadcrumbItem, index: number) => void;
}

export type BreadcrumbProps = BreadcrumbsProps;
