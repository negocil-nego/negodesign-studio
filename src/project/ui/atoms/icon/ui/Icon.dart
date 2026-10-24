import 'package:flutter/material.dart';
import 'package:flutter/material.dart' as material show Icon;
import '../data/types.dart';

/// Mapeia os nomes do registo `IconNamed` (hooks/icon.svelte.ts) para ícones
/// Material equivalentes, uma vez que o Flutter não usa os Hugeicons.
const Map<String, IconData> _iconData = {
  'FolderAdd': Icons.create_new_folder,
  'Layout04': Icons.dashboard,
  'Settings01': Icons.settings,
  'DocumentValidation': Icons.description,
  'HelpSquare': Icons.help,
  'CheckmarkSquare01': Icons.check_box,
  'Languages': Icons.language,
  'Security': Icons.security,
  'Home': Icons.home,
  'Email': Icons.mail,
  'Phone': Icons.call,
  'Password': Icons.password,
  'Date': Icons.calendar_month,
  'Time': Icons.access_time,
  'Number': Icons.numbers,
  'Age': Icons.person_outline,
  'Text': Icons.text_fields,
  'Label': Icons.label,
  'Cart': Icons.shopping_cart,
  'Favorite': Icons.favorite,
  'Star': Icons.star,
  'Play': Icons.play_arrow,
  'MoreHorizontal': Icons.more_horiz,
  'Location': Icons.location_on,
  'Link': Icons.link,
  'User': Icons.person,
  'Message': Icons.chat,
};

/// Aproximação em Flutter do átomo Icon.
/// O nome (String) é traduzido para um IconData Material; quando `isLoading`
/// é verdadeiro apresenta um indicador de progresso circular.
class Icon extends StatelessWidget {
  final IconProps props;
  const Icon(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.isLoading) {
      return const SizedBox(
        width: 20,
        height: 20,
        child: CircularProgressIndicator(strokeWidth: 2),
      );
    }
    return material.Icon(
      _iconData[props.name] ?? Icons.help_outline,
      size: 16,
      color: Colors.black54,
    );
  }
}
