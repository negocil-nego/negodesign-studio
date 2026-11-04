import 'package:flutter/material.dart';
class PageHeroDual extends StatelessWidget {
 const PageHeroDual({super.key,this.menu,this.leftSection,this.rightSection});final Widget? menu,leftSection,rightSection;
 @override Widget build(BuildContext context)=>Container(width:double.infinity,constraints:const BoxConstraints(minHeight:600),decoration:const BoxDecoration(gradient:LinearGradient(colors:[Color(0xFFB4EAF2),Color(0xFFD8EDFF),Color(0xFFD8EFFF)])),child:Stack(children:[if(menu!=null)Positioned(top:16,left:24,right:24,child:menu!),Padding(padding:const EdgeInsets.fromLTRB(24,120,24,64),child:Center(child:ConstrainedBox(constraints:const BoxConstraints(maxWidth:1440),child:LayoutBuilder(builder:(context,c)=>c.maxWidth>=900?Row(children:[if(leftSection!=null)Expanded(child:leftSection!),if(rightSection!=null)Expanded(child:rightSection!)]):Column(mainAxisAlignment:MainAxisAlignment.center,children:[if(leftSection!=null)leftSection!,if(rightSection!=null)rightSection!])))))]));
}
