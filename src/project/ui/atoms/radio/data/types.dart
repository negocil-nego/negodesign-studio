class RadioOption {
  final String value;
  final String label;
  final bool disabled;

  const RadioOption({required this.value, required this.label, this.disabled = false});
}

class RadioProps {
  final String name;
  final List<RadioOption> options;
  final String? value;
  final bool disabled;
  final void Function(String)? onchange;

  const RadioProps({
    required this.name,
    required this.options,
    this.value,
    this.disabled = false,
    this.onchange,
  });
}
