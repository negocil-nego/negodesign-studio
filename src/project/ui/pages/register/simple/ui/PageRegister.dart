import 'package:flutter/material.dart';
import '..\..\..\..\templates\section\simple\ui\SimpleSection.dart';
import '../../../../organisms/section/form/data/types.dart';
import '..\..\..\..\molecules\form\ui\Form.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/button/variant/default/type/types.dart';
class PageRegister extends StatelessWidget { const PageRegister({super.key}); @override Widget build(BuildContext context) => SimpleSection(sectionFormProps: SectionFormProps(isScreen:true,titleProps:const TextProps(Text:'Criar conta'),subtitleProps:const TextProps(Text:'Comece hoje'),descriptionProps:const TextProps(Text:'Preencha os dados para se registar.'),formProps:const FormConfig(grid:false,fields:[FormFieldConfig(label:'Nome',hint:'O seu nome',required:true),FormFieldConfig(label:'Email',type:'email',hint:'nome@exemplo.com',required:true),FormFieldConfig(label:'Palavra-passe',type:'password',hint:'Crie uma palavra-passe',required:true)]),buttonSubmitProps:const ButtonProps(text:'Criar conta'),buttonsLoginProps:const[ButtonProps(text:'Já tem conta? Iniciar sessão',variant:'outline') ])); }
