import 'package:flutter/material.dart';
import '../data/types.dart';
import 'card-01/CardProduct01.dart';
import 'card-02/CardProduct02.dart';

class CardProduct extends StatelessWidget {
  final CardProductProps props;

  const CardProduct(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.variant == '02') return CardProduct02(props);
    return CardProduct01(props);
  }
}
