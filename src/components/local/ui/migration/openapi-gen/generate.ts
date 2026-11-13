import { bootswatchUi } from "../../../data/bootswatch-ui.svelte";
import {
  generateOpenApiFrontend,
  type ApiCallPattern,
  type FrontendFramework,
  type HttpClientKind,
  type OpenApiEntity,
  type OpenApiFrontendCodegenResult,
} from "../../../data/migration";
import { buildSelectedEndpoints } from "./endpoints";

export type GenerationOptions = {
  entities: OpenApiEntity[];
  selectedEntityNames: string[];
  selectedEndpointKeys: string[];
  frontendPath: string;
  frontendFramework: FrontendFramework;
  apiCallPattern: ApiCallPattern;
  httpClient: HttpClientKind;
  useTanstackQuery: boolean;
  backendBaseUrl: string;
};

export type GenerationOutcome = {
  result: OpenApiFrontendCodegenResult | null;
  errorMessage: string;
};

export async function runOpenApiGeneration(
  opts: GenerationOptions,
): Promise<GenerationOutcome> {
  try {
    const activeEntities = opts.entities.filter((e) =>
      opts.selectedEntityNames.includes(e.name),
    );
    const selectedEndpoints = buildSelectedEndpoints(
      opts.entities,
      opts.selectedEntityNames,
      opts.selectedEndpointKeys,
    );
    const result = await generateOpenApiFrontend({
      entities: activeEntities,
      selectedEndpoints,
      frontendPath: opts.frontendPath.trim(),
      frontendFramework: opts.frontendFramework,
      apiCallPattern: opts.apiCallPattern,
      httpClient: opts.httpClient,
      useTanstackQuery: opts.useTanstackQuery,
      backendBaseUrl: opts.backendBaseUrl.trim() || undefined,
      theme: bootswatchUi.theme,
    });
    return { result, errorMessage: "" };
  } catch (err) {
    return { result: null, errorMessage: err instanceof Error ? err.message : String(err) };
  }
}
