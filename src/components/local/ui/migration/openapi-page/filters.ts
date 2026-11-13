import {
  classifyOpenApiOperation,
  summarizeEntityCrudPattern,
  type OpenApiEntity,
  type OpenApiEntityKind,
} from "../../../data/migration";

export type EntityFilter =
  | OpenApiEntityKind
  | "all"
  | "admin-crud"
  | "restful"
  | "named";

export function matchesFilter(entity: OpenApiEntity, filter: EntityFilter): boolean {
  if (filter === "all") return true;
  if (filter === "admin-crud") {
    return summarizeEntityCrudPattern(entity).hasCrud;
  }
  if (filter === "restful") {
    return (entity.operations ?? []).some(
      (op) => classifyOpenApiOperation(op).patternStyle === "restful",
    );
  }
  if (filter === "named") {
    return (entity.operations ?? []).some(
      (op) => classifyOpenApiOperation(op).patternStyle === "named",
    );
  }
  return entity.kind === filter;
}

export function countEntities(entities: OpenApiEntity[], filter: EntityFilter): number {
  return entities.filter((entity) => matchesFilter(entity, filter)).length;
}

export function distinctKinds(entities: OpenApiEntity[]): OpenApiEntityKind[] {
  return [...new Set(entities.map((entity) => entity.kind))];
}
