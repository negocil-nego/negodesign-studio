import "../../label/data/types.dart" show InputWithoutLabelProps;

enum UsernameValidationMode {
  any,
  username,
  email,
  phone,
  usernameOrEmail,
  usernameOrPhone,
}

class InputUsernameProps extends InputWithoutLabelProps {
  final String? errorMessage;
  final UsernameValidationMode validationMode;
  final ValueChanged<String>? onValueChange;
  final ValueChanged<bool>? onValidityChange;

  const InputUsernameProps({
    super.labelProps,
    required super.inputProps,
    this.errorMessage,
    this.validationMode = UsernameValidationMode.any,
    this.onValueChange,
    this.onValidityChange,
  });
}
