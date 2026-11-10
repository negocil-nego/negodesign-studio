import type {
  ComponentExport,
  ProjectLanguage,
  ProjectRecord,
} from "../../projects";
import type { BootswatchThemeSelection } from "../../../../../project/data/style.component";

export type ExportThemeOption = BootswatchThemeSelection | "all";

export type ManifestComponent = {
  id: string;
  name: string;
  entry: Partial<Record<ProjectLanguage, string>>;
  files?: Partial<Record<ProjectLanguage, string[]>>;
  dependencies: string[] | Partial<Record<ProjectLanguage, string[]>>;
  assets?: string[] | Partial<Record<ProjectLanguage, string[]>>;
};

export type ComponentManifest = {
  version: number;
  components: ManifestComponent[];
};

export type ManifestRecord = {
  directory: string;
  path: string;
  content: string;
  manifest: ComponentManifest;
};

export type ComponentCopyResult = {
  copiedFiles: number;
  exportedComponents: number;
  project: ProjectRecord;
};

export type ComponentCopyOptions = {
  locales?: string[];
  theme?: ExportThemeOption;
  staticDirectory?: string;
};

export type CollectedBundle = {
  filesByPath: Map<string, string>;
  exportedById: Map<string, ComponentExport>;
};
