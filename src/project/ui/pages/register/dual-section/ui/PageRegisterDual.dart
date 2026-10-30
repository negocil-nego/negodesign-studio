import 'package:flutter/material.dart';
import '..\..\..\..\templates\section\dual-section\ui\DualSection.dart';
import '../../../../organisms/section/form/data/types.dart';
import '..\..\..\..\molecules\form\ui\Form.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/button/variant/default/type/types.dart';
class PageRegisterDual extends StatelessWidget { const PageRegisterDual({super.key}); @override Widget build(BuildContext context) => DualSection(leftSectionForm:const SectionFormProps(isScreen:true,className:'bg-blue-600 text-white',titleProps:TextProps(Text:'Negodesign'),subtitleProps:TextProps(Text:'O seu próximo projecto começa aqui.'),descriptionProps:TextProps(Text:'Junte-se à comunidade e transforme ideias em interfaces.')),rightSectionForm:const SectionFormProps(isScreen:true,titleProps:TextProps(Text:'Criar conta'),subtitleProps:TextProps(Text:'É rápido e gratuito.'),formProps:FormConfig(grid:false,fields:[FormFieldConfig(label:'Nome',hint:'O seu nome',required:true),FormFieldConfig(label:'Email',type:'email',hint:'nome@exemplo.com',required:true),FormFieldConfig(label:'Palavra-passe',type:'password',hint:'Crie uma palavra-passe',required:true)]),buttonSubmitProps:ButtonProps(text:'Criar conta'),buttonsLoginProps:[ButtonProps(text:'Já tem conta? Iniciar sessão',variant:'outline')])); }
