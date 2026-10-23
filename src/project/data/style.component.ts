const ButtonDefault = "w-full py-2 px-4 rounded-md text-center"
const CardMediaDefault = "h-56 w-full rounded-md object-cover"

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
        default: "w-10 h-10 rounded-full border flex items-center justify-center cursor-pointer",
    },
    cardMedia: {
        default: CardMediaDefault,
    },
    cardThumbnailVideo: {
        container: "relative cursor-pointer",
        video: "absolute inset-0",
        playButton: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white/80 size-12 rounded-full flex items-center justify-center z-50",
        playIcon: "text-7xl",
    }
}
