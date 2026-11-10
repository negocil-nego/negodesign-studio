import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import type { ComponentCopyResult } from "./types";

async function dispatchSystemNotification(title: string, body: string): Promise<void> {
  try {
    let granted = await isPermissionGranted();
    if (!granted) {
      const permission = await requestPermission();
      granted = permission === "granted";
    }
    if (granted) {
      sendNotification({ title, body });
    }
  } catch {
    // Fallback silencioso caso o ambiente não suporte notificações nativas
  }
}

export async function notifyCopySuccess(
  componentName: string,
  result: ComponentCopyResult,
  theme = "default",
): Promise<void> {
  const themeLabel =
    theme === "default"
      ? "Blender Default"
      : theme === "all"
        ? "Todos os temas Bootswatch"
        : `Bootswatch (${theme})`;

  await dispatchSystemNotification(
    "NegoDesign — Cópia concluída com sucesso",
    `O componente "${componentName}" foi copiado para ${result.project.name} (${result.copiedFiles} ficheiros, tema: ${themeLabel}).`,
  );
}

export async function notifyCopyError(
  componentName: string,
  error: unknown,
): Promise<void> {
  const detail =
    error instanceof Error
      ? error.message
      : typeof error === "string"
        ? error
        : "Ocorreu um erro inesperado durante o processo de cópia.";

  await dispatchSystemNotification(
    "NegoDesign — Erro ao copiar componente",
    `Falha ao copiar "${componentName}": ${detail}`,
  );
}

