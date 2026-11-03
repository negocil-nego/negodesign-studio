import 'package:flutter/material.dart';

import '../data/types.dart';

class SectionGridCarousel extends StatefulWidget {
  final SectionGridCarouselProps props;

  const SectionGridCarousel(this.props, {super.key});

  @override
  State<SectionGridCarousel> createState() => _SectionGridCarouselState();
}

class _SectionGridCarouselState extends State<SectionGridCarousel> {
  final PageController _pages = PageController();
  int _page = 0;

  @override
  void dispose() {
    _pages.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final props = widget.props;
    final columns = props.columns.clamp(1, 1000).toInt();
    final rows = props.rows.clamp(1, 1000).toInt();
    final pageSize = rows * columns;
    final pages = <List<Widget>>[];

    for (var index = 0; index < props.children.length; index += pageSize) {
      pages.add(props.children.skip(index).take(pageSize).toList());
    }

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
                      Text(
                        props.titleProps!.Text,
                        style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w600),
                      ),
                    if (props.descriptionProps != null)
                      Padding(
                        padding: const EdgeInsets.only(top: 8),
                        child: Text(props.descriptionProps!.Text),
                      ),
                  ],
                ),
              ),
              IconButton(
                onPressed: _page > 0
                    ? () => _pages.previousPage(
                          duration: const Duration(milliseconds: 300),
                          curve: Curves.easeInOut,
                        )
                    : null,
                icon: const Icon(Icons.arrow_back),
              ),
              IconButton(
                onPressed: _page < pages.length - 1
                    ? () => _pages.nextPage(
                          duration: const Duration(milliseconds: 300),
                          curve: Curves.easeInOut,
                        )
                    : null,
                icon: const Icon(Icons.arrow_forward),
              ),
            ],
          ),
        ),
        SizedBox(
          height: 360,
          child: PageView.builder(
            controller: _pages,
            itemCount: pages.length,
            onPageChanged: (page) => setState(() => _page = page),
            itemBuilder: (context, index) => GridView.count(
              crossAxisCount: columns,
              mainAxisSpacing: 8,
              crossAxisSpacing: 8,
              children: pages[index]
                  .map(
                    (child) => FittedBox(
                      fit: BoxFit.scaleDown,
                      alignment: Alignment.topCenter,
                      child: child,
                    ),
                  )
                  .toList(),
            ),
          ),
        ),
      ],
    );
  }
}
