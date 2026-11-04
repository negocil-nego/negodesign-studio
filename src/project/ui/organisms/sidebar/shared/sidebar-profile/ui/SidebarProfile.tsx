import type { SidebarProfileProps } from "../../../data/types";
export default function SidebarProfile({
  user,
  compact = false,
  dark = false,
}: {
  user: SidebarProfileProps;
  compact?: boolean;
  dark?: boolean;
}) {
  const initials = user.name
    .split(" ")
    .map((x) => x[0])
    .slice(0, 2)
    .join("")
    .toUpperCase();
  return (
    <div
      title={compact ? user.name : undefined}
      className={`flex min-w-0 items-center gap-3 rounded-xl border p-2 ${compact ? "justify-center" : ""} ${dark ? "border-zinc-700 bg-zinc-800/70" : "border-zinc-200 bg-white"}`}
    >
      {user.avatar ? (
        <img
          src={user.avatar}
          alt=""
          className="h-9 w-9 shrink-0 rounded-lg object-cover"
        />
      ) : (
        <span className="grid h-9 w-9 shrink-0 place-items-center rounded-lg bg-blue-100 text-xs font-semibold text-blue-700">
          {initials}
        </span>
      )}
      {!compact && (
        <>
          <div className="min-w-0 flex-1">
            <p
              className={`truncate text-xs font-semibold ${dark ? "text-zinc-100" : "text-zinc-800"}`}
            >
              {user.name}
            </p>
            {user.detail && (
              <p className="truncate text-[10px] text-zinc-500">
                {user.detail}
              </p>
            )}
          </div>
          <button aria-label="Opções do perfil">···</button>
        </>
      )}
    </div>
  );
}
