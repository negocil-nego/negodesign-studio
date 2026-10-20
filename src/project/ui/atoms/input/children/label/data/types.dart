import LabelProps from "../../../../label/data/types.dart";
import InputProps from "../../../core/data/types.dart";

class InputLabelProps {
  final LabelProps labelProps;
  final InputProps inputProps;

  const InputLabelProps({required this.labelProps, required this.inputProps});
}

class InputWithoutLabelProps {
  final LabelProps? labelProps;
  final InputProps inputProps;

  const InputWithoutLabelProps({
    this.labelProps,
    required this.inputProps,
  });
}