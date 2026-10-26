import { get } from "svelte/store";
import {
  activeProject,
  type ProjectLanguage,
  type ProjectRecord,
} from "./projects";
import { copyFlutterComponent } from "./copy/flutter.copy";
import { copyNextjsComponent } from "./copy/nextjs.copy";
import { copyNuxtComponent } from "./copy/nuxt.copy";
import { copyReactComponent } from "./copy/react.copy";
import { copySvelteComponent } from "./copy/svelte.copy";
import { copyVueComponent } from "./copy/vue.copy";
import type { ComponentCopyResult } from "./copy/core";

export type ComponentCopyOptions = { locales?: string[] };

export async function copyComponentToActiveProject(
  componentId: string,
  options: ComponentCopyOptions = {},
) {
  const project = get(activeProject);
  if (!project) throw new Error("Seleccione primeiro um projecto activo.");

  const copyByLanguage: Partial<
    Record<
      ProjectLanguage,
      (
        id: string,
        target: ProjectRecord,
        locales?: string[],
      ) => Promise<ComponentCopyResult>
    >
  > = {
    Svelte: copySvelteComponent,
    Nextjs: copyNextjsComponent,
    React: copyReactComponent,
    Nuxt: copyNuxtComponent,
    Vue: copyVueComponent,
    Flutter: copyFlutterComponent,
  };
  const copy = copyByLanguage[project.language];
  if (!copy) throw new Error("A linguagem do projecto não foi identificada.");
  return copy(componentId, project, options.locales);
}
