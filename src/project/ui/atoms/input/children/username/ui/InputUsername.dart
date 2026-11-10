import 'package:flutter/material.dart';
import '../../../icon/ui/Icon.dart' as project_icons;
import '../data/types.dart';
import '../data/viewmodel.dart';

class InputUsername extends StatelessWidget {
  final InputUsernameProps props;

  const InputUsername(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final vm = InputUsernameViewModel(props);
    return TextFormField(
      initialValue: props.inputProps.value,
      enabled: !props.inputProps.disabled,
      keyboardType: TextInputType.text,
      onChanged: props.onValueChange,
      validator: vm.validate,
      decoration: InputDecoration(
        labelText: props.labelProps?.label,
        hintText: props.inputProps.placeholder ?? 'Username, email, or phone',
        prefixIcon: project_icons.Icon(project_icons.IconProps(name: props.inputProps.icon ?? 'User')),
        errorBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.red)),
        focusedErrorBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.red, width: 2)),
      ),
    );
  }
}
