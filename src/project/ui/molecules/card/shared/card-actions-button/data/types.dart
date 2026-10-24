class CardActionButton {
  final String? icon;
  final String label;
  final void Function()? onClick;

  const CardActionButton({this.icon, required this.label, this.onClick});
}

class CardActionsButtonProps {
  final List<CardActionButton> actions;

  const CardActionsButtonProps({required this.actions});
}
