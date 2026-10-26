// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import Button from "../../../variant/default/ui/Button";
import type { IconButtonProps } from "../data/types";

export default function IconButton({ icon, isLoading = false, className, onClick }: IconButtonProps) {
    return (
        <Button
            className={StyleComponent.iconCenterButton.default}
            onClick={onClick}
            isSkeleton={isLoading}
            icon={icon}
        />
    );
}
