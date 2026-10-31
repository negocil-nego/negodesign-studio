import InputUsername from "../../../ui/InputUsername";
import type { InputUsernameOrEmailProps } from "../data/types";

export default function InputUsernameOrEmail(props: InputUsernameOrEmailProps) {
  return <InputUsername {...props} inputProps={{ ...props.inputProps, icon: props.inputProps.icon ?? "Email" }} validationMode="username-or-email" />;
}
