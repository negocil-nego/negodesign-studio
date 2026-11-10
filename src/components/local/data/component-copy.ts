import { get } from "svelte/store";
import {
  activeProject,
  type ProjectLanguage,
  type ProjectRecord,
} from "./projects";
import { copyComponentForProjectFlutter } from "./copy/flutter";
import { copyComponentForProjectNextjs } from "./copy/nextjs";
import { copyComponentForProjectNuxt } from "./copy/nuxt";
import { copyComponentForProjectReact } from "./copy/react";
import { copyComponentForProjectSvelte } from "./copy/svelte";
import { copyComponentForProjectVue } from "./copy/vue";
import type { ComponentCopyOptions, ComponentCopyResult } from "./copy/core";

export type { ComponentCopyOptions } from "./copy/core";

export async function copyComponentToActiveProject(
  componentId: string,
  options: ComponentCopyOptions = {},
): Promise<ComponentCopyResult> {
  const project = get(activeProject);
  if (!project) throw new Error("Seleccione primeiro um projecto activo.");

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
  if (!copy) throw new Error("A linguagem do projecto não foi identificada.");
  return copy(componentId, project, options);
}
