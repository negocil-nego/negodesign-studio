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
  final InputType type;
  final bool required;
  final bool disabled;
  final String? placeholder;
  final String? value;

  const InputProps({
    required this.type,
    this.required = false,
    this.disabled = false,
    this.placeholder,
    this.value,
  });
}

class InputWithoutLabelProps {
  final InputProps inputProps;

  const InputWithoutLabelProps({
    required this.inputProps,
  });
}
