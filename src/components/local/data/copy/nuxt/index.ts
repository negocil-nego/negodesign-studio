import type { ProjectRecord } from "../../projects";
import { invokeRustComponentCopy } from "../shared/ipc";
import type { ComponentCopyOptions, ComponentCopyResult } from "../shared/types";

export async function copyComponentForProjectNuxt(
  componentId: string,
  project: ProjectRecord,
  options: ComponentCopyOptions = {},
): Promise<ComponentCopyResult> {
  return invokeRustComponentCopy("copy_component_nuxt", componentId, project, options);
}

export const copyNuxtComponent = copyComponentForProjectNuxt;
