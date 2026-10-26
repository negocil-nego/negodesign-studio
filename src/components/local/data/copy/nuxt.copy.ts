import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

// Nuxt uses the Vue implementation from the component catalog.
export function copyNuxtComponent(componentId: string, project: ProjectRecord) {
  return copyComponentForProject(componentId, project, "Vue");
}
