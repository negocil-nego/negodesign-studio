import '../../../../../../atoms/text/data/types.dart';

class MenuTitleProps {
  final String? href;
  final String variant;
  final TextProps? title;
  final TextProps? description;
  final TextProps? link;
  const MenuTitleProps({this.href, this.variant = 'item', this.title, this.description, this.link});
}
