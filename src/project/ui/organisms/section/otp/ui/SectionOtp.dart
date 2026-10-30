import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../../atoms/button/variant/default/ui/Button.dart';
import '../../../../atoms/input/children/otp/ui/InputOtp.dart';
import '../../../../atoms/text/ui/text.dart' as textui;

class SectionOtp extends StatelessWidget {
  final SectionOtpProps props;
  const SectionOtp(this.props, {super.key});
  @override
  Widget build(BuildContext context) => SizedBox(width: double.infinity, height: props.isScreen ? double.infinity : null, child: Row(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
    if (props.layout == 'dual') Expanded(child: Container(color: const Color(0xff2563eb), padding: const EdgeInsets.all(32), child: Column(crossAxisAlignment: CrossAxisAlignment.start, mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [const Text('N', style: TextStyle(fontWeight: FontWeight.bold)), Column(crossAxisAlignment: CrossAxisAlignment.start, children: [if (props.brandSubtitleProps != null) textui.Text(props.brandSubtitleProps!), if (props.brandTitleProps != null) textui.Text(props.brandTitleProps!), if (props.brandDescriptionProps != null) textui.Text(props.brandDescriptionProps!)]), const Text('© 2026 Negodesign')]))),
    Expanded(child: Center(child: ConstrainedBox(constraints: const BoxConstraints(maxWidth: 448), child: Padding(padding: const EdgeInsets.all(24), child: Column(mainAxisSize: MainAxisSize.min, children: [if (props.titleProps != null) textui.Text(props.titleProps!), if (props.subtitleProps != null) textui.Text(props.subtitleProps!), if (props.descriptionProps != null) Padding(padding: const EdgeInsets.symmetric(vertical: 12), child: textui.Text(props.descriptionProps!)), if (props.inputOtpProps != null) InputOtp(props.inputOtpProps!), if (props.buttonSubmitProps != null) Padding(padding: const EdgeInsets.only(top: 16), child: Button(props.buttonSubmitProps!)), Wrap(spacing: 8, children: props.buttonsLoginProps.map((p) => Button(p)).toList())]))))),
  ]));
}
