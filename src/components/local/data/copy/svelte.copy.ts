import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

export function copySvelteComponent(componentId: string, project: ProjectRecord) {
  return copyComponentForProject(componentId, project, "Svelte");
}
