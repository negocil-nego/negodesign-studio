class CardTag {
  final String? icon;
  final String title;
  final String? description;

  const CardTag({
    this.icon,
    required this.title,
    this.description,
  });
}

class CardTagsProps {
  final List<CardTag> tags;
  final String? className;
  final String? itemClassName;
  final String? titleClassName;
  final String? descriptionClassName;

  const CardTagsProps({
    required this.tags,
    this.className,
    this.itemClassName,
    this.titleClassName,
    this.descriptionClassName,
  });
}
