import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";
import type { OpenApiCrudRole, OpenApiPatternStyle } from "./classification";

/** Tipo de entidade extraída da documentação OpenAPI. */
export type OpenApiEntityKind = "response" | "dto" | "request" | "schema";

export type OpenApiField = {
  name: string;
  type: string;
  required: boolean;
  description: string | null;
};

export type OpenApiEntity = {
  name: string;
  kind: OpenApiEntityKind;
  fields: OpenApiField[];
  operations: OpenApiOperation[];
  serverUrl?: string | null;
};

export type OpenApiOperation = {
  method: string;
  path: string;
  operationId?: string | null;
  summary?: string | null;
  crudRole?: OpenApiCrudRole | string | null;
  patternStyle?: OpenApiPatternStyle | string | null;
};

export const openApiEntities = writable<OpenApiEntity[]>([]);

export async function fetchOpenApi(url: string): Promise<OpenApiEntity[]> {
  const entities = await invoke<OpenApiEntity[]>("fetch_openapi", { url });
  openApiEntities.set(entities);
  return entities;
}

export async function importOpenApiFile(path: string): Promise<OpenApiEntity[]> {
  const entities = await invoke<OpenApiEntity[]>("import_openapi_file", { path });
  openApiEntities.set(entities);
  return entities;
}

export const entityKindLabels: Record<OpenApiEntityKind, string> = {
  response: "Response",
  dto: "DTO",
  request: "Request",
  schema: "Schema",
};
