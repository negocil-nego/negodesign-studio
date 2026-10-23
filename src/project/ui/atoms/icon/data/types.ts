import { IconNamed } from "../../../../../hooks/icon.svelte";

/** Nome semântico de um ícone registado em `IconNamed` (ex.: `"Home"`). */
export type IconName = keyof typeof IconNamed;

export type IconProps = {
  /** Nome do ícone do registo `IconNamed`. */
  name: IconName;
  /** Se o ícone está carregando. */
  isLoading?: boolean;
  /** Classes adicionais. */
  className?: string;
};
