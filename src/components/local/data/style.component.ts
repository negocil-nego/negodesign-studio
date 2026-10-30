/* Estilos globais da aplicação, para uso fora de src/project */

/* Superfície base dos painéis (adapta-se ao tema mantendo a paleta) */
const PanelDefault = "bg-white dark:bg-blender-card border border-blender-border transition-colors"

/* Fundo decorativo do workspace com a textura de circuito */
const CircuitBackground = "bg-[linear-gradient(to_bottom,rgba(248,250,252,0.82),rgba(248,250,252,0.92)),url('/img/pawel-d-browski-circuit-7447920.png')] bg-cover bg-center bg-no-repeat dark:bg-[linear-gradient(to_bottom,rgba(24,24,27,0.88),rgba(24,24,27,0.96)),url('/img/pawel-d-browski-circuit-7447920.png')]"

export const StyleComponent = {
    // ── Layout / Shell ─────────────────────────────────────
    layout: {
        app: "h-screen overflow-hidden bg-blender-base text-slate-900 dark:text-zinc-100 font-commissioner select-none",
        workspace: "flex-1 min-h-0 p-3 md:p-6 lg:p-8 bg-blender-base overflow-y-auto flex items-center justify-center",
        sidebar: "bg-blender-sidebar border-r border-blender-border text-zinc-300 select-none",
        topbar: "h-11 px-4 bg-blender-sidebar border-b border-blender-border flex items-center justify-between select-none",
        card: "bg-blender-card text-zinc-200 border border-blender-border rounded-lg shadow-sm overflow-hidden font-commissioner select-none",
        panel: `${PanelDefault} rounded p-3`,
        header: "text-xs font-semibold uppercase tracking-wider text-zinc-500 select-none flex items-center gap-1.5",
        item: "flex items-center gap-2.5 px-2.5 py-1.5 rounded text-sm text-zinc-300 hover:text-slate-900 dark:hover:text-zinc-100 hover:bg-blender-hover active:bg-blender-active transition-colors cursor-pointer select-none",
        action: "flex items-center justify-between p-3 rounded bg-white dark:bg-blender-card hover:bg-slate-50 dark:hover:bg-blender-hover border border-blender-border hover:border-blue-500/50 transition-all text-left dark:text-zinc-300",
        badge: "text-[11px] px-1.5 py-0.5 rounded font-mono shrink-0 bg-slate-100 dark:bg-blender-hover text-zinc-400",
        footer: "px-5 py-2.5 bg-white dark:bg-blender-card border-t border-blender-border flex flex-wrap items-center justify-between gap-3 text-xs text-zinc-500 select-none",
        iconButton: "w-8 h-8 rounded flex items-center justify-center bg-white dark:bg-blender-card hover:bg-blender-hover active:bg-blender-active text-zinc-600 dark:text-zinc-300 hover:text-slate-900 dark:hover:text-zinc-100 transition-colors cursor-pointer border border-blender-border",
        themeToggle: "h-7 w-7 rounded-md border flex items-center justify-center text-sm font-semibold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-1 focus-visible:ring-offset-blender-sidebar",
        themeToggleLight: "border-blue-500/60 bg-blue-500/15 text-blue-600 hover:border-blue-400 hover:bg-blue-500/25 dark:text-blue-300",
        themeToggleDark: "border-amber-400/70 bg-amber-400/20 text-amber-300 shadow-sm shadow-amber-950/30 hover:border-amber-300 hover:bg-amber-400/30",
        circuitBg: CircuitBackground,
    },
    menu: {
        navigation: {
            root: "relative w-full",
            bar: "grid min-h-16 grid-cols-[auto_minmax(0,1fr)_auto] items-center gap-5 border-b border-blender-border bg-blender-card px-5 py-3 md:px-8",
            logo: "h-8 w-8 shrink-0 object-contain",
            logoContainer: "flex min-w-0 items-center",
            list: "flex min-w-0 items-center justify-center gap-2 md:gap-5",
            item: "min-w-0",
            submenuTrigger: "inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100",
            submenuLabel: "truncate",
            submenuIcon: "h-3.5 w-3.5 shrink-0 opacity-70",
            link: "inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100",
            linkIcon: "h-4 w-4 shrink-0",
            linkLabel: "truncate",
            actions: "flex items-center justify-end gap-2 [&>button]:w-auto!",
            panel: "absolute inset-x-0 top-full z-40 overflow-hidden rounded-b-xl border-x border-b border-blender-border bg-blender-card shadow-2xl",
        },
        text: {
            root: "inline-flex min-w-0 items-center gap-2 text-sm font-medium text-zinc-300",
            icon: "h-4 w-4 shrink-0 text-zinc-400",
            label: "truncate [&>div]:inline",
        },
        title: {
            item: {
                root: "group flex min-w-0 flex-col gap-1.5 rounded-lg p-2 transition-colors hover:bg-blender-hover",
                title: "text-sm font-medium text-zinc-200 transition-colors group-hover:text-blue-400",
                titleLink: "text-sm font-medium text-zinc-200 transition-colors hover:text-blue-400",
                description: "text-xs leading-relaxed text-zinc-400",
                link: "inline-flex items-center gap-1 text-xs font-medium text-blue-400",
            },
            featured: {
                root: "flex max-w-md flex-col items-start gap-4",
                title: "text-xl font-semibold leading-snug text-zinc-100 md:text-2xl",
                titleLink: "text-xl font-semibold leading-snug text-zinc-100 transition-colors hover:text-blue-400 md:text-2xl",
                description: "text-sm leading-relaxed text-zinc-400",
                link: "inline-flex items-center gap-1 text-sm font-semibold text-blue-400 transition-colors hover:text-blue-300",
            },
        },
        submenu: {
            gridTitle: {
                root: "@container grid min-w-0 grid-cols-1 divide-y divide-blender-border @lg:grid-cols-[minmax(0,0.9fr)_minmax(0,2.1fr)] @lg:divide-x @lg:divide-y-0",
                intro: "flex min-w-0 flex-col items-start gap-4 p-6 @md:p-8 @lg:pr-10",
                groups: "grid min-w-0 grid-cols-1 gap-6 p-6 @md:p-8 @xl:grid-cols-2 @xl:gap-5",
                group: "min-w-0",
            },
            title: {
                root: "@container flex min-w-0 flex-col gap-3",
                groups: "grid min-w-0 grid-cols-1 @xl:grid-cols-[repeat(2,minmax(0,1fr))]",
            },
            sidebarGrid: {
                root: "@container grid min-h-80 min-w-0 grid-cols-1 overflow-hidden border border-blender-border bg-blender-card md:grid-cols-[minmax(12rem,0.28fr)_minmax(0,0.72fr)]",
                sidebar: "min-w-0 border-b border-blender-border bg-blender-sidebar p-3 md:border-b-0 md:border-r md:p-4",
                sidebarHeader: "mb-3 px-3 text-[10px] font-semibold uppercase tracking-[0.14em] text-zinc-500",
                nav: "flex min-w-0 flex-col gap-1",
                navItem: "w-full min-w-0 rounded-lg px-3 py-2 text-left text-sm text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100",
                navItemActive: "bg-blender-hover font-medium text-zinc-100 shadow-sm",
                content: "min-w-0 p-4 @md:p-6 @lg:p-8",
                contentInner: "mx-auto flex w-full max-w-5xl flex-col gap-6",
                contentHeader: "border-b border-blender-border pb-5",
                groupContent: "min-w-0",
                empty: "flex min-h-48 items-center justify-center rounded-lg border border-dashed border-blender-border text-sm text-zinc-500",
            },
        },
    },
}
