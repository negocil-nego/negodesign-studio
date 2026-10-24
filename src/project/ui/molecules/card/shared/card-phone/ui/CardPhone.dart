import 'package:flutter/material.dart';
import '../data/types.dart';

class CardPhone extends StatelessWidget {
  final CardPhoneProps props;

  const CardPhone(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      props.phone,
      style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w500),
    );
  }
}
