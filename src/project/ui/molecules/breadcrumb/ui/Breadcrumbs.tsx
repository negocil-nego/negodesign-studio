// @ts-nocheck
import { StyleComponent } from "../../../../data/style.component";
import type { BreadcrumbsProps } from "../data/types";

export default function Breadcrumbs({
  items = [
    { label: "Home", href: "#home" },
    { label: "Library", href: "#library" },
    { label: "Data", active: true },
  ],
  onSelect,
}: BreadcrumbsProps) {
  return (
    <nav aria-label="breadcrumb" className={StyleComponent.breadcrumb.nav}>
      <ol className={StyleComponent.breadcrumb.list}>
        {items.map((item, idx) => {
          const isLast = idx === items.length - 1 || Boolean(item.active);
          return (
            <li
              key={idx}
              className={`${StyleComponent.breadcrumb.item} ${
                isLast ? StyleComponent.breadcrumb.itemActive : ""
              }`}
              aria-current={isLast ? "page" : undefined}
            >
              {!isLast && item.href ? (
                <a
                  href={item.href}
                  className={StyleComponent.breadcrumb.link}
                  onClick={(e) => {
                    if (onSelect) {
                      e.preventDefault();
                      onSelect(item, idx);
                    }
                  }}
                >
                  {item.label}
                </a>
              ) : (
                <span>{item.label}</span>
              )}
            </li>
          );
        })}
      </ol>
    </nav>
  );
}
