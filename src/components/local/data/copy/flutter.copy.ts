import type { ProjectRecord } from "../projects";
import { copyComponentForProject } from "./core";

export function copyFlutterComponent(componentId: string, project: ProjectRecord) {
  return copyComponentForProject(componentId, project, "Flutter");
}
