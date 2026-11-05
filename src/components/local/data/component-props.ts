export type ComponentPropDef = {
  label: string;
  key: string;
  type: "text" | "number" | "image" | "boolean";
};

export const componentPropsMap: Record<string, ComponentPropDef[]> = {
  "card-media": [
    { label: "Título", key: "title", type: "text" },
    { label: "Imagem (URL)", key: "image", type: "image" },
    { label: "Descrição", key: "description", type: "text" },
    { label: "Preço", key: "price", type: "number" },
  ],
  "card-profile": [
    { label: "Nome", key: "title", type: "text" },
    { label: "Foto (URL)", key: "logo", type: "image" },
    { label: "Capa (URL)", key: "image", type: "image" },
    { label: "Handle (@)", key: "handle", type: "text" },
    { label: "Bio", key: "description", type: "text" },
  ],
  "card-product": [
    { label: "Nome", key: "title", type: "text" },
    { label: "Imagem (URL)", key: "image", type: "image" },
    { label: "Marca (URL)", key: "logo", type: "image" },
    { label: "Preço", key: "price", type: "number" },
    { label: "Descrição", key: "description", type: "text" },
  ],
  "input-text": [
    { label: "Label", key: "label", type: "text" },
    { label: "Placeholder", key: "placeholder", type: "text" },
    { label: "Valor Inicial", key: "value", type: "text" },
  ],
  "textarea": [
    { label: "Label", key: "label", type: "text" },
    { label: "Placeholder", key: "placeholder", type: "text" },
    { label: "Linhas", key: "rows", type: "number" },
  ],
  "data-table": [
    { label: "Título da Tabela", key: "title", type: "text" },
  ],
};

export function getComponentProps(componentId: string): ComponentPropDef[] {
  return componentPropsMap[componentId] || [
    { label: "Título/Nome", key: "title", type: "text" },
    { label: "Descrição", key: "description", type: "text" },
  ];
}
