import type { ButtonProps } from "../../../../../atoms/button/variant/default/type/types";
import type { TextProps } from "../../../../../atoms/text/data/types";

export type SectionCardGridProps = {
  titleProps?: TextProps;
  subtitleProps?: TextProps;
  descriptionProps?: TextProps;
  buttonsProps?: ButtonProps[];
  tone?: "light" | "dark";
  contentPosition?: "start" | "center" | "end" | "between";
};
