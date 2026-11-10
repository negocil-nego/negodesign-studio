import 'package:flutter/foundation.dart';

class ContainersProps {
  final String title;
  final String lead;
  final String body;
  final String actionLabel;
  final VoidCallback? onAction;

  const ContainersProps({
    this.title = 'Hello, world!',
    this.lead = 'This is a simple hero unit, a simple jumbotron-style component.',
    this.body = 'It uses utility classes for typography and spacing to space content out.',
    this.actionLabel = 'Learn more',
    this.onAction,
  });
}
