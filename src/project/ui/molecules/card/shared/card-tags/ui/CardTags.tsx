// @ts-nocheck
import type { CardTagsProps } from "../data/types";
import Icon from "../../../../../atoms/icon/ui/Icon";

export default function CardTags({ tags, className, itemClassName }: CardTagsProps) {
    return (
        <div className={className ?? "flex gap-4 w-full border-y border-gray-100 py-4"}>
            {tags.map((tag, index) => (
                <div key={index} className={itemClassName ?? "flex flex-col items-center justify-center flex-1 first:border-l-0 border-l border-gray-100"}>
                    {tag.icon && <Icon name={tag.icon} className="mb-1" />}
                    <span className="text-lg font-bold text-gray-900">{tag.title}</span>
                    {tag.description && <span className="text-sm text-gray-500">{tag.description}</span>}
                </div>
            ))}
        </div>
    );
}
