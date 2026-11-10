/// NegoDesign Package Entrypoint (Flutter)
/// When copied to `lib/negodesign` in a Flutter project,
/// import theme tokens, icons, ViewModels, and core widgets from `negodesign.dart`.
library negodesign;

export 'data/theme/theme_data.dart';
export 'data/icons/icons_config.dart';

// Core Atoms & ViewModels
export 'ui/atoms/text/data/types.dart';
export 'ui/atoms/text/ui/Text.dart';
export 'ui/atoms/icon/data/types.dart';
export 'ui/atoms/icon/ui/Icon.dart';
export 'ui/atoms/badge/data/types.dart';
export 'ui/atoms/badge/ui/Badge.dart';
export 'ui/atoms/select/data/types.dart';
export 'ui/atoms/select/data/viewmodel.dart';
export 'ui/atoms/select/ui/Select.dart';
export 'ui/atoms/input/children/otp/data/types.dart';
export 'ui/atoms/input/children/otp/data/viewmodel.dart';
export 'ui/atoms/input/children/otp/ui/InputOtp.dart';
export 'ui/atoms/input/children/date/data/types.dart';
export 'ui/atoms/input/children/date/data/viewmodel.dart';
export 'ui/atoms/input/children/date/ui/InputDate.dart';
export 'ui/atoms/input/children/time/data/types.dart';
export 'ui/atoms/input/children/time/data/viewmodel.dart';
export 'ui/atoms/input/children/time/ui/InputTime.dart';

// Organisms & ViewModels
export 'ui/organisms/data-table/data/types.dart';
export 'ui/organisms/data-table/data/viewmodel.dart';
export 'ui/organisms/data-table/ui/DataTable.dart';
export 'ui/organisms/data-table/shared/data-table-filter/data/types.dart';
export 'ui/organisms/data-table/shared/data-table-filter/data/viewmodel.dart';
export 'ui/organisms/data-table/shared/data-table-filter/ui/DataTableFilter.dart';
export 'ui/organisms/menu/simple/data/types.dart';
export 'ui/organisms/menu/simple/data/viewmodel.dart';
export 'ui/organisms/menu/simple/ui/Menu.dart';
export 'ui/organisms/sidebar/data/types.dart';
export 'ui/organisms/sidebar/data/viewmodel.dart';
export 'ui/organisms/sidebar/ui/Sidebar.dart';
