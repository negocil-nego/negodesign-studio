// @ts-nocheck
import { useState } from "react";
import { StyleComponent } from "../../../../data/style.component";
import type { ListGroupEntry, ListGroupsProps } from "../data/types";

const defaultItems: ListGroupEntry[] = [
  { id: "item-1", label: "Cras justo odio", badge: 14, active: true },
  { id: "item-2", label: "Dapibus ac facilisis in", badge: 2 },
  { id: "item-3", label: "Morbi leo risus", badge: 1 },
];

export default function ListGroups({
  items = defaultItems,
  activeId,
  onSelect,
}: ListGroupsProps) {
  const [selectedId, setSelectedId] = useState<string | undefined>(activeId);

  function isActive(item: ListGroupEntry): boolean {
    return selectedId !== undefined ? selectedId === item.id : Boolean(item.active);
  }

  return (
    <ul className={StyleComponent.listGroup.root}>
      {items.map((item) => {
        const active = isActive(item);
        return (
          <li key={item.id}>
            <button
              type="button"
              disabled={item.disabled}
              onClick={() => {
                if (item.disabled) return;
                setSelectedId(item.id);
                onSelect?.(item.id);
              }}
              className={`${StyleComponent.listGroup.item} ${StyleComponent.listGroup.action} ${
                active ? StyleComponent.listGroup.active : ""
              } ${item.disabled ? StyleComponent.listGroup.disabled : ""}`}
            >
              {item.description ? (
                <div className="w-full text-left">
                  <div className={StyleComponent.listGroup.richHeader}>
                    <span className={StyleComponent.listGroup.richTitle}>{item.label}</span>
                    {item.meta && (
                      <span className={StyleComponent.listGroup.richMeta}>{item.meta}</span>
                    )}
                  </div>
                  <p className={StyleComponent.listGroup.richBody}>{item.description}</p>
                  {item.footer && (
                    <small className={StyleComponent.listGroup.richFooter}>{item.footer}</small>
                  )}
                </div>
              ) : (
                <>
                  <span>{item.label}</span>
                  {item.badge !== undefined && (
                    <span className="badge rounded-pill text-bg-primary">{item.badge}</span>
                  )}
                </>
              )}
            </button>
          </li>
        );
      })}
    </ul>
  );
}
