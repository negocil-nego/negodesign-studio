/// Equivalent to TextProps & DataTextProps in TypeScript.
class TextSegment {
  const TextSegment({
    required this.text,
    this.className,
    this.type,
    this.component,
    this.badgeVariant,
    this.dot = false,
    this.actions,
    this.checked,
    this.subtitle,
    this.avatar,
  });

  final String text;
  final String? className;
  final String? type;
  final String? component;
  final String? badgeVariant;
  final bool dot;
  final List<String>? actions;
  final bool? checked;
  final String? subtitle;
  final String? avatar;
}

typedef DataTextProps = TextSegment;
