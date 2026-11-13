import type { PageProductDetailsProps } from "../../../../project/ui/pages/product-details/data/types";
import { sidebar } from "./sidebar";
import { showcase } from "./showcase";
import { classic } from "./classic";
import { minimal } from "./minimal";
import { relatedGrid } from "./related-grid";
import { catalogSpecs } from "./catalog-specs";
import { fresh } from "./fresh";

export const productDetailsTemplates: Record<string, PageProductDetailsProps> = {
  "PageProductDetails · Sidebar": sidebar,
  "PageProductDetails · Showcase": showcase,
  "PageProductDetails · Classic": classic,
  "PageProductDetails · Minimal": minimal,
  "PageProductDetails · Related Grid": relatedGrid,
  "PageProductDetails · Catalog Specs": catalogSpecs,
  "PageProductDetails · Fresh": fresh,
};
