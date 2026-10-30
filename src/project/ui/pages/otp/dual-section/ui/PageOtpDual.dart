import 'package:flutter/material.dart';
import '../../../../organisms/section/otp/data/types.dart';
import '..\..\..\..\organisms\section\otp\ui\SectionOtp.dart';
import '../../../../atoms/input/children/otp/data/types.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/button/variant/default/type/types.dart';
class PageOtpDual extends StatelessWidget { const PageOtpDual({super.key}); @override Widget build(BuildContext context) => SectionOtp(SectionOtpProps(isScreen:true,layout:'dual',brandSubtitleProps:const TextProps(Text:'Conta protegida'),brandTitleProps:const TextProps(Text:'Mais um passo para continuar.'),brandDescriptionProps:const TextProps(Text:'A verificação em duas etapas ajuda a manter a sua conta segura.'),titleProps:const TextProps(Text:'Introduza o código'),subtitleProps:const TextProps(Text:'Verificação de segurança'),descriptionProps:const TextProps(Text:'Enviámos um código de 6 dígitos para o seu email.'),inputOtpProps:const InputOtpProps(id:'page-otp-dual-code',name:'verification-code',value:'456789',length:6,separatorEvery:3,labelProps:InputOtpLabelProps(label:'Código de verificação',htmlFor:'page-otp-dual-code')),buttonSubmitProps:const ButtonProps(text:'Confirmar código'),buttonsLoginProps:const[ButtonProps(text:'Não recebeu? Enviar novamente',variant:'outline') ])); }
