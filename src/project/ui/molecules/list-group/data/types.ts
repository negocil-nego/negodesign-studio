export interface ListGroupEntry {
  id: string;
  label: string;
  badge?: string | number;
  description?: string;
  meta?: string;
  footer?: string;
  active?: boolean;
  disabled?: boolean;
}

export interface ListGroupsProps {
  items?: ListGroupEntry[];
  activeId?: string;
  onSelect?: (id: string) => void;
}
