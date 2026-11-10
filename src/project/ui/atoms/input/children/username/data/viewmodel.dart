import 'types.dart';

class InputUsernameViewModel {
  final InputUsernameProps props;

  const InputUsernameViewModel(this.props);

  static final _username = RegExp(r'^[\p{L}\p{N}][\p{L}\p{N}._-]{2,29}$', unicode: true);
  static final _email = RegExp(r'^[^\s@]+@[^\s@]+\.[^\s@]{2,}$');
  static final _phone = RegExp(r'^\+?[1-9]\d{6,14}$');

  bool isValid(String? value) {
    final candidate = (value ?? '').trim();
    if (candidate.isEmpty) return !props.inputProps.required;
    final normalizedPhone = candidate.replaceAll(RegExp(r'[\s().-]'), '');
    final matchesUsername = _username.hasMatch(candidate);
    final matchesEmail = _email.hasMatch(candidate);
    final matchesPhone = _phone.hasMatch(normalizedPhone);

    return switch (props.validationMode) {
      UsernameValidationMode.username => matchesUsername,
      UsernameValidationMode.email => matchesEmail,
      UsernameValidationMode.phone => matchesPhone,
      UsernameValidationMode.usernameOrEmail => matchesUsername || matchesEmail,
      UsernameValidationMode.usernameOrPhone => matchesUsername || matchesPhone,
      UsernameValidationMode.any => matchesUsername || matchesEmail || matchesPhone,
    };
  }

  String get errorMessage =>
      props.errorMessage ??
      switch (props.validationMode) {
        UsernameValidationMode.username => 'Please enter a valid username.',
        UsernameValidationMode.email => 'Please enter a valid email address.',
        UsernameValidationMode.phone => 'Please enter a valid phone number with country code.',
        UsernameValidationMode.usernameOrEmail => 'Please enter a valid username or email.',
        UsernameValidationMode.usernameOrPhone => 'Please enter a valid username or phone number.',
        UsernameValidationMode.any => 'Please enter a valid username, email, or phone number.',
      };

  String? validate(String? value) {
    final valid = isValid(value);
    props.onValidityChange?.call(valid);
    return valid ? null : errorMessage;
  }
}

