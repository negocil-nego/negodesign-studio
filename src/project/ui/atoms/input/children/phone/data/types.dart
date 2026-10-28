import "../../label/data/types.dart" show LabelProps;
import "../../../core/data/types.dart" show InputProps;

typedef InputPhoneProps = InputWithoutLabelProps;

class InputWithoutLabelProps {
  final LabelProps? labelProps;
  final InputProps inputProps;

  const InputWithoutLabelProps({this.labelProps, required this.inputProps});
}
