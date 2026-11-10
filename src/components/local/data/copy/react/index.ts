import type { ProjectRecord } from "../../projects";
import { invokeRustComponentCopy } from "../shared/ipc";
import type { ComponentCopyOptions, ComponentCopyResult } from "../shared/types";

export async function copyComponentForProjectReact(
  componentId: string,
  project: ProjectRecord,
  options: ComponentCopyOptions = {},
): Promise<ComponentCopyResult> {
  return invokeRustComponentCopy("copy_component_react", componentId, project, options);
}

export const copyReactComponent = copyComponentForProjectReact;
