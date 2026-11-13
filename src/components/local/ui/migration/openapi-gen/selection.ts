import type { OpenApiEntity, OpenApiOperation } from "../../../data/migration";
import { defaultOperationsForEntity, endpointKey } from "./endpoints";

export type Selection = {
  entityNames: string[];
  endpointKeys: string[];
};

function keysForEntity(entity: OpenApiEntity): string[] {
  return defaultOperationsForEntity(entity).map((op) =>
    endpointKey(entity.name, op.method, op.path),
  );
}

export function initialSelection(
  entities: OpenApiEntity[],
  initialNames: string[],
): Selection {
  const entityNames =
    initialNames.length > 0 ? [...initialNames] : entities.map((e) => e.name);
  const endpointKeys = entities
    .filter((e) => entityNames.includes(e.name))
    .flatMap(keysForEntity);
  return { entityNames, endpointKeys };
}

export function toggleEntitySelection(
  entities: OpenApiEntity[],
  sel: Selection,
  name: string,
): Selection {
  const entity = entities.find((e) => e.name === name);
  const keys = entity ? keysForEntity(entity) : [];
  if (sel.entityNames.includes(name)) {
    return {
      entityNames: sel.entityNames.filter((n) => n !== name),
      endpointKeys: sel.endpointKeys.filter((k) => !keys.includes(k)),
    };
  }
  return {
    entityNames: [...sel.entityNames, name],
    endpointKeys: [...new Set([...sel.endpointKeys, ...keys])],
  };
}

export function toggleEndpointSelection(
  entities: OpenApiEntity[],
  sel: Selection,
  entityName: string,
  op: OpenApiOperation,
): Selection {
  const key = endpointKey(entityName, op.method, op.path);
  if (sel.endpointKeys.includes(key)) {
    const endpointKeys = sel.endpointKeys.filter((k) => k !== key);
    const entity = entities.find((e) => e.name === entityName);
    const remaining = (entity ? defaultOperationsForEntity(entity) : []).some((o) =>
      endpointKeys.includes(endpointKey(entityName, o.method, o.path)),
    );
    return {
      entityNames: remaining
        ? sel.entityNames
        : sel.entityNames.filter((n) => n !== entityName),
      endpointKeys,
    };
  }
  return {
    entityNames: sel.entityNames.includes(entityName)
      ? sel.entityNames
      : [...sel.entityNames, entityName],
    endpointKeys: [...sel.endpointKeys, key],
  };
}

export function selectAllEntities(entities: OpenApiEntity[]): Selection {
  return {
    entityNames: entities.map((e) => e.name),
    endpointKeys: entities.flatMap(keysForEntity),
  };
}

export function emptySelection(): Selection {
  return { entityNames: [], endpointKeys: [] };
}
