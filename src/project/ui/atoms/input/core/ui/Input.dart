import 'package:flutter/material.dart';
import '../data/types.dart';

class _InputField extends StatelessWidget {
  final InputProps props;

  const _InputField({required this.props});

  TextInputType _mapInputType(InputType type) {
    return switch (type) {
      InputType.email         => TextInputType.emailAddress,
      InputType.password      => TextInputType.visiblePassword,
      InputType.number        => TextInputType.number,
      InputType.tel           => TextInputType.phone,
      InputType.url           => TextInputType.url,
      InputType.search        => TextInputType.text,
      InputType.date          => TextInputType.datetime,
      InputType.datetimeLocal => TextInputType.datetime,
      InputType.month         => TextInputType.datetime,
      InputType.week          => TextInputType.datetime,
      InputType.time          => TextInputType.datetime,
      InputType.text          => TextInputType.text,
    };
  }

  @override
  Widget build(BuildContext context) {
    return TextField(
      enabled: !props.disabled,
      obscureText: props.type == InputType.password,
      keyboardType: _mapInputType(props.type),
      controller: props.value != null
          ? TextEditingController(text: props.value)
          : null,
      decoration: InputDecoration(
        hintText: props.placeholder,
      ),
    );
  }
}

class Input extends StatelessWidget {
  final InputProps props;

  const Input({super.key, required this.props});

  @override
  Widget build(BuildContext context) {
    if (props.icon != null) {
      return Row(
        children: [
          Image.asset('assets/icons/${props.icon}'),
          const SizedBox(width: 8),
          Expanded(child: _InputField(props: props)),
        ],
      );
    }

    return _InputField(props: props);
  }
}
