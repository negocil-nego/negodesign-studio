// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardTagsProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon";

export default function CardTags({ tags, className, itemClassName }: CardTagsProps) {
    return (
        <div className={className ?? StyleComponent.card.tags.root}>
            {tags.map((tag, index) => (
                <div key={index} className={itemClassName ?? StyleComponent.card.tags.item}>
                    {tag.icon && <Icon name={tag.icon} className={StyleComponent.card.tags.icon} />}
                    <span className={StyleComponent.card.tags.value}>{tag.title}</span>
                    {tag.description && <span className={StyleComponent.card.tags.label}>{tag.description}</span>}
                </div>
            ))}
        </div>
    );
}
