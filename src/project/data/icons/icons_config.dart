/// Unified Icon Configuration for Flutter.
/// Maps semantic icon keys to their local SVG asset path.
class NegoIconsConfig {
  const NegoIconsConfig._();

  static const String basePath = 'assets/hugeicons/svg';

  static const Map<String, String> svgByName = {
    'Age': '$basePath/user-02.svg',
    'ArrowUp': '$basePath/arrow-up-01.svg',
    'ArrowDown': '$basePath/arrow-down-01.svg',
    'ArrowDown01': '$basePath/arrow-down-01.svg',
    'ArrowLeft': '$basePath/square-chevron-left.svg',
    'ArrowRight': '$basePath/square-chevron-right.svg',
    'Cancel': '$basePath/cancel-01.svg',
    'Calendar': '$basePath/calendar-03.svg',
    'Cart': '$basePath/shopping-cart-add-01.svg',
    'CalendarDays': '$basePath/calendar-days.svg',
    'CheckmarkSquare01': '$basePath/checkmark-square-01.svg',
    'Date': '$basePath/calendar-03.svg',
    'DocumentValidation': '$basePath/document-validation.svg',
    'DragDropVertical': '$basePath/drag-drop-vertical.svg',
    'Email': '$basePath/mail-01.svg',
    'FilterHorizontal': '$basePath/filter-horizontal.svg',
    'Favorite': '$basePath/favourite.svg',
    'FolderAdd': '$basePath/folder-add.svg',
    'Home04': '$basePath/home-04.svg',
    'HelpSquare': '$basePath/help-square.svg',
    'Image02': '$basePath/image-02.svg',
    'InputCursorText': '$basePath/input-cursor-text.svg',
    'Label': '$basePath/tag-01.svg',
    'Languages': '$basePath/languages.svg',
    'Layout04': '$basePath/layout-04.svg',
    'LayoutThreeColumn': '$basePath/layout-three-column.svg',
    'Link': '$basePath/link-01.svg',
    'Location': '$basePath/location-01.svg',
    'Message': '$basePath/chat-dot-01.svg',
    'MoreHorizontal': '$basePath/more-horizontal.svg',
    'MoreVertical': '$basePath/more-vertical.svg',
    'Number': '$basePath/square-01.svg',
    'Password': '$basePath/security-password.svg',
    'Phone': '$basePath/call.svg',
    'Play': '$basePath/play.svg',
    'Search01': '$basePath/search-01.svg',
    'Security': '$basePath/security.svg',
    'Settings01': '$basePath/settings-01.svg',
    'Star': '$basePath/star.svg',
    'Text': '$basePath/text.svg',
    'TextFont': '$basePath/text-font.svg',
    'Time': '$basePath/clock-01.svg',
    'User': '$basePath/user.svg',
  };

  static String resolveSvgPath(String name) =>
      svgByName[name] ?? '$basePath/$name.svg';
}
