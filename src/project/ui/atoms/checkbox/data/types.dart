class CheckboxProps {
  final String id;
  final String? name;
  final bool checked;
  final bool disabled;
  final String? label;
  final bool required;
  final void Function(bool)? onchange;

  const CheckboxProps({
    required this.id,
    this.name,
    this.checked = false,
    this.disabled = false,
    this.label,
    this.required = false,
    this.onchange,
  });
}
