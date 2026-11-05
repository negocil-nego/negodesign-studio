import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

/** Subpasta de `static/` onde os assets dos componentes são instalados em projectos Svelte. */
const SVELTE_STATIC_DIRECTORY = "nego";

export function copySvelteComponent(
  componentId: string,
  project: ProjectRecord,
  locales?: string[],
) {
  return copyComponentForProject(componentId, project, "Svelte", locales, {
    staticDirectory: SVELTE_STATIC_DIRECTORY,
  });
}
