import 'package:flutter/material.dart';
import '../data/types.dart';

class SectionCarouselButtons extends StatefulWidget {
  final SectionCarouselButtonsProps props;
  const SectionCarouselButtons(this.props, {super.key});

  @override
  State<SectionCarouselButtons> createState() => _SectionCarouselButtonsState();
}

class _SectionCarouselButtonsState extends State<SectionCarouselButtons> {
  final ScrollController _controller = ScrollController();

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _scroll(int direction) {
    if (!_controller.hasClients) return;
    _controller.animateTo(
      (_controller.offset + direction * _controller.position.viewportDimension * 0.8)
          .clamp(0, _controller.position.maxScrollExtent)
          .toDouble(),
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  @override
  Widget build(BuildContext context) {
    final props = widget.props;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.only(bottom: 24),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    if (props.titleProps != null)
                      Text(props.titleProps!.Text, style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w600)),
                    if (props.descriptionProps != null)
                      Padding(
                        padding: const EdgeInsets.only(top: 8),
                        child: Text(props.descriptionProps!.Text, style: const TextStyle(color: Colors.grey)),
                      ),
                  ],
                ),
              ),
              IconButton.filledTonal(onPressed: () => _scroll(-1), tooltip: 'Itens anteriores', icon: const Icon(Icons.arrow_back)),
              const SizedBox(width: 8),
              IconButton.filledTonal(onPressed: () => _scroll(1), tooltip: 'Itens seguintes', icon: const Icon(Icons.arrow_forward)),
            ],
          ),
        ),
        SizedBox(
          height: 360,
          child: ListView.separated(
            controller: _controller,
            scrollDirection: Axis.horizontal,
            itemCount: props.children.length,
            separatorBuilder: (_, __) => const SizedBox(width: 20),
            itemBuilder: (_, index) => SizedBox(width: 272, child: props.children[index]),
          ),
        ),
      ],
    );
  }
}
