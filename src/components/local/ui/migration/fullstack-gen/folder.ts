import { open } from "@tauri-apps/plugin-dialog";

export async function pickProjectFolder(
  target: "frontend" | "backend",
): Promise<string | null> {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title:
        target === "frontend"
          ? "Seleccionar pasta do projecto Frontend"
          : "Seleccionar pasta do projecto Backend",
    });
    if (typeof selected === "string" && selected.trim()) return selected;
    return null;
  } catch (err) {
    console.error("Erro ao abrir selector de pastas:", err);
    return null;
  }
}
