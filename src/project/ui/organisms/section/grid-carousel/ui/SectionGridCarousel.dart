import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class SectionGridCarousel extends StatefulWidget {
  final SectionGridCarouselProps props;
  const SectionGridCarousel(this.props, {super.key});

  @override
  State<SectionGridCarousel> createState() => _SectionGridCarouselState();
}

class _SectionGridCarouselState extends State<SectionGridCarousel> {
  late final SectionGridCarouselViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = SectionGridCarouselViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant SectionGridCarousel oldWidget) {
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
    final props = widget.props;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) {
        final pages = _vm.pages;
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: const EdgeInsets.only(bottom: 16),
              child: Row(
                children: [
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        if (props.titleProps != null)
                          Text(props.titleProps!.Text, style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w600)),
                        if (props.descriptionProps != null)
                          Padding(padding: const EdgeInsets.only(top: 8), child: Text(props.descriptionProps!.Text)),
                      ],
                    ),
                  ),
                  IconButton(onPressed: _vm.canGoPrevious ? _vm.previousPage : null, icon: const Icon(Icons.arrow_back)),
                  IconButton(onPressed: _vm.canGoNext ? _vm.nextPage : null, icon: const Icon(Icons.arrow_forward)),
                ],
              ),
            ),
            SizedBox(
              height: 360,
              child: PageView.builder(
                controller: _vm.pageController,
                itemCount: pages.length,
                onPageChanged: _vm.onPageChanged,
                itemBuilder: (context, index) => GridView.count(
                  crossAxisCount: _vm.columns,
                  mainAxisSpacing: 8,
                  crossAxisSpacing: 8,
                  children: pages[index].map((c) => FittedBox(fit: BoxFit.scaleDown, alignment: Alignment.topCenter, child: c)).toList(),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}
