import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

export type ProjectLanguage = "Svelte" | "React" | "Vue" | "Flutter" | "Unknown";

export type ProjectRecord = {
  id: number;
  name: string;
  path: string;
  language: ProjectLanguage;
  active: boolean;
};

export const activeProject = writable<ProjectRecord | null>(null);

export function listProjects(): Promise<ProjectRecord[]> {
  return invoke<ProjectRecord[]>("list_projects");
}

export function saveProject(path: string): Promise<ProjectRecord> {
  return invoke<ProjectRecord>("save_project", { path });
}

export function setActiveProject(id: number): Promise<ProjectRecord> {
  return invoke<ProjectRecord>("set_active_project", { id });
}

export type ProjectSourceFile = { path: string; content: string };
export type ComponentExport = { id: string; name: string; entry: string };
export type ComponentInstallResult = { copiedFiles: number; exportedComponents: number };

export function installComponent(
  files: ProjectSourceFile[],
  components: ComponentExport[],
): Promise<ComponentInstallResult> {
  return invoke<ComponentInstallResult>("install_component", { files, components });
}
