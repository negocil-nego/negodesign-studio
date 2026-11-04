<script lang="ts">
  import Sidebar from "../../../../../project/ui/organisms/sidebar/ui/Sidebar.svelte";
  import sidebarSource from "../../../../../project/ui/organisms/sidebar/ui/Sidebar.svelte?raw";
  import reactSource from "../../../../../project/ui/organisms/sidebar/ui/Sidebar.tsx?raw";
  import vueSource from "../../../../../project/ui/organisms/sidebar/ui/Sidebar.vue?raw";
  import flutterSource from "../../../../../project/ui/organisms/sidebar/ui/Sidebar.dart?raw";
  import SourceCode from "../terminal/SourceCode.svelte";
  import Terminal from "../terminal/Terminal.svelte";

  const user = {
    name: "Sienna Hewitt",
    detail: "sienna@untitledui.com",
    avatar: "/img/programmer-01.jpg",
  };

  const primary = [
    {
      title: "",
      items: [
        { label: "Home", icon: "Home", href: "#home", active: true },
        { label: "My store", icon: "Store", href: "#store" },
        { label: "My projects", icon: "Grid", href: "#projects" },
        { label: "Scheduled", icon: "Calendar", href: "#scheduled" },
      ],
    },
  ];

  const folders = [
    {
      title: "Folders",
      items: [
        { label: "View all", icon: "Folder", badge: "48" },
        { label: "Recent", icon: "Clock", badge: "6" },
        { label: "Favorites", icon: "Star", badge: "4" },
        { label: "Shared", icon: "Share", badge: "22" },
        { label: "Archived", icon: "Archive", badge: "14" },
      ],
    },
  ];

  const tools = [
    {
      title: "",
      items: [
        { label: "All files", icon: "File" },
        { label: "Team members", icon: "User" },
        { label: "Appearance", icon: "Settings" },
      ],
    },
  ];

  const footer = [
    {
      title: "",
      items: [
        { label: "Support", icon: "Help" },
        { label: "Settings", icon: "Settings" },
      ],
    },
  ];

  const enjooy = [
    {
      title: "Overview",
      items: [
        { label: "Home", icon: "Home", active: true },
        { label: "Inbox", icon: "Mail", badge: "21" },
        {
          label: "Members",
          icon: "User",
          children: [{ label: "All members" }, { label: "Invitations" }],
        },
        { label: "Settings", icon: "Settings" },
      ],
    },
    {
      title: "Private",
      items: [
        { label: "Credential", icon: "Key" },
        { label: "Meeting Notes", icon: "File" },
        { label: "Product Plans", icon: "Grid" },
      ],
    },
    {
      title: "Team Members",
      items: [
        { label: "Theresa Webb", icon: "User" },
        { label: "Cody Fisher", icon: "User" },
        { label: "Esther Howard", icon: "User" },
      ],
    },
  ];

  const commerce = [
    {
      title: "",
      items: [
        { label: "Products", icon: "Grid" },
        { label: "Analytics", icon: "Chart", active: true },
        { label: "Transactions", icon: "Clock" },
        { label: "Orders", icon: "File" },
        { label: "Subscribers", icon: "User" },
        { label: "Payouts", icon: "Wallet" },
        {
          label: "Posts",
          icon: "Edit",
          children: [
            { label: "Drafts", badge: "10" },
            { label: "Scheduled", badge: "2" },
            { label: "Published", badge: "28" },
          ],
        },
        { label: "Pages", icon: "Layout" },
        { label: "Performance", icon: "Chart" },
        { label: "Team management", icon: "User" },
        { label: "Customize", icon: "Settings" },
      ],
    },
  ];

  const enjooyUser = {
    name: "Darlene Robertson",
    detail: "darlene@enjooy.design",
    avatar: "/img/programmer-02.jpg",
  };
</script>

<div class="space-y-5">
  <div>
    <p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-blue-400">
      Organismos / Sidebar
    </p>
    <h2 class="mt-1 text-lg font-semibold text-zinc-100">Sidebars</h2>
    <p class="mt-2 text-xs leading-5 text-zinc-400">
      Navegação lateral composta por secções, itens e perfil partilhados. Cada variação suporta
      tema claro ou escuro; a primeira também permite recolher os rótulos.
    </p>
  </div>

  <Terminal
    id="sidebar"
    name="Sidebar"
    description="Organismo de navegação lateral com quatro composições."
  >
    {#snippet preview()}
      <div class="space-y-4">
        <div class="grid gap-4 xl:grid-cols-2">
          <div class="flex min-w-0 justify-center rounded-xl bg-zinc-100 p-4">
            <Sidebar
              variant="01"
              brand="Untitled UI"
              sections={primary}
              footerSections={footer}
              user={user}
            />
          </div>
          <div class="flex min-w-0 justify-center rounded-xl bg-zinc-100 p-4">
            <Sidebar
              variant="01"
              collapsed
              brand="Untitled UI"
              sections={primary}
              footerSections={footer}
              user={user}
            />
          </div>
          <div class="flex min-w-0 justify-center rounded-xl bg-zinc-100 p-4">
            <Sidebar variant="02" brand="enjooy" sections={enjooy} user={enjooyUser} />
          </div>
          <div class="flex min-w-0 justify-center rounded-xl bg-zinc-100 p-4">
            <Sidebar
              variant="03"
              brand="Untitled UI"
              sections={[...primary, ...folders, ...tools]}
              footerSections={footer}
              user={user}
            />
          </div>
          <div class="flex min-w-0 justify-center rounded-xl bg-zinc-100 p-4">
            <Sidebar
              variant="03"
              theme="dark"
              brand="Untitled UI"
              sections={[...primary, ...folders, ...tools]}
              footerSections={footer}
              user={user}
            />
          </div>
          <div class="flex min-w-0 justify-center rounded-xl bg-zinc-100 p-4">
            <Sidebar
              variant="04"
              brand="Untitled UI"
              workspace="store.untitledui.com"
              sections={commerce}
              footerSections={footer}
              user={user}
            />
          </div>
          <div class="flex min-w-0 justify-center rounded-xl bg-zinc-100 p-4">
            <Sidebar
              variant="04"
              theme="dark"
              brand="Untitled UI"
              workspace="store.untitledui.com"
              sections={commerce}
              footerSections={footer}
              user={user}
            />
          </div>
        </div>

        <div class="flex justify-center rounded-xl bg-zinc-100 p-4">
          <Sidebar variant="02" theme="dark" brand="enjooy" sections={enjooy} user={enjooyUser} />
        </div>
      </div>
    {/snippet}

    {#snippet svelte()}
      <SourceCode
        path="src/project/ui/organisms/sidebar/ui/Sidebar.svelte"
        source={sidebarSource}
      />
    {/snippet}
    {#snippet react()}
      <SourceCode
        path="src/project/ui/organisms/sidebar/ui/Sidebar.tsx"
        source={reactSource}
      />
    {/snippet}
    {#snippet vue()}
      <SourceCode
        path="src/project/ui/organisms/sidebar/ui/Sidebar.vue"
        source={vueSource}
      />
    {/snippet}
    {#snippet flutter()}
      <SourceCode
        path="src/project/ui/organisms/sidebar/ui/Sidebar.dart"
        source={flutterSource}
      />
    {/snippet}
  </Terminal>
</div>
