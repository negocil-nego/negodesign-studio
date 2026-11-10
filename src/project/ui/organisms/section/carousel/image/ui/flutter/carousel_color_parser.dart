import 'package:flutter/material.dart';

class CarouselColorParser {
  static final RegExp _colorTokens = RegExp(r'#[0-9a-fA-F]{3,8}|(?:bg-|from-|to-)?[a-z]+(?:-\d{2,3})?');
  static final RegExp _gradientColors = RegExp(r'#[0-9a-fA-F]{3,8}|(?:white|black|red|blue|green|purple|pink)');
  static final RegExp _fromColor = RegExp(r'from-([^\s,)]+)');
  static final RegExp _toColor = RegExp(r'to-([^\s,)]+)');
  static final RegExp _colorPrefix = RegExp(r'^(bg-|from-|to-)');

  static const Map<String, int> _namedColors = {
    'white': 0xffffffff, 'black': 0xff000000, 'transparent': 0x00000000,
    'red': 0xffef4444, 'blue': 0xff3b82f6, 'green': 0xff22c55e,
    'blue-500': 0xff3b82f6, 'blue-600': 0xff2563eb, 'blue-700': 0xff1d4ed8,
    'cyan-500': 0xff06b6d4, 'indigo-500': 0xff6366f1, 'indigo-600': 0xff4f46e5,
    'indigo-700': 0xff4338ca, 'purple-500': 0xffa855f7, 'purple-600': 0xff9333ea,
    'purple-700': 0xff7e22ce, 'pink-500': 0xffec4899, 'red-500': 0xffef4444,
    'green-500': 0xff22c55e, 'teal-500': 0xff14b8a6, 'slate-700': 0xff334155,
    'slate-800': 0xff1e293b, 'zinc-900': 0xff18181b,
  };

  static Color? parseColor(String? value) {
    if (value == null || value.isEmpty) return null;
    for (final match in _colorTokens.allMatches(value)) {
      final token = match.group(0)!.replaceFirst(_colorPrefix, '');
      if (_namedColors.containsKey(token)) return Color(_namedColors[token]!);
      final normalized = token.replaceFirst('#', '');
      if (normalized.length == 3) {
        final expanded = normalized.split('').map((d) => '$d$d').join();
        final parsed = int.tryParse('FF$expanded', radix: 16);
        if (parsed != null) return Color(parsed);
      }
      if (normalized.length == 6) {
        final parsed = int.tryParse('FF$normalized', radix: 16);
        if (parsed != null) return Color(parsed);
      }
      if (normalized.length == 8) {
        final parsed = int.tryParse(normalized, radix: 16);
        if (parsed != null) return Color(parsed);
      }
    }
    return null;
  }

  static LinearGradient? parseGradient(String? value) {
    if (value == null || value.isEmpty) return null;
    final fromMatch = _fromColor.firstMatch(value);
    final toMatch = _toColor.firstMatch(value);
    final colors = <Color>[];
    if (fromMatch != null) {
      final c = parseColor(fromMatch.group(1));
      if (c != null) colors.add(c);
    }
    if (toMatch != null) {
      final c = parseColor(toMatch.group(1));
      if (c != null) colors.add(c);
    }
    if (colors.length < 2 && value.contains('gradient(')) {
      for (final match in _gradientColors.allMatches(value)) {
        final c = parseColor(match.group(0));
        if (c != null) colors.add(c);
      }
    }
    if (colors.length < 2) return null;

    var begin = Alignment.topLeft;
    var end = Alignment.bottomRight;
    if (value.contains('to right') || value.contains('to-r')) { begin = Alignment.centerLeft; end = Alignment.centerRight; }
    else if (value.contains('to left') || value.contains('to-l')) { begin = Alignment.centerRight; end = Alignment.centerLeft; }
    else if (value.contains('to bottom') || value.contains('to-b')) { begin = Alignment.topCenter; end = Alignment.bottomCenter; }
    else if (value.contains('to top') || value.contains('to-t')) { begin = Alignment.bottomCenter; end = Alignment.topCenter; }
    return LinearGradient(begin: begin, end: end, colors: colors.take(2).toList());
  }
}

