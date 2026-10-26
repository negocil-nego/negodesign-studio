import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

export function copySvelteComponent(
  componentId: string,
  project: ProjectRecord,
  locales?: string[],
) {
  return copyComponentForProject(componentId, project, "Svelte", locales);
}
