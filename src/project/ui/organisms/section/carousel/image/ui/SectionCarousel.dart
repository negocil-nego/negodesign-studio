import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class SectionCarousel extends StatefulWidget {
  final SectionImageCarouselProps props;
  const SectionCarousel(this.props, {super.key});

  @override
  State<SectionCarousel> createState() => _SectionCarouselState();
}

class _SectionCarouselState extends State<SectionCarousel> {
  late final SectionCarouselViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = SectionCarouselViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant SectionCarousel oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(widget.props);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.props.items.isEmpty) return const SizedBox.shrink();
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) {
        final item = _vm.currentItem;
        final image = _vm.currentImageProvider;
        final gradient = _vm.backgroundGradient;
        return ClipRRect(
          borderRadius: BorderRadius.circular(12),
          child: AspectRatio(
            aspectRatio: 4 / 5,
            child: Container(
              width: double.infinity,
              decoration: BoxDecoration(color: _vm.backgroundColor),
              child: Stack(children: [
                if (image != null) Positioned.fill(child: DecoratedBox(decoration: BoxDecoration(image: DecorationImage(image: image, fit: BoxFit.cover)))),
                if (gradient != null) Positioned.fill(child: DecoratedBox(decoration: BoxDecoration(gradient: gradient))),
                const DecoratedBox(
                  decoration: BoxDecoration(gradient: LinearGradient(begin: Alignment.topCenter, end: Alignment.bottomCenter, colors: [Color(0x08000000), Color(0xb8000000)])),
                  child: SizedBox.expand(),
                ),
                Align(
                  alignment: Alignment.bottomCenter,
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(24, 24, 24, 56),
                    child: Column(mainAxisSize: MainAxisSize.min, children: [
                      if (item.title != null) Text(item.title!.Text, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.w600)),
                      if (item.description != null)
                        Padding(padding: const EdgeInsets.only(top: 8), child: Text(item.description!.Text, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70, fontSize: 14, height: 1.5))),
                    ]),
                  ),
                ),
                if (widget.props.items.length > 1)
                  Positioned(
                    bottom: 20,
                    left: 0,
                    right: 0,
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: List.generate(
                        widget.props.items.length,
                        (idx) => Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 4),
                          child: InkWell(
                            customBorder: const CircleBorder(),
                            onTap: () => _vm.selectSlide(idx),
                            child: AnimatedContainer(
                              duration: const Duration(milliseconds: 180),
                              width: _vm.currentIndex == idx ? 20 : 8,
                              height: 8,
                              decoration: BoxDecoration(color: _vm.currentIndex == idx ? Colors.white : Colors.black45, border: Border.all(color: Colors.white70), borderRadius: BorderRadius.circular(99)),
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
              ]),
            ),
          ),
        );
      },
    );
  }
}
