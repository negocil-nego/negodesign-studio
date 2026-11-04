export type SidebarItemProps = {
  label: string;
  icon?: string;
  href?: string;
  active?: boolean;
  badge?: string | number;
  children?: SidebarItemProps[];
};
export type SidebarSectionProps = { title?: string; items: SidebarItemProps[] };
export type SidebarProfileProps = {
  name: string;
  detail?: string;
  avatar?: string;
};
export type SidebarProps = {
  variant?: "01" | "02" | "03" | "04";
  theme?: "light" | "dark";
  collapsed?: boolean;
  logo?: string;
  brand?: string;
  workspace?: string;
  sections?: SidebarSectionProps[];
  footerSections?: SidebarSectionProps[];
  user?: SidebarProfileProps;
  storage?: { title: string; description: string; usage: number };
  className?: string;
};
