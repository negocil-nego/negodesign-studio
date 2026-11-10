import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class FormFieldConfig extends FormIt {
  const FormFieldConfig({
    required String label,
    String? hint,
    String type = 'text',
    bool required = false,
  }) : super(label: label, placeholder: hint, type: type, required: required);
}

class FormConfig extends FormProps {
  const FormConfig({
    bool grid = true,
    List<FormIt> fields = const [],
    VoidCallback? onSubmit,
  }) : super(style: grid ? 'grid' : 'inline', fields: fields, onSubmit: onSubmit);
}

class ProjectForm extends StatelessWidget {
  final FormProps props;
  final FormViewModel _vm = const FormViewModel();

  const ProjectForm(this.props, {super.key});

  Widget _buildField(FormIt field) {
    final type = field.effectiveType;
    final label = field.effectiveLabel;
    final hint = field.effectiveHint;
    if (type == 'select') {
      return DropdownButtonFormField<String>(
        decoration: InputDecoration(labelText: label),
        items: field.options
            .map((o) => DropdownMenuItem(value: o.value, child: Text(o.label)))
            .toList(),
        onChanged: field.disabled ? null : (_) {},
      );
    }
    if (type == 'checkbox' || type == 'toggle') {
      return CheckboxListTile(
        value: field.checked,
        title: Text(label),
        onChanged: field.disabled ? null : (_) {},
      );
    }
    if (type == 'radio') {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label),
          for (final option in field.options)
            RadioListTile<String>(
              value: option.value,
              groupValue: field.value,
              title: Text(option.label),
              onChanged: field.disabled || option.disabled ? null : (_) {},
            ),
        ],
      );
    }
    if (type == 'textarea') {
      return TextFormField(
        maxLines: field.rows ?? 4,
        initialValue: field.value,
        enabled: !field.disabled,
        decoration: InputDecoration(labelText: label, hintText: hint),
      );
    }
    return TextFormField(
      initialValue: field.value ?? field.inputProps?.value,
      obscureText: type.contains('password'),
      enabled: !_vm.isDisabled(field),
      keyboardType: _vm.keyboardTypeFor(type),
      validator: _vm.validatorFor(field),
      decoration: InputDecoration(labelText: label, hintText: hint),
    );
  }

  @override
  Widget build(BuildContext context) {
    final itemWidth = props.style == 'grid' ? 240.0 : double.infinity;
    return Form(
      key: GlobalKey<FormState>(),
      onChanged: props.onSubmit,
      child: Wrap(
        spacing: 16,
        runSpacing: 16,
        children: props.fields
            .map((field) => SizedBox(width: itemWidth, child: _buildField(field)))
            .toList(),
      ),
    );
  }
}
