import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class Radio extends StatefulWidget {
  final RadioProps props;
  const Radio(this.props, {super.key});

  @override
  State<Radio> createState() => _RadioState();
}

class _RadioState extends State<Radio> {
  late final RadioViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = RadioViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Radio oldWidget) {
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
    final primary = Theme.of(context).colorScheme.primary;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: widget.props.options.map((option) {
          final isDisabled = option.disabled || widget.props.disabled;
          final isSelected = _vm.selected == option.value;
          return Padding(
            padding: const EdgeInsets.symmetric(vertical: 4),
            child: InkWell(
              onTap: isDisabled ? null : () => _vm.select(option.value),
              borderRadius: BorderRadius.circular(NegoThemeData.radiusSm),
              child: Opacity(
                opacity: isDisabled ? 0.5 : 1.0,
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    AnimatedContainer(
                      duration: const Duration(milliseconds: 150),
                      width: 16,
                      height: 16,
                      alignment: Alignment.center,
                      decoration: BoxDecoration(
                        shape: BoxShape.circle,
                        color: isSelected ? primary : NegoThemeData.blenderSurface,
                        border: Border.all(
                          color: isSelected ? primary : NegoThemeData.blenderBorder,
                        ),
                      ),
                      child: isSelected
                          ? Container(
                              width: 6,
                              height: 6,
                              decoration: const BoxDecoration(
                                shape: BoxShape.circle,
                                color: Colors.white,
                              ),
                            )
                          : null,
                    ),
                    const SizedBox(width: 10),
                    Text(
                      option.label,
                      style: const TextStyle(fontSize: 13, color: NegoThemeData.textPrimary),
                    ),
                  ],
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }
}
