import { get } from "svelte/store";
import { componentCatalog } from "./components";
import {
  activeProject,
  type ProjectLanguage,
  type ProjectRecord,
} from "./projects";
import { copyComponentForProjectFlutter } from "./copy/flutter";
import { copyComponentForProjectNextjs } from "./copy/nextjs";
import { copyComponentForProjectNuxt } from "./copy/nuxt";
import { copyComponentForProjectReact } from "./copy/react";
import { notifyCopyError } from "./copy/shared/notify";
import { copyComponentForProjectSvelte } from "./copy/svelte";
import { copyComponentForProjectVue } from "./copy/vue";
import type { ComponentCopyOptions, ComponentCopyResult } from "./copy/core";

export type { ComponentCopyOptions } from "./copy/core";

export async function copyComponentToActiveProject(
  componentId: string,
  options: ComponentCopyOptions = {},
): Promise<ComponentCopyResult> {
  const componentName =
    componentCatalog.find((entry) => entry.id === componentId)?.name ?? componentId;
  const project = get(activeProject);
  if (!project) {
    const err = new Error("Seleccione primeiro um projecto activo.");
    await notifyCopyError(componentName, err);
    throw err;
  }

  const copyByLanguage: Partial<
    Record<
      ProjectLanguage,
      (
        id: string,
        target: ProjectRecord,
        opts?: ComponentCopyOptions,
      ) => Promise<ComponentCopyResult>
    >
  > = {
    Svelte: copyComponentForProjectSvelte,
    Nextjs: copyComponentForProjectNextjs,
    React: copyComponentForProjectReact,
    Nuxt: copyComponentForProjectNuxt,
    Vue: copyComponentForProjectVue,
    Flutter: copyComponentForProjectFlutter,
  };

  const copy = copyByLanguage[project.language];
  if (!copy) {
    const err = new Error("A linguagem do projecto não foi identificada.");
    await notifyCopyError(componentName, err);
    throw err;
  }
  return copy(componentId, project, options);
}
