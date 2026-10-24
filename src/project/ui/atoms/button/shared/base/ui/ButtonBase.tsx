// @ts-nocheck
import Icon from "../../../../icon/ui/Icon";
import type { ButtonBaseProps } from "../data/types";

export default function ButtonBase({ icon, text, isLoading, className, isDisabled = false, onClick }: ButtonBaseProps) {
    return (
        <button className={className} onClick={onClick} disabled={isDisabled || isLoading}>
            {icon && <Icon {...icon} />}
            {text && <span>{text}</span>}
        </button>
    );
}
