import 'package:flutter/material.dart';
import '../data/types.dart';

class FormFieldConfig extends FormIt { const FormFieldConfig({required String label, String? hint, String type='text', bool required=false}) : super(label: label, placeholder: hint, type: type, required: required); }
class FormConfig extends FormProps { const FormConfig({bool grid=true, List<FormIt> fields=const [], VoidCallback? onSubmit}) : super(style: grid ? 'grid' : 'inline', fields: fields, onSubmit: onSubmit); }
class ProjectForm extends StatelessWidget {
  final FormProps props;
  const ProjectForm(this.props, {super.key});
  @override
  Widget build(BuildContext context) {
    final children = props.fields.map<Widget>((field) {
      final type = field.effectiveType;
      final label = field.effectiveLabel;
      final hint = field.effectiveHint;
      if (type == 'select') return DropdownButtonFormField<String>(decoration: InputDecoration(labelText: label), items: field.options.map((o) => DropdownMenuItem(value: o.value, child: Text(o.label))).toList(), onChanged: field.disabled ? null : (_) {});
      if (type == 'checkbox' || type == 'toggle') return CheckboxListTile(value: field.checked, title: Text(label), onChanged: field.disabled ? null : (_) {});
      if (type == 'radio') return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [Text(label), for (final option in field.options) RadioListTile<String>(value: option.value, groupValue: field.value, title: Text(option.label), onChanged: field.disabled || option.disabled ? null : (_) {})]);
      if (type == 'textarea') return TextFormField(maxLines: field.rows ?? 4, initialValue: field.value, enabled: !field.disabled, decoration: InputDecoration(labelText: label, hintText: hint));
      return TextFormField(initialValue: field.value ?? field.inputProps?.value, obscureText: type.contains('password'), enabled: !field.disabled && !(field.inputProps?.disabled ?? false), keyboardType: type.contains('email') ? TextInputType.emailAddress : type.contains('phone') || type == 'tel' ? TextInputType.phone : null, validator: field.required || (field.inputProps?.required ?? false) ? (value) => value == null || value.isEmpty ? 'Campo obrigatório' : null : null, decoration: InputDecoration(labelText: label, hintText: hint));
    }).toList();
    return Form(key: GlobalKey<FormState>(), onChanged: props.onSubmit, child: Wrap(spacing: 16, runSpacing: 16, children: children.map((child) => SizedBox(width: props.style == 'grid' ? 240 : double.infinity, child: child)).toList()));
  }
}
