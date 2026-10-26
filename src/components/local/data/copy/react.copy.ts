import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

export function copyReactComponent(
  componentId: string,
  project: ProjectRecord,
  locales?: string[],
) {
  return copyComponentForProject(componentId, project, "React", locales);
}
