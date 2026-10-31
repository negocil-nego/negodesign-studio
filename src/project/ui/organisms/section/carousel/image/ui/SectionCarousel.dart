import 'package:flutter/material.dart';
import '../data/types.dart';

class SectionCarousel extends StatefulWidget {
  final SectionImageCarouselProps props;
  const SectionCarousel(this.props, {super.key});

  @override
  State<SectionCarousel> createState() => _SectionCarouselState();
}

class _SectionCarouselState extends State<SectionCarousel> {
  static final RegExp _colorTokens = RegExp(
    r'#[0-9a-fA-F]{3,8}|(?:bg-|from-|to-)?[a-z]+(?:-\d{2,3})?',
  );
  static final RegExp _gradientColors = RegExp(
    r'#[0-9a-fA-F]{3,8}|(?:white|black|red|blue|green|purple|pink)',
  );
  static final RegExp _fromColor = RegExp(r'from-([^\s,)]+)');
  static final RegExp _toColor = RegExp(r'to-([^\s,)]+)');
  static final RegExp _colorPrefix = RegExp(r'^(bg-|from-|to-)');
  static const Map<String, int> _namedColors = {
    'white': 0xffffffff,
    'black': 0xff000000,
    'transparent': 0x00000000,
    'red': 0xffef4444,
    'blue': 0xff3b82f6,
    'green': 0xff22c55e,
    'blue-500': 0xff3b82f6,
    'blue-600': 0xff2563eb,
    'blue-700': 0xff1d4ed8,
    'cyan-500': 0xff06b6d4,
    'indigo-500': 0xff6366f1,
    'indigo-600': 0xff4f46e5,
    'indigo-700': 0xff4338ca,
    'purple-500': 0xffa855f7,
    'purple-600': 0xff9333ea,
    'purple-700': 0xff7e22ce,
    'pink-500': 0xffec4899,
    'red-500': 0xffef4444,
    'green-500': 0xff22c55e,
    'teal-500': 0xff14b8a6,
    'slate-700': 0xff334155,
    'slate-800': 0xff1e293b,
    'zinc-900': 0xff18181b,
  };
  int activeIndex = 0;

  ImageProvider<Object>? _imageProvider(String? source) {
    if (source == null || source.isEmpty) return null;
    final uri = Uri.tryParse(source);
    if (uri != null && (uri.scheme == 'http' || uri.scheme == 'https')) {
      return NetworkImage(source);
    }
    return AssetImage(source);
  }

  @override
  Widget build(BuildContext context) {
    final items = widget.props.items;
    if (items.isEmpty) return const SizedBox.shrink();
    final currentIndex = activeIndex.clamp(0, items.length - 1).toInt();
    final item = items[currentIndex];
    final image = _imageProvider(item.backgroundImage);
    final backgroundGradient = _parseGradient(widget.props.backgroundColor);

    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: AspectRatio(
        aspectRatio: 4 / 5,
        child: Container(
          width: double.infinity,
          decoration: BoxDecoration(
            color: _parseColor(widget.props.backgroundColor) ?? const Color(0xff18181b),
          ),
          child: Stack(
            children: [
              if (image != null)
                Positioned.fill(
                  child: DecoratedBox(
                    decoration: BoxDecoration(
                      image: DecorationImage(image: image, fit: BoxFit.cover),
                    ),
                  ),
                ),
              if (backgroundGradient != null)
                Positioned.fill(
                  child: DecoratedBox(
                    decoration: BoxDecoration(gradient: backgroundGradient),
                  ),
                ),
              const DecoratedBox(
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    begin: Alignment.topCenter,
                    end: Alignment.bottomCenter,
                    colors: [Color(0x08000000), Color(0xb8000000)],
                  ),
                ),
                child: SizedBox.expand(),
              ),
              Align(
                alignment: Alignment.bottomCenter,
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(24, 24, 24, 56),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      if (item.title != null)
                        Text(item.title!.Text, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.w600)),
                      if (item.description != null)
                        Padding(
                          padding: const EdgeInsets.only(top: 8),
                          child: Text(item.description!.Text, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70, fontSize: 14, height: 1.5)),
                        ),
                    ],
                  ),
                ),
              ),
              if (items.length > 1)
                Positioned(
                  bottom: 20,
                  left: 0,
                  right: 0,
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: List.generate(items.length, (index) => Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 4),
                      child: Semantics(
                        button: true,
                        label: 'Mostrar slide ${index + 1}',
                        selected: currentIndex == index,
                        child: InkWell(
                          customBorder: const CircleBorder(),
                          onTap: () => setState(() => activeIndex = index),
                          child: AnimatedContainer(
                            duration: const Duration(milliseconds: 180),
                            width: currentIndex == index ? 20 : 8,
                            height: 8,
                            decoration: BoxDecoration(
                              color: currentIndex == index ? Colors.white : Colors.black45,
                              border: Border.all(color: Colors.white70),
                              borderRadius: BorderRadius.circular(99),
                            ),
                          ),
                        ),
                      ),
                    )),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }

  Color? _parseColor(String? value) {
    if (value == null || value.isEmpty) return null;
    for (final match in _colorTokens.allMatches(value)) {
      final token = match.group(0)!.replaceFirst(_colorPrefix, '');
      if (_namedColors.containsKey(token)) return Color(_namedColors[token]!);
      final normalized = token.replaceFirst('#', '');
      if (normalized.length == 3) {
        final expanded = normalized.split('').map((digit) => '$digit$digit').join();
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

  LinearGradient? _parseGradient(String? value) {
    if (value == null || value.isEmpty) return null;
    final fromMatch = _fromColor.firstMatch(value);
    final toMatch = _toColor.firstMatch(value);
    final colors = <Color>[];
    if (fromMatch != null) {
      final color = _parseColor(fromMatch.group(1));
      if (color != null) colors.add(color);
    }
    if (toMatch != null) {
      final color = _parseColor(toMatch.group(1));
      if (color != null) colors.add(color);
    }
    if (colors.length < 2 && value.contains('gradient(')) {
      for (final match in _gradientColors.allMatches(value)) {
        final color = _parseColor(match.group(0));
        if (color != null) colors.add(color);
      }
    }
    if (colors.length < 2) return null;

    var begin = Alignment.topLeft;
    var end = Alignment.bottomRight;
    if (value.contains('to right') || value.contains('to-r')) {
      begin = Alignment.centerLeft;
      end = Alignment.centerRight;
    } else if (value.contains('to left') || value.contains('to-l')) {
      begin = Alignment.centerRight;
      end = Alignment.centerLeft;
    } else if (value.contains('to bottom') || value.contains('to-b')) {
      begin = Alignment.topCenter;
      end = Alignment.bottomCenter;
    } else if (value.contains('to top') || value.contains('to-t')) {
      begin = Alignment.bottomCenter;
      end = Alignment.topCenter;
    }
    return LinearGradient(begin: begin, end: end, colors: colors.take(2).toList());
  }
}
