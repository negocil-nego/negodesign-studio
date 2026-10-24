// @ts-nocheck
import OutlineButton from "../../../../../atoms/button/variant/outline/ui/OutlineButton";
import type { CardActionsButtonProps } from "../data/types";

export default function CardActionsButton({ actions }: CardActionsButtonProps) {
    return (
        <div className="flex gap-3 w-full">
            {actions.map((action, index) => (
                <OutlineButton
                    key={index}
                    onClick={action.onClick}
                    className="flex-1 justify-center py-2.5 rounded-2xl border-gray-200 text-gray-900 font-semibold flex items-center gap-2"
                    text={action.label}
                    icon={action.icon ? { name: action.icon, className: "text-lg" } : undefined}
                />
            ))}
        </div>
    );
}
