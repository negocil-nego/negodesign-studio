import 'package:flutter/material.dart';
import '../data/types.dart';
import 'card-01/CardMedia01.dart';
import 'card-02/CardMedia02.dart';

class CardMedia extends StatelessWidget {
  final CardMediaProps props;

  const CardMedia(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.variant == '02') return CardMedia02(props);
    return CardMedia01(props);
  }
}
