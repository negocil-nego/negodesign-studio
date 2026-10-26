// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import OutlineButton from "../../../../../atoms/button/variant/outline/ui/OutlineButton";
import type { CardActionsButtonProps } from "../data/types";

export default function CardActionsButton({ actions }: CardActionsButtonProps) {
    return (
        <div className={StyleComponent.card.cardActionsButton.root}>
            {actions.map((action, index) => (
                <OutlineButton
                    key={index}
                    onClick={action.onClick}
                    className={StyleComponent.card.cardActionsButton.button}
                    text={action.label}
                    icon={action.icon ? { name: action.icon, className: "text-lg" } : undefined}
                />
            ))}
        </div>
    );
}
