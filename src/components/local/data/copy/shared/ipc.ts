import { invoke } from "@tauri-apps/api/core";
import { componentCatalog } from "../../components";
import {
  refreshActiveProjectComponents,
  type ComponentInstallResult,
  type ProjectRecord,
} from "../../projects";
import type { ComponentCopyOptions, ComponentCopyResult } from "./types";

export type RustCopyCommand =
  | "copy_component_svelte"
  | "copy_component_react"
  | "copy_component_nextjs"
  | "copy_component_vue"
  | "copy_component_nuxt"
  | "copy_component_flutter";

export type RustComponentCopyRequest = {
  componentId: string;
  manifestPath?: string | null;
  locales?: string[];
  theme?: string;
  staticDirectory?: string;
};

export async function invokeRustComponentCopy(
  command: RustCopyCommand,
  componentId: string,
  project: ProjectRecord,
  options: ComponentCopyOptions = {},
): Promise<ComponentCopyResult> {
  const catalogEntry = componentCatalog.find((entry) => entry.id === componentId);
  const request: RustComponentCopyRequest = {
    componentId,
    manifestPath: catalogEntry?.dependence ?? null,
    locales: options.locales,
    theme: options.theme ?? "default",
    staticDirectory: options.staticDirectory,
  };

  const result = await invoke<ComponentInstallResult>(command, { request });
  await refreshActiveProjectComponents();

  return {
    ...result,
    project,
  };
}

