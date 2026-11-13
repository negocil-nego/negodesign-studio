import { invoke } from "@tauri-apps/api/core";

export type CodegenPath = {
  id: number;
  target: string;
  path: string;
  label: string | null;
  lastUsed: string;
};

export type SpringBootProjectResult = {
  projectDir: string;
  buildTool: string;
  springBootVersion: string;
  generatedFiles: number;
  dependencies: string[];
  nextSteps: string[];
};

export type SpringBootProjectInput = {
  targetDir: string;
  groupId: string;
  artifactId: string;
  name: string;
  description: string;
  language: "java" | "kotlin";
  ormStrategy: string;
  databaseEngine: "postgres" | "mysql" | "sqlite";
  includeApiDocs: boolean;
};

export type BackendProjectResult = {
  projectDir: string;
  buildTool: string;
  generatedFiles: number;
  dependencies: string[];
  nextSteps: string[];
};

export type GenericBackendInput = {
  targetDir: string;
  name: string;
  description: string;
  ormStrategy: string;
  databaseEngine: "postgres" | "mysql" | "sqlite";
  includeApiDocs: boolean;
};

export async function listCodegenPaths(
  target: "frontend" | "backend",
): Promise<CodegenPath[]> {
  try {
    return await invoke<CodegenPath[]>("list_codegen_paths", { target });
  } catch (err) {
    console.error(`Erro ao listar paths de ${target}:`, err);
    return [];
  }
}

export async function saveCodegenPath(
  target: "frontend" | "backend",
  path: string,
  label?: string,
): Promise<CodegenPath | null> {
  try {
    return await invoke<CodegenPath>("save_codegen_path", { target, path, label: label ?? null });
  } catch (err) {
    console.error(`Erro ao guardar path de ${target}:`, err);
    return null;
  }
}

export async function deleteCodegenPath(id: number): Promise<void> {
  try {
    await invoke("delete_codegen_path", { id });
  } catch (err) {
    console.error("Erro ao remover path:", err);
  }
}

export type FrontendProjectResult = {
  projectDir: string;
  framework: string;
  buildTool: string;
  generatedFiles: number;
  dependencies: string[];
  nextSteps: string[];
};

export type FrontendProjectInput = {
  targetDir: string;
  name: string;
  description: string;
  framework: "sveltekit" | "nuxt" | "nextjs";
  apiCallPattern: "bff" | "direct";
  httpClient: "native" | "axios";
  useTanstackQuery: boolean;
  includeTailwind?: boolean;
};

export async function createSpringBootProject(
  input: SpringBootProjectInput,
): Promise<SpringBootProjectResult> {
  return invoke<SpringBootProjectResult>("create_spring_boot_project", {
    request: input,
  });
}

export async function createNestjsProject(
  input: GenericBackendInput,
): Promise<BackendProjectResult> {
  return invoke<BackendProjectResult>("create_nestjs_project", { request: input });
}

export async function createLaravelProject(
  input: Omit<GenericBackendInput, "includeApiDocs">,
): Promise<BackendProjectResult> {
  return invoke<BackendProjectResult>("create_laravel_project", { request: input });
}

export async function createAspnetProject(
  input: GenericBackendInput,
): Promise<BackendProjectResult> {
  return invoke<BackendProjectResult>("create_aspnet_project", { request: input });
}

export async function createSveltekitProject(
  input: FrontendProjectInput,
): Promise<FrontendProjectResult> {
  return invoke<FrontendProjectResult>("create_sveltekit_project", { request: input });
}

export async function createNuxtProject(
  input: FrontendProjectInput,
): Promise<FrontendProjectResult> {
  return invoke<FrontendProjectResult>("create_nuxt_project", { request: input });
}

export async function createNextjsProject(
  input: FrontendProjectInput,
): Promise<FrontendProjectResult> {
  return invoke<FrontendProjectResult>("create_nextjs_project", { request: input });
}

export async function detectSpringPackage(path: string): Promise<string | null> {
  try {
    return await invoke<string | null>("detect_spring_package", { path });
  } catch {
    return null;
  }
}
