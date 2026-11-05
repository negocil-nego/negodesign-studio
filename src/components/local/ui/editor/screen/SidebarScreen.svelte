<script lang="ts">
  import Sidebar from "../../../../../project/ui/organisms/sidebar/ui/Sidebar.svelte";
  import type { SidebarProps } from "../../../../../project/ui/organisms/sidebar/data/types";
  import SourceCode from "../terminal/SourceCode.svelte";
  import Terminal from "../terminal/Terminal.svelte";

  const user = {
    name: "Sienna Hewitt",
    detail: "sienna@untitledui.com",
    avatar: "/img/programmer-01.jpg",
  };
  const enjooyUser = {
    name: "Darlene Robertson",
    detail: "darlene@enjooy.design",
    avatar: "/img/programmer-02.jpg",
  };
  const primary = [{ title: "", items: [
    { label: "Home", icon: "Home", href: "#home", active: true },
    { label: "My store", icon: "Store", href: "#store" },
    { label: "My projects", icon: "Grid", href: "#projects" },
    { label: "Scheduled", icon: "Calendar", href: "#scheduled" },
  ] }];
  const folders = [
    { title: "Folders", items: [
      { label: "View all", icon: "Folder", badge: "48" },
      { label: "Recent", icon: "Clock", badge: "6" },
      { label: "Favorites", icon: "Star", badge: "4" },
      { label: "Shared", icon: "Share", badge: "22" },
      { label: "Archived", icon: "Archive", badge: "14" },
    ] },
    { title: "", items: [
      { label: "All files", icon: "File" },
      { label: "Team members", icon: "User" },
      { label: "Appearance", icon: "Settings" },
    ] },
  ];
  const enjooy = [
    { title: "Overview", items: [
      { label: "Home", icon: "Home", active: true },
      { label: "Inbox", icon: "Mail", badge: "21" },
      { label: "Members", icon: "User", children: [{ label: "All members" }, { label: "Invitations" }] },
      { label: "Settings", icon: "Settings" },
    ] },
    { title: "Private", items: [
      { label: "Credential", icon: "Key" },
      { label: "Meeting Notes", icon: "File" },
      { label: "Product Plans", icon: "Grid" },
    ] },
    { title: "Team Members", items: [
      { label: "Theresa Webb", icon: "User" },
      { label: "Cody Fisher", icon: "User" },
      { label: "Esther Howard", icon: "User" },
    ] },
  ];
  const commerce = [{ title: "", items: [
    { label: "Products", icon: "Grid" },
    { label: "Analytics", icon: "Chart", active: true },
    { label: "Transactions", icon: "Clock" },
    { label: "Orders", icon: "File" },
    { label: "Subscribers", icon: "User" },
    { label: "Payouts", icon: "Wallet" },
    { label: "Posts", icon: "Edit", children: [{ label: "Drafts", badge: "10" }, { label: "Scheduled", badge: "2" }, { label: "Published", badge: "28" }] },
    { label: "Pages", icon: "Layout" },
    { label: "Performance", icon: "Chart" },
    { label: "Team management", icon: "User" },
    { label: "Customize", icon: "Settings" },
  ] }];
  const footer = [{ title: "", items: [
    { label: "Support", icon: "Help" },
    { label: "Settings", icon: "Settings" },
  ] }];

  type SidebarVariant = SidebarProps & { id: string; label: string };
  const variants: SidebarVariant[] = [
    { id: "sidebar-01-full", label: "01 · Navegação completa", variant: "01", sections: primary, footerSections: footer, user },
    { id: "sidebar-01-collapsed", label: "01 · Recolhida", variant: "01", collapsed: true, sections: primary, footerSections: footer, user },
    { id: "sidebar-02-rail", label: "02 · Barra de atalhos", variant: "02", brand: "enjooy", sections: enjooy, user: enjooyUser },
    { id: "sidebar-02-rail-dark", label: "02 · Barra de atalhos · Escuro", variant: "02", theme: "dark", brand: "enjooy", sections: enjooy, user: enjooyUser },
    { id: "sidebar-03-folders", label: "03 · Navegação por pastas · Claro", variant: "03", sections: [...primary, ...folders], footerSections: footer, user },
    { id: "sidebar-03-folders-dark", label: "03 · Navegação por pastas · Escuro", variant: "03", theme: "dark", sections: [...primary, ...folders], footerSections: footer, user },
    { id: "sidebar-04-workspace", label: "04 · Espaço de trabalho · Claro", variant: "04", workspace: "store.untitledui.com", sections: commerce, footerSections: footer, user },
    { id: "sidebar-04-workspace-dark", label: "04 · Espaço de trabalho · Escuro", variant: "04", theme: "dark", workspace: "store.untitledui.com", sections: commerce, footerSections: footer, user },
  ];

  function svelteUsage(item: (typeof variants)[number]) {
    return `<script lang="ts">\n  import Sidebar from "./Sidebar.svelte";\n  const sections = ${JSON.stringify(item.sections, null, 2)};\n<${"/scr" + "ipt"}>\n\n<Sidebar variant="${item.variant}"${item.theme === "dark" ? ' theme="dark"' : ""}${item.collapsed ? " collapsed" : ""} brand="${item.brand ?? "Untitled UI"}" sections={sections} />`;
  }
  function reactUsage(item: (typeof variants)[number]) {
    return `import Sidebar from "./Sidebar";\n\nconst sections = ${JSON.stringify(item.sections, null, 2)};\n\nexport function SidebarExample() {\n  return <Sidebar variant="${item.variant}"${item.theme === "dark" ? ' theme="dark"' : ""}${item.collapsed ? " collapsed" : ""} brand="${item.brand ?? "Untitled UI"}" sections={sections} />;\n}`;
  }
  function vueUsage(item: (typeof variants)[number]) {
    return `<script setup lang="ts">\nimport Sidebar from "./Sidebar.vue";\nconst sections = ${JSON.stringify(item.sections, null, 2)};\n<${"/scr" + "ipt"}>\n\n<template>\n  <Sidebar variant="${item.variant}"${item.theme === "dark" ? ' theme="dark"' : ""}${item.collapsed ? " collapsed" : ""} brand="${item.brand ?? "Untitled UI"}" :sections="sections" />\n<${"/tem" + "plate"}>`;
  }
  function flutterUsage(item: (typeof variants)[number]) {
    return `import 'package:flutter/material.dart';\nimport 'Sidebar.dart';\nimport '../data/types.dart';\n\nfinal sections = <SidebarSectionData>[\n  SidebarSectionData(title: 'Overview', items: [\n    SidebarItemData('Home', icon: Icons.home, active: true),\n    SidebarItemData('Inbox', icon: Icons.mail, badge: '21'),\n  ]),\n];\n\nSidebar(variant: '${item.variant}', theme: '${item.theme ?? "light"}', collapsed: ${item.collapsed ?? false}, brand: '${item.brand ?? "Untitled UI"}', sections: sections)`;
  }
</script>

<div class="space-y-5">
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">Organismos / Sidebar</p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Sidebars</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">Cada variação tem a sua pré-visualização, código para quatro linguagens e opção para copiar o componente para o projecto.</p>
  </div>

  <div class="grid min-w-0 grid-cols-1 gap-4 2xl:grid-cols-2">
    {#each variants as item (item.id)}
      <Terminal id={item.id} name={item.label} description="Navegação lateral responsiva com secções e perfil reutilizáveis.">
        {#snippet preview()}
          <div class="flex min-h-36 justify-center rounded-lg bg-blender-base p-5">
            <Sidebar
              variant={item.variant}
              theme={item.theme ?? "light"}
              collapsed={item.collapsed ?? false}
              brand={item.brand ?? "Untitled UI"}
              workspace={item.workspace ?? "workspace.untitledui.com"}
              sections={item.sections}
              footerSections={item.footerSections ?? []}
              user={item.user}
            />
          </div>
        {/snippet}
        {#snippet svelte()}
          <SourceCode path={`Utilização · ${item.label} · Sidebar.svelte`} source={svelteUsage(item)} />
        {/snippet}
        {#snippet react()}
          <SourceCode path={`Utilização · ${item.label} · Sidebar.tsx`} source={reactUsage(item)} />
        {/snippet}
        {#snippet vue()}
          <SourceCode path={`Utilização · ${item.label} · Sidebar.vue`} source={vueUsage(item)} />
        {/snippet}
        {#snippet flutter()}
          <SourceCode path={`Utilização · ${item.label} · Sidebar.dart`} source={flutterUsage(item)} />
        {/snippet}
      </Terminal>
    {/each}
  </div>
</div>
