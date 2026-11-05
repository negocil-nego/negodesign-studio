import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

/** Motores de base de dados suportados na migração de projecto. */
export type DatabaseDriver = "mysql" | "postgres";

export type DatabaseConnection = {
  driver: DatabaseDriver;
  host: string;
  port: number;
  database: string;
  user: string;
  password: string;
};

export type DatabaseColumn = {
  name: string;
  dataType: string;
  nullable: boolean;
  primaryKey: boolean;
};

export type DatabaseTable = {
  name: string;
  columns: DatabaseColumn[];
};

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
};

export const databaseTables = writable<DatabaseTable[]>([]);
export const openApiEntities = writable<OpenApiEntity[]>([]);

export async function connectDatabase(
  connection: DatabaseConnection,
): Promise<DatabaseTable[]> {
  const tables = await invoke<DatabaseTable[]>("connect_database", { connection });
  databaseTables.set(tables);
  return tables;
}

export async function fetchOpenApi(url: string): Promise<OpenApiEntity[]> {
  const entities = await invoke<OpenApiEntity[]>("fetch_openapi", { url });
  openApiEntities.set(entities);
  return entities;
}

export const entityKindLabels: Record<OpenApiEntityKind, string> = {
  response: "Response",
  dto: "DTO",
  request: "Request",
  schema: "Schema",
};
