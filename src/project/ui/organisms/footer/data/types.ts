import type { ButtonProps } from "../../../atoms/button/variant/default/type/types";
import type { ImageProps } from "../../../atoms/image/data/types";
import type { TextProps } from "../../../atoms/text/data/types";

export interface FooterLinkItem {
  label: string;
  href?: string;
}

export interface FooterColumnProps {
  title: string;
  links: FooterLinkItem[];
}

export interface FooterContactItem {
  icon?: string;
  text: string;
}

export interface FooterSocialItem {
  icon: string;
  label?: string;
  href?: string;
}

export interface FooterNewsletterProps {
  bannerTitle?: string;
  title?: string;
  description?: string;
  placeholder?: string;
  buttonProps?: ButtonProps;
  onSubmit?: (email: string) => void;
}

export interface FooterProps {
  variant?: "01" | "02" | "03";
  logoProps?: ImageProps;
  logoText?: string;
  logoSubtitle?: string;
  descriptionProps?: TextProps;
  openingHours?: string[];
  contactItems?: FooterContactItem[];
  columns?: FooterColumnProps[];
  newsletterProps?: FooterNewsletterProps;
  socialLinks?: FooterSocialItem[];
  paymentBadges?: string[];
  bottomLinks?: FooterLinkItem[];
  copyrightText?: string;
  className?: string;
}

export interface FooterMainProps {
  variant?: "01" | "02" | "03";
  logoProps?: ImageProps;
  logoText?: string;
  logoSubtitle?: string;
  descriptionProps?: TextProps;
  openingHours?: string[];
  contactItems?: FooterContactItem[];
  columns?: FooterColumnProps[];
  newsletterProps?: FooterNewsletterProps;
  socialLinks?: FooterSocialItem[];
  email?: string;
  onEmailChange?: (email: string) => void;
  onSubmit?: (event: SubmitEvent) => void;
}

export interface FooterBottomProps {
  paymentBadges?: string[];
  bottomLinks?: FooterLinkItem[];
  copyrightText?: string;
}
