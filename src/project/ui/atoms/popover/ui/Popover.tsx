// @ts-nocheck
export default function Popover({ id, trigger, panel, role = "dialog", ariaLabel }: { id: string; trigger: React.ReactNode; panel: React.ReactNode; role?: string; ariaLabel?: string }) {
    return (
        <div className="popover w-full">
            {trigger}
            <div id={id} popover="auto" role={role} aria-label={ariaLabel}>
                {panel}
            </div>
        </div>
    );
}
