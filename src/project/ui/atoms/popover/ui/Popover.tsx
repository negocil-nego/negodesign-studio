// @ts-nocheck
import { StyleComponent } from "../../../../data/style.component";

export default function Popover({ id, trigger, panel, role = "dialog", ariaLabel }: { id: string; trigger: React.ReactNode; panel: React.ReactNode; role?: string; ariaLabel?: string }) {
    return (
        <div className={StyleComponent.popover.root}>
            {trigger}
            <div id={id} popover="auto" role={role} aria-label={ariaLabel}>
                {panel}
            </div>
        </div>
    );
}
