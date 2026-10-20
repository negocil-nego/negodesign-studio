enum InputType {
  text,
  email,
  password,
  number,
  tel,
  url,
  search,
  date,
  datetimeLocal,
  month,
  week,
  time,
}

class InputProps {
  final String? icon;
  final InputType type;
  final bool required;
  final bool disabled;
  final String? placeholder;
  final String? value;

  const InputProps({
    this.icon,
    required this.type,
    this.required = false,
    this.disabled = false,
    this.placeholder,
    this.value,
  });

  InputProps copyWith({
    String? icon,
    InputType? type,
    bool? required,
    bool? disabled,
    String? placeholder,
    String? value,
  }) {
    return InputProps(
      icon: icon ?? this.icon,
      type: type ?? this.type,
      required: required ?? this.required,
      disabled: disabled ?? this.disabled,
      placeholder: placeholder ?? this.placeholder,
      value: value ?? this.value,
    );
  }
}

class InputWithoutLabelProps {
  final InputProps inputProps;

  const InputWithoutLabelProps({
    required this.inputProps,
  });
}
