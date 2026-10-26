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
        circuitBg: CircuitBackground,
    },
}
