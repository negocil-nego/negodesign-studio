import type { OpenApiEntity, OpenApiOperation } from "./openapi";
import {
  classifyOpenApiOperation,
  type OpenApiCrudRole,
} from "./classification";

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
