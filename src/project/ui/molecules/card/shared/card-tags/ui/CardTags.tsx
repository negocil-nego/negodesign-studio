// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardTagsProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon";

export default function CardTags({ tags, variant = "stats", className, itemClassName, titleClassName, descriptionClassName }: CardTagsProps) {
    const tagStyle = variant === "buttons" ? StyleComponent.card.tags.buttons : StyleComponent.card.tags;
    return (
        <div className={className ?? tagStyle.root}>
            {tags.map((tag, index) => (
                <div key={index} className={itemClassName ?? tagStyle.item}>
                    {tag.icon && <Icon name={tag.icon} className={tagStyle.icon} />}
                    <span className={titleClassName ?? tagStyle.value}>{tag.title}</span>
                    {tag.description && <span className={descriptionClassName ?? tagStyle.label}>{tag.description}</span>}
                </div>
            ))}
        </div>
    );
}
