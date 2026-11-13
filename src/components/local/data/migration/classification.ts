export type OpenApiCrudRole = "list" | "get" | "create" | "update" | "delete";
export type OpenApiPatternStyle = "restful" | "named";

const LIST_NAMED_SEGMENTS = [
  "list", "all", "find-all", "findall", "find_all", "get-all", "getall", "get_all",
  "search", "index", "page", "paginate", "paginated", "query", "fetch", "items", "catalog",
];
const GET_ONE_NAMED_SEGMENTS = [
  "detail", "details", "by-id", "byid", "by_id", "find-by-id", "get-by-id", "show", "view", "one",
];
const CREATE_NAMED_SEGMENTS = ["create", "add", "insert", "new", "store", "register"];
const UPDATE_NAMED_SEGMENTS = ["update", "edit", "save", "modify", "change", "patch", "upsert"];
const DELETE_NAMED_SEGMENTS = ["delete", "remove", "destroy", "erase", "del", "drop"];

export function classifyOpenApiOperation(op: {
  method: string;
  path: string;
  crudRole?: OpenApiCrudRole | string | null;
  patternStyle?: OpenApiPatternStyle | string | null;
}): {
  crudRole: OpenApiCrudRole;
  patternStyle: OpenApiPatternStyle;
} {
  if (
    op.crudRole &&
    ["list", "get", "create", "update", "delete"].includes(op.crudRole) &&
    op.patternStyle &&
    ["restful", "named"].includes(op.patternStyle)
  ) {
    return {
      crudRole: op.crudRole as OpenApiCrudRole,
      patternStyle: op.patternStyle as OpenApiPatternStyle,
    };
  }
  const m = op.method.trim().toUpperCase();
  const segments = op.path
    .split("/")
    .map((s) => s.trim().toLowerCase())
    .filter((s) => s.length > 0 && !(s.startsWith("{") && s.endsWith("}")));
  const endsWithParam = /\/\{[^}]+\}\/?$/.test(op.path.trim());

  const hasSeg = (list: string[]) => segments.some((seg) => list.includes(seg));

  if (m === "GET") {
    if (hasSeg(LIST_NAMED_SEGMENTS)) return { crudRole: "list", patternStyle: "named" };
    if (hasSeg(GET_ONE_NAMED_SEGMENTS)) return { crudRole: "get", patternStyle: "named" };
    if (endsWithParam) return { crudRole: "get", patternStyle: "restful" };
    return { crudRole: "list", patternStyle: "restful" };
  }
  if (m === "POST") {
    if (hasSeg(DELETE_NAMED_SEGMENTS)) return { crudRole: "delete", patternStyle: "named" };
    if (hasSeg(UPDATE_NAMED_SEGMENTS)) return { crudRole: "update", patternStyle: "named" };
    if (hasSeg(LIST_NAMED_SEGMENTS)) return { crudRole: "list", patternStyle: "named" };
    if (hasSeg(CREATE_NAMED_SEGMENTS)) return { crudRole: "create", patternStyle: "named" };
    return { crudRole: "create", patternStyle: "restful" };
  }
  if (m === "PUT" || m === "PATCH") {
    if (hasSeg(UPDATE_NAMED_SEGMENTS)) return { crudRole: "update", patternStyle: "named" };
    return { crudRole: "update", patternStyle: "restful" };
  }
  if (m === "DELETE") {
    if (hasSeg(DELETE_NAMED_SEGMENTS)) return { crudRole: "delete", patternStyle: "named" };
    return { crudRole: "delete", patternStyle: "restful" };
  }
  return { crudRole: "list", patternStyle: "restful" };
}
