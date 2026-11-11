import 'package:flutter/material.dart';
import '../data/types.dart';
import 'AdminCrud.dart';

class Admin extends StatelessWidget {
  const Admin({super.key, required this.props});
  final AdminProps props;

  @override
  Widget build(BuildContext context) => AdminCrud(props: props);
}

