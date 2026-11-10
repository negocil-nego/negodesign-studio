import 'package:flutter/material.dart';

class PageHeroSimple extends StatelessWidget {
  final String? title;
  final String? subtitle;
  final String? description;
  final String? backgroundImage;
  final Color backgroundColor;
  final bool light;
  final Widget? menu;
  final List<Widget> actions;

  const PageHeroSimple({
    super.key,
    this.title,
    this.subtitle,
    this.description,
    this.backgroundColor = const Color(0xFF10212B),
    this.backgroundImage,
    this.light = false,
    this.menu,
    this.actions = const [],
  });

  @override
  Widget build(BuildContext context) {
    final textColor = light ? Colors.blueGrey : Colors.white70;
    return Container(
      width: double.infinity,
      constraints: const BoxConstraints(minHeight: 600),
      decoration: BoxDecoration(
        color: light ? const Color(0xFFEEF5F6) : backgroundColor,
        image: !light && backgroundImage != null
            ? DecorationImage(
                image: NetworkImage(backgroundImage!),
                fit: BoxFit.cover,
                colorFilter: const ColorFilter.mode(
                  Color(0x99000000),
                  BlendMode.darken,
                ),
              )
            : null,
      ),
      child: Stack(
        children: [
          if (menu != null)
            Positioned(top: 24, left: 24, right: 24, child: menu!),
          Center(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(24, 120, 24, 72),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (title != null)
                    Text(title!, textAlign: TextAlign.center, style: TextStyle(color: textColor)),
                  if (subtitle != null)
                    Padding(
                      padding: const EdgeInsets.only(top: 12),
                      child: Text(
                        subtitle!,
                        textAlign: TextAlign.center,
                        style: TextStyle(
                          fontSize: 42,
                          fontWeight: FontWeight.bold,
                          color: light ? const Color(0xFF18181B) : Colors.white,
                        ),
                      ),
                    ),
                  if (description != null)
                    Padding(
                      padding: const EdgeInsets.only(top: 16),
                      child: Text(description!, textAlign: TextAlign.center, style: TextStyle(color: textColor)),
                    ),
                  if (actions.isNotEmpty)
                    Padding(
                      padding: const EdgeInsets.only(top: 24),
                      child: Wrap(
                        alignment: WrapAlignment.center,
                        spacing: 12,
                        runSpacing: 12,
                        children: actions,
                      ),
                    ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
