import { invoke } from "@tauri-apps/api/core";
import type { DatabaseTable } from "./database";
import type { OpenApiCrudRole, OpenApiPatternStyle } from "./classification";
import type { OpenApiEntity } from "./openapi";

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
