class ToggleProps {
  final String id;
  final String? name;
  final bool checked;
  final bool disabled;
  final String? label;
  final void Function(bool)? onchange;

  const ToggleProps({
    required this.id,
    this.name,
    this.checked = false,
    this.disabled = false,
    this.label,
    this.onchange,
  });
}
