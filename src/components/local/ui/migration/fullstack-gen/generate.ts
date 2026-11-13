import { get } from "svelte/store";
import { bootswatchUi } from "../../../data/bootswatch-ui.svelte";
import {
  activeDatabaseConnection,
  generateFullstackCrud,
  type ApiCallPattern,
  type ArchitectureMode,
  type BackendFramework,
  type DatabaseEngine,
  type DatabaseTable,
  type FrontendFramework,
  type FullstackCodegenResult,
  type HttpClientKind,
  type SpringLanguage,
} from "../../../data/migration";

export type FullstackGenerationInput = {
  architectureMode: ArchitectureMode;
  tables: DatabaseTable[];
  selectedTableNames: string[];
  frontendPath: string;
  frontendFramework: FrontendFramework;
  apiCallPattern: ApiCallPattern;
  httpClient: HttpClientKind;
  useTanstackQuery: boolean;
  backendPath: string;
  backendFramework: BackendFramework;
  basePackage: string;
  springLanguage: SpringLanguage;
  ormStrategy: string;
  databaseEngine: DatabaseEngine;
  backendBaseUrl: string;
};

export type FullstackGenerationOutcome = {
  result: FullstackCodegenResult | null;
  errorMessage: string;
};

export function initialTableSelection(
  tables: DatabaseTable[],
  initialSelectedTables: string[],
): string[] {
  return initialSelectedTables.length > 0
    ? [...initialSelectedTables]
    : tables.map((t) => t.name);
}

export function toggleTableName(names: string[], name: string): string[] {
  return names.includes(name) ? names.filter((n) => n !== name) : [...names, name];
}

export function selectAllTableNames(tables: DatabaseTable[]): string[] {
  return tables.map((t) => t.name);
}

export async function runFullstackGeneration(
  input: FullstackGenerationInput,
): Promise<FullstackGenerationOutcome> {
  const chosenTables = input.tables.filter((t) =>
    input.selectedTableNames.includes(t.name),
  );
  if (chosenTables.length === 0) {
    return { result: null, errorMessage: "Seleccione pelo menos uma tabela para gerar." };
  }
  try {
    const result = await generateFullstackCrud({
      architectureMode: input.architectureMode,
      tables: chosenTables,
      frontendPath: input.frontendPath,
      frontendFramework: input.frontendFramework,
      apiCallPattern: input.apiCallPattern,
      httpClient: input.httpClient,
      useTanstackQuery: input.useTanstackQuery,
      backendPath: input.architectureMode === "split" ? input.backendPath : undefined,
      backendFramework:
        input.architectureMode === "split" ? input.backendFramework : undefined,
      ormStrategy: input.architectureMode === "split" ? input.ormStrategy : undefined,
      basePackage:
        input.architectureMode === "split" && input.backendFramework === "spring-boot"
          ? input.basePackage
          : undefined,
      springLanguage:
        input.architectureMode === "split" && input.backendFramework === "spring-boot"
          ? input.springLanguage
          : undefined,
      databaseEngine: input.databaseEngine,
      backendBaseUrl: input.backendBaseUrl,
      theme: bootswatchUi.theme,
      databaseConnection: get(activeDatabaseConnection) ?? undefined,
    });
    return { result, errorMessage: "" };
  } catch (err) {
    return {
      result: null,
      errorMessage: err instanceof Error ? err.message : String(err),
    };
  }
}
