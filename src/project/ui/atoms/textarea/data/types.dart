class TextareaProps {
  final String id;
  final String? name;
  final String? value;
  final String? placeholder;
  final int rows;
  final int? cols;
  final bool disabled;
  final bool required;
  final bool readonly;
  final int? maxlength;

  const TextareaProps({
    required this.id,
    this.name,
    this.value,
    this.placeholder,
    this.rows = 4,
    this.cols,
    this.disabled = false,
    this.required = false,
    this.readonly = false,
    this.maxlength,
  });
}
