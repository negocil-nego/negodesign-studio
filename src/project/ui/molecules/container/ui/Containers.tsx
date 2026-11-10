// @ts-nocheck
import { StyleComponent } from "../../../../data/style.component";
import type { ContainersProps } from "../data/types";

export default function Containers({
  title = "Hello, world!",
  lead = "This is a simple hero unit, a simple jumbotron-style component for calling extra attention to featured content or information.",
  body = "It uses utility classes for typography and spacing to space content out within the larger container.",
  actionLabel = "Learn more",
  onAction,
}: ContainersProps) {
  return (
    <div className={StyleComponent.containers.root}>
      <div className={StyleComponent.containers.jumbotron}>
        <h2 className={StyleComponent.containers.title}>{title}</h2>
        <p className={StyleComponent.containers.lead}>{lead}</p>
        <hr className={StyleComponent.containers.divider} />
        <p className={StyleComponent.containers.body}>{body}</p>
        {actionLabel && (
          <div className={StyleComponent.containers.actionWrap}>
            <button type="button" className="btn btn-primary btn-lg" onClick={onAction}>
              {actionLabel}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
