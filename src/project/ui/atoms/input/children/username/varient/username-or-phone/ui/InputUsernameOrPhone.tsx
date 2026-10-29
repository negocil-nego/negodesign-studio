import InputUsername from "../../../ui/InputUsername";
import type { InputUsernameOrPhoneProps } from "../data/types";

export default function InputUsernameOrPhone(props: InputUsernameOrPhoneProps) {
  return <InputUsername {...props} validationMode="username-or-phone" />;
}
