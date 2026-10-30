import type { ReactNode } from "react";
import { StyleComponent } from "../../../../data/style.component";

type AccordionProps = { title: string; count?: string; open?: boolean; children: ReactNode };

export default function Accordion({ title, count, open = false, children }: AccordionProps) {
  return <details className={StyleComponent.accordion.root} open={open}>
    <summary className={StyleComponent.accordion.summary}>
      <span>{title}</span>
      <span className={StyleComponent.accordion.meta}>{count && <span>{count}</span>}<span className={StyleComponent.accordion.indicator} aria-hidden="true">⌄</span></span>
    </summary>
    <div className={StyleComponent.accordion.content}>{children}</div>
  </details>;
}
