// @ts-nocheck
import Button from "../../../variant/default/ui/Button";
import type { IconButtonProps } from "../data/types";

export default function IconButton({ icon, isLoading = false, onClick }: IconButtonProps) {
    return (
        <Button
            className={"w-10 h-10 rounded-full border border-gray-100 flex items-center justify-center cursor-pointer bg-gray-100"}
            onClick={onClick}
            isSkeleton={isLoading}
            icon={icon}
        />
    );
}
