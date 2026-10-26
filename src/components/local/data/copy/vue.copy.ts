import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

export function copyVueComponent(componentId: string, project: ProjectRecord) {
  return copyComponentForProject(componentId, project, "Vue");
}
