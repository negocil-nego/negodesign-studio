import 'package:flutter/material.dart';
import 'types.dart';
import '../ui/flutter/carousel_color_parser.dart';

class SectionCarouselViewModel extends ChangeNotifier {
  SectionImageCarouselProps props;
  int activeIndex = 0;

  SectionCarouselViewModel(this.props);

  void didUpdate(SectionImageCarouselProps newProps) {
    props = newProps;
  }

  int get currentIndex => props.items.isEmpty ? 0 : activeIndex.clamp(0, props.items.length - 1).toInt();
  SectionImageCarouselItem get currentItem => props.items[currentIndex];
  Color get backgroundColor => CarouselColorParser.parseColor(props.backgroundColor) ?? const Color(0xff18181b);
  LinearGradient? get backgroundGradient => CarouselColorParser.parseGradient(props.backgroundColor);

  ImageProvider<Object>? get currentImageProvider {
    final source = currentItem.backgroundImage;
    if (source == null || source.isEmpty) return null;
    final uri = Uri.tryParse(source);
    if (uri != null && (uri.scheme == 'http' || uri.scheme == 'https')) return NetworkImage(source);
    return AssetImage(source);
  }

  void selectSlide(int index) {
    activeIndex = index;
    notifyListeners();
  }
}

