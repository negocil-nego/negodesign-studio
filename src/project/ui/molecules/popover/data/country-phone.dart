import 'country_phone_a.dart';
import 'country_phone_b.dart';
import 'country_phone_c.dart';

export 'country_phone_a.dart' show CountryPhoneOption;

const countryPhoneOptions = <CountryPhoneOption>[
  ...countryPhoneOptionsA,
  ...countryPhoneOptionsB,
  ...countryPhoneOptionsC,
];
