import 'package:flutter/material.dart';
import '../../../../organisms/section/otp/data/types.dart';
import '..\..\..\..\organisms\section\otp\ui\SectionOtp.dart';
import '../../../../atoms/input/children/otp/data/types.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/button/variant/default/type/types.dart';
class PageOtp extends StatelessWidget { const PageOtp({super.key}); @override Widget build(BuildContext context) => SectionOtp(SectionOtpProps(isScreen:true,titleProps:const TextProps(Text:'Confirme o seu código'),subtitleProps:const TextProps(Text:'Verificação de segurança'),descriptionProps:const TextProps(Text:'Introduza o código de 6 dígitos enviado para o seu email.'),inputOtpProps:const InputOtpProps(id:'page-otp-code',name:'verification-code',value:'456789',length:6,separatorEvery:3,labelProps:InputOtpLabelProps(label:'Código de verificação',htmlFor:'page-otp-code')),buttonSubmitProps:const ButtonProps(text:'Verificar código'),buttonsLoginProps:const[ButtonProps(text:'Não recebeu o código? Reenviar código',variant:'outline') ])); }
