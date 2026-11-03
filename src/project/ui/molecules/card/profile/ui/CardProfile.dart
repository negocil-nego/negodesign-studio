import 'package:flutter/material.dart';
import '../data/types.dart';
import 'card-01/CardProfile01.dart';
import 'card-02/CardProfile02.dart';
import 'card-03/CardProfile03.dart';

class CardProfile extends StatelessWidget {
  final CardProfileProps props;

  const CardProfile(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.variant == '03') {
      return CardProfile03(props);
    }
    if (props.variant == '02') return CardProfile02(props);
    return CardProfile01(props);
  }
}
