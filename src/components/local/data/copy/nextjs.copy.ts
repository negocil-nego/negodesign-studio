import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

// Next.js uses the React implementation from the component catalog.
export function copyNextjsComponent(
  componentId: string,
  project: ProjectRecord,
  locales?: string[],
) {
  return copyComponentForProject(componentId, project, "React", locales);
}
