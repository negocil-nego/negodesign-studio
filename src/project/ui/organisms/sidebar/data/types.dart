import 'package:flutter/material.dart';

class SidebarItemData {
  const SidebarItemData(
    this.label, {
    this.icon = Icons.circle_outlined,
    this.active = false,
    this.badge,
    this.children = const [],
  });
  final String label;
  final IconData icon;
  final bool active;
  final String? badge;
  final List<SidebarItemData> children;
}

class SidebarSectionData {
  const SidebarSectionData({this.title, required this.items});
  final String? title;
  final List<SidebarItemData> items;
}
