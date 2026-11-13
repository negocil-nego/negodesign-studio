import {
  classifyOpenApiOperation,
  type OpenApiEndpointSelection,
  type OpenApiEntity,
  type OpenApiOperation,
} from "../../../data/migration";

export function endpointKey(entityName: string, method: string, path: string): string {
  return `${entityName}::${method.toUpperCase()}::${path}`;
}

export function defaultOperationsForEntity(entity: OpenApiEntity): OpenApiOperation[] {
  if (entity.operations && entity.operations.length > 0) {
    return entity.operations;
  }
  const base = entity.name
    .replace(/(Response|Request|Dto|Model|Entity|Schema)$/i, "")
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .toLowerCase();
  const slug = base || entity.name.toLowerCase();
  return [
    { method: "GET", path: `/api/${slug}`, crudRole: "list", patternStyle: "restful" },
    { method: "POST", path: `/api/${slug}`, crudRole: "create", patternStyle: "restful" },
    { method: "PUT", path: `/api/${slug}/{id}`, crudRole: "update", patternStyle: "restful" },
    { method: "DELETE", path: `/api/${slug}/{id}`, crudRole: "delete", patternStyle: "restful" },
  ];
}

export function filterEntities(entities: OpenApiEntity[], searchQuery: string): OpenApiEntity[] {
  if (!searchQuery.trim()) return entities;
  const q = searchQuery.toLowerCase();
  return entities.filter(
    (e) =>
      e.name.toLowerCase().includes(q) ||
      e.kind.toLowerCase().includes(q) ||
      e.operations.some(
        (op) =>
          op.path.toLowerCase().includes(q) ||
          op.method.toLowerCase().includes(q) ||
          (op.summary && op.summary.toLowerCase().includes(q)),
      ),
  );
}

export function methodColor(method: string): string {
  const m = method.toUpperCase();
  if (m === "GET") return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
  if (m === "POST") return "bg-blue-500/10 text-blue-500 border-blue-500/20";
  if (m === "PUT") return "bg-amber-500/10 text-amber-500 border-amber-500/20";
  if (m === "DELETE") return "bg-rose-500/10 text-rose-500 border-rose-500/20";
  return "bg-purple-500/10 text-purple-500 border-purple-500/20";
}

export function buildSelectedEndpoints(
  entities: OpenApiEntity[],
  selectedEntityNames: string[],
  selectedEndpointKeys: string[],
): OpenApiEndpointSelection[] {
  const activeEntities = entities.filter((e) => selectedEntityNames.includes(e.name));
  const selectedEndpoints: OpenApiEndpointSelection[] = [];
  for (const entity of activeEntities) {
    const ops = defaultOperationsForEntity(entity);
    const chosenOps = ops.filter((op) =>
      selectedEndpointKeys.includes(endpointKey(entity.name, op.method, op.path)),
    );
    const finalOps = chosenOps.length > 0 ? chosenOps : ops;
    for (const op of finalOps) {
      const info = classifyOpenApiOperation(op);
      selectedEndpoints.push({
        entityName: entity.name,
        method: op.method,
        path: op.path,
        operationId: op.operationId ?? null,
        summary: op.summary ?? null,
        kind: entity.kind,
        crudRole: info.crudRole,
        patternStyle: info.patternStyle,
      });
    }
  }
  return selectedEndpoints;
}
