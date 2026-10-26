import {
  installComponent,
  refreshActiveProjectComponents,
  type ComponentExport,
  type ProjectLanguage,
  type ProjectRecord,
  type ProjectSourceFile,
} from "../projects";
import { componentCatalog } from "../components";


type ManifestComponent = {
  id: string;
  name: string;
  entry: Partial<Record<ProjectLanguage, string>>;
  files?: Partial<Record<ProjectLanguage, string[]>>;
  dependencies:
    | string[]
    | Partial<Record<ProjectLanguage, string[]>>;
  assets?: string[] | Partial<Record<ProjectLanguage, string[]>>;
};

type ComponentManifest = { version: number; components: ManifestComponent[] };
export type ComponentCopyResult = {
  copiedFiles: number;
  exportedComponents: number;
  project: ProjectRecord;
};
type ManifestRecord = {
  directory: string;
  path: string;
  content: string;
  manifest: ComponentManifest;
};

const sourceModules = import.meta.glob(
  [
    "../../../project/**/*.{svelte,ts,tsx,vue,dart,js}",
    "../../../hooks/**/*.{svelte,ts,tsx,vue,dart}",
    "../../../../static/hugeicons/svg/*.svg",
    "../../../../static/flags/4x3/*.svg",
    "../../../../static/hugeicons/css/*.css",
  ],
  { query: "?raw", import: "default", eager: true },
) as Record<string, string>;

const manifestModules = import.meta.glob(
  "../../../project/ui/**/dependence.json",
  { query: "?raw", import: "default", eager: true },
) as Record<string, string>;

function normalizePath(path: string): string {
  const segments: string[] = [];
  for (const segment of path.replaceAll("\\", "/").split("/")) {
    if (!segment || segment === ".") continue;
    if (segment === "..") segments.pop();
    else segments.push(segment);
  }
  return segments.join("/");
}

function sourcePath(key: string) {
  return normalizePath(key.replace(/^(?:\.\.\/)+/, ""));
}

const sourceByPath = new Map(
  Object.entries(sourceModules).map(([key, content]) => [sourcePath(key), content]),
);

const manifests: ManifestRecord[] = Object.entries(manifestModules).map(
  ([key, content]) => {
    const path = sourcePath(key);
    return {
      directory: path.slice(0, -"/dependence.json".length),
      path,
      content,
      manifest: JSON.parse(content) as ComponentManifest,
    };
  },
);

const componentsById = new Map<
  string,
  { component: ManifestComponent; manifest: ManifestRecord }
>();
const manifestsByPath = new Map(manifests.map((manifest) => [manifest.path, manifest]));
for (const manifest of manifests) {
  if (!manifest.manifest || !Array.isArray(manifest.manifest.components)) {
    continue;
  }
  for (const component of manifest.manifest.components) {
    componentsById.set(component.id, { component, manifest });
  }
}

function resolveManifestPath(path: string): string {
  return normalizePath(path).replace(/^src\//, "");
}

function dependenciesFor(
  component: ManifestComponent,
  language: ProjectLanguage,
): string[] {
  if (Array.isArray(component.dependencies)) return component.dependencies;
  return component.dependencies[language] ?? [];
}

function assetsFor(
  component: ManifestComponent,
  language: ProjectLanguage,
): string[] {
  if (!component.assets) return [];
  if (Array.isArray(component.assets)) return component.assets;
  return component.assets[language] ?? [];
}

function addSource(
  path: string,
  filesByPath: Map<string, string>,
  language: ProjectLanguage,
  optional = false,
) {
  const normalized = normalizePath(path);
  let outputPath = normalized;
  let content = sourceByPath.get(normalized);
  if (content === undefined) {
    if (optional) return;
    throw new Error(`O ficheiro indicado no dependence.json não existe: ${normalized}`);
  }

  if (language === "React" || language === "Vue") {
    outputPath = outputPath.replace(/\.svelte\.ts$/, ".ts");
    content = content.replace(/(\/hooks\/[^"']+)\.svelte(?=["'])/g, "$1");
  }
  filesByPath.set(outputPath, content);
}

function encodeBase64(bytes: Uint8Array): string {
  let binary = "";
  const chunkSize = 0x8000;
  for (let offset = 0; offset < bytes.length; offset += chunkSize) {
    binary += String.fromCharCode(...bytes.subarray(offset, offset + chunkSize));
  }
  return btoa(binary);
}

async function addAsset(
  asset: string,
  filesByPath: Map<string, string>,
  language: ProjectLanguage,
) {
  const normalized = normalizePath(asset);
  if (normalized.endsWith(".woff2")) {
    const response = await fetch(`/${normalized.replace(/^static\//, "")}`);
    if (!response.ok) {
      throw new Error(`Não foi possível carregar o asset Hugeicons ${normalized}.`);
    }
    filesByPath.set(normalized, `base64:${encodeBase64(new Uint8Array(await response.arrayBuffer()))}`);
    return;
  }
  if (asset.includes("*")) {
    const pattern = new RegExp(
      `^${normalizePath(asset).replace(/[.+?^${}()|[\]\\]/g, "\\$&").replaceAll("*", ".*")}$`,
    );
    const matchingPaths = [...sourceByPath.keys()].filter((path) => pattern.test(path));
    if (!matchingPaths.length) {
      throw new Error(`Não foram encontrados ficheiros para o asset ${asset}.`);
    }
    for (const path of matchingPaths) addSource(path, filesByPath, language);
    return;
  }
  addSource(asset, filesByPath, language);
}

export async function copyComponentForProject(
  componentId: string,
  project: ProjectRecord,
  manifestLanguage: ProjectLanguage,
): Promise<ComponentCopyResult> {
  const filesByPath = new Map<string, string>();
  const exportedById = new Map<string, ComponentExport>();
  const visited = new Set<string>();

  async function collect(id: string, manifestPath?: string): Promise<void> {
    if (visited.has(id)) return;
    const requestedManifest = manifestPath
      ? manifestsByPath.get(resolveManifestPath(manifestPath))
      : undefined;
    const record = requestedManifest
      ? (() => {
          const component = requestedManifest.manifest.components.find((item) => item.id === id);
          return component ? { component, manifest: requestedManifest } : undefined;
        })()
      : componentsById.get(id);
    if (!record) throw new Error(`O componente dependente '${id}' não existe no catálogo.`);

    const { component } = record;
    const manifest = record.manifest;
    const entry = component.entry[manifestLanguage];
    if (!entry) {
      throw new Error(`${component.name} não tem uma implementação para ${project.language}.`);
    }

    visited.add(id);
    const componentRoot = `${manifest.directory}/`;
    const entryPath = normalizePath(`${componentRoot}${entry}`);
    addSource(entryPath, filesByPath, manifestLanguage);

    for (const file of component.files?.[manifestLanguage] ?? []) {
      addSource(`${componentRoot}${file}`, filesByPath, manifestLanguage);
    }
    for (const asset of assetsFor(component, manifestLanguage)) {
      await addAsset(asset, filesByPath, manifestLanguage);
    }
    exportedById.set(id, {
      id: component.id,
      name: component.name,
      entry: entryPath,
    });

    for (const dependency of dependenciesFor(component, manifestLanguage)) {
      await collect(dependency);
    }
  }

  const definition = componentCatalog.find((component) => component.id === componentId);
  if (!definition?.dependence) {
    throw new Error(`O componente '${componentId}' não tem um dependence.json associado.`);
  }
  await collect(componentId, definition.dependence);

  const files: ProjectSourceFile[] = [...filesByPath].map(([path, content]) => ({
    path,
    content,
  }));
  const components = [...exportedById.values()];
  const result = await installComponent(files, components);
  await refreshActiveProjectComponents();
  return { ...result, project };
}
