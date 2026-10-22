class SelectOption {
  final String value;
  final String label;
  final bool disabled;

  const SelectOption({
    required this.value,
    required this.label,
    this.disabled = false,
  });
}

class SelectProps {
  final String id;
  final String? name;
  final List<SelectOption> options;
  final String? value;
  final String? placeholder;
  final bool disabled;
  final bool required;

  const SelectProps({
    required this.id,
    required this.options,
    this.name,
    this.value,
    this.placeholder,
    this.disabled = false,
    this.required = false,
  });
}
