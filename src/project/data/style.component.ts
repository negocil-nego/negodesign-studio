const ButtonDefault = "w-full py-2 px-4 rounded-md text-center"

const InputForm = "flex w-full min-w-0 items-center gap-2 rounded-md border border-blender-border bg-blender-surface px-3 py-2 text-sm text-zinc-200 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 [&>input]:w-full [&>input]:min-w-0 [&>input]:border-0 [&>input]:bg-transparent [&>input]:p-0 [&>input]:text-inherit [&>input]:outline-none [&>input::placeholder]:text-zinc-500 [&>input:disabled]:cursor-not-allowed [&>input:disabled]:opacity-50"

export const StyleComponent = {
    avatar: {
        default: "w-12 h-12 rounded-lg",
    },
    button: {
        solid: {
            default: `${ButtonDefault} bg-blue-900 hover:bg-primary/80 text-white`,
            danger: `${ButtonDefault} bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:border-destructive/40 focus-visible:ring-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40`,
            success: `${ButtonDefault} bg-success/10 text-success hover:bg-success/20 focus-visible:border-success/40 focus-visible:ring-success/20 dark:bg-success/20 dark:hover:bg-success/30 dark:focus-visible:ring-success/40`,
            warning: `${ButtonDefault} bg-warning/10 text-warning hover:bg-warning/20 focus-visible:border-warning/40 focus-visible:ring-warning/20 dark:bg-warning/20 dark:hover:bg-warning/30 dark:focus-visible:ring-warning/40`,
        },
        outline: {
            default: `${ButtonDefault} border border-primary/40 text-primary-foreground`,
            danger: `${ButtonDefault} bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:border-destructive/40 focus-visible:ring-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40`,
            success: `${ButtonDefault} bg-success/10 text-success hover:bg-success/20 focus-visible:border-success/40 focus-visible:ring-success/20 dark:bg-success/20 dark:hover:bg-success/30 dark:focus-visible:ring-success/40`,
            warning: `${ButtonDefault} bg-warning/10 text-warning hover:bg-warning/20 focus-visible:border-warning/40 focus-visible:ring-warning/20 dark:bg-warning/20 dark:hover:bg-warning/30 dark:focus-visible:ring-warning/40`,
        }
    },
    iconCenterButton: {
        default: "w-10 h-10 rounded-full border border-gray-100 flex items-center justify-center cursor-pointer bg-gray-100",
    },
    checkbox: {
        root: "checkbox-root",
        disabled: "checkbox-disabled",
        input: "checkbox-input",
        box: "checkbox-box",
        tick: "checkbox-tick",
        label: "checkbox-label",
    },
    icon: {
        spinner: "animate-spin inline-block w-5 h-5",
        default: "text-base opacity-75",
    },
    input: {
        form: InputForm,
        error: "border border-destructive/40 text-destructive",
        success: "border border-success/40 text-success",
        focus: "border border-primary/40 text-primary-foreground",
        text: {
            root: InputForm,
        },
        email: {
            root: InputForm,
        },
        password: {
            root: InputForm,
        },
        number: {
            root: InputForm,
        },
        time: {
            root: InputForm,
        },
        phone: {
            field: "input-phone-field",
            form: `${InputForm} input-phone-flag`,
        },
        date: {
            root: "relative w-full",
            trigger: `${InputForm} w-full cursor-pointer flex items-center justify-between text-left`,
            triggerBody: "flex flex-col w-full min-w-0",
            triggerLabel: "text-[10px] uppercase font-semibold text-zinc-500 select-none",
            valueRow: "flex items-center gap-2 mt-0.5",
            valueIcon: "text-sm",
            value: "text-sm font-medium truncate",
            expandIcon: "flex flex-col text-zinc-400 text-xs ml-2 select-none shrink-0",
            panel: "mt-1.5 text-zinc-200 z-50 outline-none",
            panelHeader: "flex items-center justify-between mb-4",
            navButton: "p-1.5 rounded hover:bg-blender-hover cursor-pointer",
            monthTitle: "font-bold text-sm text-zinc-200",
            weekdays: "grid grid-cols-7 text-center text-xs font-medium text-zinc-500 mb-3",
            daysGrid: "grid grid-cols-7 gap-y-1.5 text-center text-sm font-medium",
            paddingDay: "text-zinc-600 py-1 opacity-40",
            day: "w-8 h-8 mx-auto flex items-center justify-center rounded-full transition-all cursor-pointer",
            daySelected: "bg-orange-500 text-white font-bold shadow-md",
            dayDefault: "text-zinc-300 hover:bg-blender-hover",
        },
    },
    popover: {
        root: "popover w-full",
    },
    radio: {
        group: "radio-group",
        item: "radio-item",
        disabled: "radio-disabled",
        input: "radio-input",
        dot: "radio-dot",
        label: "radio-label",
    },
    select: {
        root: "relative w-full",
        trigger: "w-full flex justify-center items-center text-left",
        triggerLabel: "text-[10px] uppercase font-semibold text-zinc-500 select-none w-full",
        triggerValue: "text-sm truncate w-full",
        panel: "mt-1.5 p-2 w-full min-w-60 shadow-lg z-50",
        search: "w-full px-3 py-1.5 mb-1.5 text-sm bg-blender-surface border border-blender-border rounded text-inherit outline-none focus:border-blue-500",
        list: "max-h-56 overflow-y-auto flex flex-col gap-0.5",
        option: "w-full flex items-center justify-between px-3 py-2 rounded text-sm text-zinc-300 hover:text-slate-900 hover:bg-blender-hover active:bg-blender-active transition-colors cursor-pointer text-left",
        optionSelected: "bg-blender-hover text-slate-900 font-medium",
        optionContent: "flex items-center gap-2.5 truncate",
        optionIcon: "text-base",
        optionEmpty: "px-3 py-2 text-xs text-zinc-500 text-center select-none",
        optionCheck: "text-blue-500 font-bold text-xs",
    },
    skeleton: {
        default: "relative overflow-hidden rounded-xl bg-gray-200 before:absolute before:inset-0 before:-translate-x-full before:animate-shimmer before:bg-linear-to-r before:from-transparent before:via-white/70 before:to-transparent",
    },
    textarea: {
        form: InputForm,
    },
    toggle: {
        root: "toggle-root",
        track: "toggle-track",
        on: "toggle-on",
        thumb: "toggle-thumb",
        label: "toggle-label",
    },

    // ── Molecules ──────────────────────────────────────────
    card: {
        root: "card",
        description: "text-justify line-clamp-2 xl:line-clamp-3 2xl:line-clamp-4",
        title: {
            root: "card-title",
            text: "card-title-text",
        },
        price: {
            root: "card-price",
            number: "card-price-number",
        },
        phone: {
            root: "card-phone",
            number: "card-phone-number",
        },
        rating: {
            root: "card-rating flex gap-2",
            filled: "text-yellow-600",
        },
        tags: {
            root: "flex gap-4 w-full border-y border-gray-100 py-4",
            item: "flex flex-col items-center justify-center flex-1 first:border-l-0 border-l border-gray-100",
            icon: "mb-1",
            value: "text-lg font-bold text-gray-900",
            label: "text-sm text-gray-500",
        },
        cardThumbnailVideo: {
            container: "relative cursor-pointer",
            video: "absolute inset-0",
            playButton: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white/80 size-12 rounded-full flex items-center justify-center z-50",
            playIcon: "text-7xl",
        },
        media: {
            default: "h-50 w-full rounded-md object-cover",
            frame: "overflow-hidden rounded-md border border-gray-100",
            fill: "h-full w-full",
            v01: {
                root: "w-87.5 p-5 rounded-lg bg-white dark:bg-blender-card border border-blender-border transition-colors",
                header: "flex justify-between",
                headerGroup: "flex gap-2",
                body: "h-full",
                description: "my-4",
                footer: "my-2",
            },
            v02: {
                root: "w-87.5 flex flex-col gap-4 p-6 rounded-2xl bg-white dark:bg-blender-card border border-blender-border shadow-sm transition-colors",
                header: "flex items-start justify-between gap-3",
                headerGroup: "flex items-center gap-3",
                titleGroup: "flex flex-col gap-1",
                actions: "flex flex-row gap-2 shrink-0",
                body: "flex flex-col gap-3",
                description: "my-1",
                footer: "mt-1",
            },
        },
        profile: {
            fill: "h-40 w-full rounded-md object-cover",
            v01: {
                root: "w-72 rounded-3xl bg-white dark:bg-blender-card border border-blender-border transition-colors overflow-hidden shadow-xl",
                cover: "relative h-32 w-full",
                actions: "absolute top-2 flex justify-between w-full gap-2 z-10 p-2",
                logo: "absolute left-1/2 -translate-x-1/2 -translate-y-1/2 z-10",
                logoFrame: "rounded-full overflow-hidden border-4 border-white bg-white shadow-md",
                logoImage: "w-20 h-20 rounded-full",
                body: "px-6 pt-14 pb-6 flex flex-col gap-4",
                titleGroup: "flex flex-col items-center gap-1 z-20 border-white text-white bg-black/5 p-2 font-bold",
                descriptionGroup: "flex flex-col gap-3",
                footer: "mt-2",
            },
            v02: {
                root: "w-72 rounded-3xl bg-white dark:bg-blender-card border border-blender-border transition-colors overflow-hidden shadow-sm",
                cover: "h-44 w-full relative",
                body: "px-6 pb-6 flex flex-col gap-4",
                actions: "absolute top-2 flex justify-end w-full gap-2 z-10 p-2",
                logoFrame: "w-24 h-24 rounded-full border-4 border-white overflow-hidden bg-white shadow-md absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-10",
                logoImage: "w-full h-full object-cover",
                infoTitle: "text-2xl font-bold text-gray-900 mt-8",
                infoSubtitle: "text-sm text-gray-500 font-medium",
                statValue: "text-lg font-bold text-gray-900",
                statLabel: "text-sm text-gray-500",
            },
        },
        cardActionsButton: {
            root: "flex gap-3 w-full",
            button: "flex-1 justify-center py-2.5 rounded-2xl border-gray-200 text-gray-900 font-semibold flex items-center gap-2",
        },
        cardMinFile: {
            root: "relative shrink-0 size-14 overflow-hidden rounded-md border border-blender-border cursor-pointer transition-colors [&>img]:h-full [&>img]:w-full [&>img]:rounded-none [&>img]:object-cover [&>img]:transition-none",
            active: "border-blue-500 ring-1 ring-blue-500",
        },
        product: {
            frame: "overflow-hidden rounded-md border border-gray-100",
            fill: "h-full w-full",
            v01: {
                root: "w-72 rounded-2xl bg-white dark:bg-blender-card border border-blender-border overflow-hidden transition-colors",
                media: "relative h-56 w-full",
                actions: "absolute top-2 inset-x-0 flex justify-between gap-2 z-10 p-2",
                priceBadge: "absolute bottom-2 left-2 z-10 rounded-full bg-white/90 px-3 py-1 backdrop-blur",
                body: "flex flex-col gap-3 p-5",
                header: "flex items-center gap-3",
                titleGroup: "flex flex-col gap-1 min-w-0",
                thumbs: "flex gap-2 overflow-x-auto",
                footer: "mt-1",
            },
            v02: {
                root: "w-100 flex gap-4 p-5 rounded-2xl bg-white dark:bg-blender-card border border-blender-border shadow-sm transition-colors",
                gallery: "flex flex-col gap-2 w-44 shrink-0",
                media: "relative h-40 w-full",
                actions: "absolute top-2 inset-x-0 flex justify-between gap-2 z-10 p-2",
                thumbs: "flex gap-2 overflow-x-auto",
                body: "flex flex-col gap-3 flex-1 min-w-0",
                header: "flex items-center gap-3",
                titleGroup: "flex flex-col gap-1 min-w-0",
                footer: "mt-auto",
            },
        }
    }
}
