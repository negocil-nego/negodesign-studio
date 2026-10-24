const ButtonDefault = "w-full py-2 px-4 rounded-md text-center"

export const StyleComponent = {
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
    input: {
        error: "border border-destructive/40 text-destructive",
        success: "border border-success/40 text-success",
        focus: "border border-primary/40 text-primary-foreground",
    },
    iconCenterButton: {
        default: "w-10 h-10 rounded-full border border-gray-100 flex items-center justify-center cursor-pointer bg-gray-100",
    },
    card: {
        description: "text-justify line-clamp-2 xl:line-clamp-3 2xl:line-clamp-4",
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
                root: "w-87.5 p-5 rounded-lg blender-panel",
                header: "flex justify-between",
                headerGroup: "flex gap-2",
                body: "h-full",
                description: "my-4",
                footer: "my-2",
            },
        },
        profile: {
            fill: "h-40 w-full rounded-md object-cover",
            v01: {
                root: "w-72 rounded-3xl blender-panel overflow-hidden shadow-xl",
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
                root: "w-72 rounded-3xl blender-panel overflow-hidden shadow-sm",
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
        }
    }
}
