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

export type SavedDatabaseConnection = Omit<DatabaseConnection, "password"> & {
  id: number;
  lastConnected: string;
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
  operations: OpenApiOperation[];
  serverUrl?: string | null;
};

export type OpenApiOperation = {
  method: string;
  path: string;
  operationId?: string | null;
  summary?: string | null;
};

export type ArchitectureMode = "split" | "monolith";
export type FrontendFramework = "sveltekit" | "nuxt" | "nextjs";
export type BackendFramework = "spring-boot" | "nestjs" | "laravel" | "aspnet-core";
export type ApiCallPattern = "bff" | "direct";
export type HttpClientKind = "native" | "axios";
export type DatabaseEngine = "postgres" | "mysql" | "sqlite";

export type FullstackCodegenRequest = {
  architectureMode: ArchitectureMode;
  tables: DatabaseTable[];
  frontendPath: string;
  frontendFramework: FrontendFramework;
  apiCallPattern: ApiCallPattern;
  httpClient: HttpClientKind;
  useTanstackQuery: boolean;
  backendPath?: string;
  backendFramework?: BackendFramework;
  ormStrategy?: string;
  basePackage?: string;
  databaseEngine: DatabaseEngine;
  backendBaseUrl?: string;
  theme?: string;
};

export type FullstackCodegenResult = {
  generatedBackendFiles: number;
  generatedFrontendFiles: number;
  copiedUiFiles: number;
  tablesProcessed: number;
  summary: string[];
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

export async function listSavedDatabaseConnections(): Promise<SavedDatabaseConnection[]> {
  return invoke<SavedDatabaseConnection[]>("list_saved_database_connections");
}

export async function connectSavedDatabase(id: number): Promise<DatabaseTable[]> {
  const tables = await invoke<DatabaseTable[]>("connect_saved_database", { id });
  databaseTables.set(tables);
  return tables;
}

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

export async function generateFullstackCrud(
  request: FullstackCodegenRequest,
): Promise<FullstackCodegenResult> {
  return invoke<FullstackCodegenResult>("generate_fullstack_crud", { request });
}

export const entityKindLabels: Record<OpenApiEntityKind, string> = {
  response: "Response",
  dto: "DTO",
  request: "Request",
  schema: "Schema",
};
