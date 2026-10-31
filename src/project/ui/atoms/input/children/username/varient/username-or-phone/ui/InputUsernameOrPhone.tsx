import InputUsername from "../../../ui/InputUsername";
import type { InputUsernameOrPhoneProps } from "../data/types";

export default function InputUsernameOrPhone(props: InputUsernameOrPhoneProps) {
  return <InputUsername {...props} inputProps={{ ...props.inputProps, icon: props.inputProps.icon ?? "Phone" }} validationMode="username-or-phone" />;
}
