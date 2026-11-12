// @ts-nocheck
import CardProduct from "../../../../../molecules/card/product/ui/CardProduct";
import CardButton from "../../../../../molecules/card/shared/card-button/ui/CardButton";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle";
import type { ProductSimiliarProps } from "../data/types";

export default function ProductSimiliar({
  variant = "grid",
  headerAlign = "left",
  titleProps = { title: "Similar Products" },
  descriptionProps,
  columns = 4,
  items = [],
  moreButtonProps,
  className = "",
}: ProductSimiliarProps) {
  const gridColsClass =
    columns === 5
      ? "grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5"
      : columns === 3
        ? "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3"
        : "grid-cols-1 sm:grid-cols-2 lg:grid-cols-4";

  if (variant === "sidebar") {
    return (
      <aside className={`space-y-4 rounded-2xl border border-blender-border bg-blender-card p-5 shadow-sm ${className}`}>
        {titleProps && (
          <div className="border-b border-blender-border pb-3 text-center">
            <CardTitle {...titleProps} />
          </div>
        )}
        <div className="space-y-3">
          {items.map((item, idx) => (
            <CardProduct key={idx} {...item} variant={item.variant ?? "02"} />
          ))}
        </div>
        {moreButtonProps && (
          <div className="pt-2 text-center">
            <CardButton variant="outline" {...moreButtonProps} />
          </div>
        )}
      </aside>
    );
  }

  return (
    <section className={`space-y-6 ${className}`}>
      <div
        className={
          headerAlign === "center"
            ? "flex flex-col items-center text-center space-y-1.5"
            : "flex flex-wrap items-end justify-between gap-4"
        }
      >
        <div>
          {titleProps && <CardTitle {...titleProps} />}
          {descriptionProps && <CardDescription {...descriptionProps} />}
        </div>
        {moreButtonProps && <CardButton variant="outline" {...moreButtonProps} />}
      </div>
      <div className={`grid gap-4 ${gridColsClass}`}>
        {items.map((item, idx) => (
          <CardProduct key={idx} {...item} variant={item.variant ?? "01"} />
        ))}
      </div>
    </section>
  );
}
