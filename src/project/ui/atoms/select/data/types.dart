class SelectOption {
  final String value;
  final String label;
  final String? icon;
  final bool disabled;

  const SelectOption({
    required this.value,
    required this.label,
    this.icon,
    this.disabled = false,
  });
}

class SelectProps {
  final String id;
  final String? name;
  final String? label;
  final List<SelectOption> options;
  final String? value;
  final String? placeholder;
  final bool disabled;
  final bool required;

  const SelectProps({
    required this.id,
    required this.options,
    this.name,
    this.label,
    this.value,
    this.placeholder,
    this.disabled = false,
    this.required = false,
  });
}
