import type { ProjectRecord } from "../../projects";
import { invokeRustComponentCopy } from "../shared/ipc";
import type { ComponentCopyOptions, ComponentCopyResult } from "../shared/types";

export async function copyComponentForProjectVue(
  componentId: string,
  project: ProjectRecord,
  options: ComponentCopyOptions = {},
): Promise<ComponentCopyResult> {
  return invokeRustComponentCopy("copy_component_vue", componentId, project, options);
}

export const copyVueComponent = copyComponentForProjectVue;
