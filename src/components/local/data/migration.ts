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

export type OpenApiCrudRole = "list" | "get" | "create" | "update" | "delete";
export type OpenApiPatternStyle = "restful" | "named";

export type OpenApiOperation = {
  method: string;
  path: string;
  operationId?: string | null;
  summary?: string | null;
  crudRole?: OpenApiCrudRole | string | null;
  patternStyle?: OpenApiPatternStyle | string | null;
};

const LIST_NAMED_SEGMENTS = [
  "list", "all", "find-all", "findall", "find_all", "get-all", "getall", "get_all",
  "search", "index", "page", "paginate", "paginated", "query", "fetch", "items", "catalog",
];
const GET_ONE_NAMED_SEGMENTS = [
  "detail", "details", "by-id", "byid", "by_id", "find-by-id", "get-by-id", "show", "view", "one",
];
const CREATE_NAMED_SEGMENTS = ["create", "add", "insert", "new", "store", "register"];
const UPDATE_NAMED_SEGMENTS = ["update", "edit", "save", "modify", "change", "patch", "upsert"];
const DELETE_NAMED_SEGMENTS = ["delete", "remove", "destroy", "erase", "del", "drop"];

export function classifyOpenApiOperation(op: OpenApiOperation): {
  crudRole: OpenApiCrudRole;
  patternStyle: OpenApiPatternStyle;
} {
  if (
    op.crudRole &&
    ["list", "get", "create", "update", "delete"].includes(op.crudRole) &&
    op.patternStyle &&
    ["restful", "named"].includes(op.patternStyle)
  ) {
    return {
      crudRole: op.crudRole as OpenApiCrudRole,
      patternStyle: op.patternStyle as OpenApiPatternStyle,
    };
  }
  const m = op.method.trim().toUpperCase();
  const segments = op.path
    .split("/")
    .map((s) => s.trim().toLowerCase())
    .filter((s) => s.length > 0 && !(s.startsWith("{") && s.endsWith("}")));
  const endsWithParam = /\/\{[^}]+\}\/?$/.test(op.path.trim());

  const hasSeg = (list: string[]) => segments.some((seg) => list.includes(seg));

  if (m === "GET") {
    if (hasSeg(LIST_NAMED_SEGMENTS)) return { crudRole: "list", patternStyle: "named" };
    if (hasSeg(GET_ONE_NAMED_SEGMENTS)) return { crudRole: "get", patternStyle: "named" };
    if (endsWithParam) return { crudRole: "get", patternStyle: "restful" };
    return { crudRole: "list", patternStyle: "restful" };
  }
  if (m === "POST") {
    if (hasSeg(DELETE_NAMED_SEGMENTS)) return { crudRole: "delete", patternStyle: "named" };
    if (hasSeg(UPDATE_NAMED_SEGMENTS)) return { crudRole: "update", patternStyle: "named" };
    if (hasSeg(LIST_NAMED_SEGMENTS)) return { crudRole: "list", patternStyle: "named" };
    if (hasSeg(CREATE_NAMED_SEGMENTS)) return { crudRole: "create", patternStyle: "named" };
    return { crudRole: "create", patternStyle: "restful" };
  }
  if (m === "PUT" || m === "PATCH") {
    if (hasSeg(UPDATE_NAMED_SEGMENTS)) return { crudRole: "update", patternStyle: "named" };
    return { crudRole: "update", patternStyle: "restful" };
  }
  if (m === "DELETE") {
    if (hasSeg(DELETE_NAMED_SEGMENTS)) return { crudRole: "delete", patternStyle: "named" };
    return { crudRole: "delete", patternStyle: "restful" };
  }
  return { crudRole: "list", patternStyle: "restful" };
}

export function summarizeEntityCrudPattern(entity: OpenApiEntity): {
  hasCrud: boolean;
  styleLabel: string;
  roles: Record<OpenApiCrudRole, OpenApiOperation[]>;
} {
  const roles: Record<OpenApiCrudRole, OpenApiOperation[]> = {
    list: [],
    get: [],
    create: [],
    update: [],
    delete: [],
  };
  let hasNamed = false;
  let hasRestful = false;

  for (const op of entity.operations ?? []) {
    const { crudRole, patternStyle } = classifyOpenApiOperation(op);
    roles[crudRole].push(op);
    if (patternStyle === "named") hasNamed = true;
    else hasRestful = true;
  }

  const hasCrud =
    roles.list.length > 0 ||
    roles.create.length > 0 ||
    roles.update.length > 0 ||
    roles.delete.length > 0;

  const styleLabel =
    hasNamed && hasRestful
      ? "RESTful + Métodos Nomeados"
      : hasNamed
        ? "Nomenclatura de Métodos (/list, /create, /update, /delete)"
        : hasRestful
          ? "API RESTful [GET, POST, PUT/PATCH, DELETE]"
          : "Padrão Default REST";

  return { hasCrud, styleLabel, roles };
}

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

export type OpenApiEndpointSelection = {
  entityName: string;
  method: string;
  path: string;
  operationId?: string | null;
  summary?: string | null;
  kind: string;
  crudRole?: OpenApiCrudRole | string | null;
  patternStyle?: OpenApiPatternStyle | string | null;
};

export type OpenApiFrontendCodegenRequest = {
  entities: OpenApiEntity[];
  selectedEndpoints: OpenApiEndpointSelection[];
  frontendPath: string;
  frontendFramework: FrontendFramework;
  apiCallPattern: ApiCallPattern;
  httpClient: HttpClientKind;
  useTanstackQuery: boolean;
  backendBaseUrl?: string;
  theme?: string;
};

export type OpenApiFrontendCodegenResult = {
  generatedFrontendFiles: number;
  copiedUiFiles: number;
  endpointsProcessed: number;
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

export async function generateOpenApiFrontend(
  request: OpenApiFrontendCodegenRequest,
): Promise<OpenApiFrontendCodegenResult> {
  return invoke<OpenApiFrontendCodegenResult>("generate_openapi_frontend", { request });
}

export const entityKindLabels: Record<OpenApiEntityKind, string> = {
  response: "Response",
  dto: "DTO",
  request: "Request",
  schema: "Schema",
};
