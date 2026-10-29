import InputUsername from "../../../ui/InputUsername";
import type { InputUsernameOrEmailProps } from "../data/types";

export default function InputUsernameOrEmail(props: InputUsernameOrEmailProps) {
  return <InputUsername {...props} validationMode="username-or-email" />;
}
