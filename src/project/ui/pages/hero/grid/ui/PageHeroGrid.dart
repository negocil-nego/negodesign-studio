import 'package:flutter/material.dart';
class PageHeroGrid extends StatelessWidget {
 const PageHeroGrid({super.key,this.menu,this.backgroundColor=const Color(0xFF17161C),this.cards=const []});final Widget? menu;final Color backgroundColor;final List<Widget> cards;
 @override Widget build(BuildContext context)=>Container(width:double.infinity,padding:const EdgeInsets.all(16),decoration:BoxDecoration(color:backgroundColor,borderRadius:BorderRadius.circular(24)),child:Column(children:[if(menu!=null)Padding(padding:const EdgeInsets.only(bottom:12),child:menu!),Expanded(child:GridView.count(crossAxisCount:12,mainAxisSpacing:12,crossAxisSpacing:12,children:cards.map((card)=>ClipRRect(borderRadius:BorderRadius.circular(18),child:card)).toList()))]));
}
