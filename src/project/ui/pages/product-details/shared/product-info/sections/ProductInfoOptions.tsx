// @ts-nocheck
import type { ProductInfoOptionsProps } from "../data/types";

export default function ProductInfoOptions({
  colorsLabel = "Colors", colors = [], sizesLabel = "Size", sizes = [], purchaseOptions = [],
  showQuantity = false, stockCountText, subtotalText, deliveryText, urgencyText,
  quantity, selectedColor, selectedSize, selectedPurchase,
  onQuantityChange, onSelectedColorChange, onSelectedSizeChange, onSelectedPurchaseChange,
}: ProductInfoOptionsProps) {
  return (
    <>
      {colors.length > 0 && (
        <div className="space-y-2">
          <span className="block text-xs font-semibold text-zinc-300">{colorsLabel}</span>
          <div className="flex flex-wrap items-center gap-2.5">
            {colors.map((c) => (
              <button
                key={c.value}
                type="button"
                onClick={() => onSelectedColorChange?.(c.value)}
                style={{ backgroundColor: c.hex }}
                className="flex size-7 items-center justify-center rounded-md border border-blender-border text-[10px] font-bold text-white"
              >
                {selectedColor === c.value ? "✓" : ""}
              </button>
            ))}
          </div>
        </div>
      )}

      {sizes.length > 0 && (
        <div className="space-y-2">
          <span className="block text-xs font-semibold text-zinc-300">{sizesLabel}</span>
          <div className="flex flex-wrap items-center gap-2">
            {sizes.map((s) => (
              <button
                key={s.value}
                type="button"
                onClick={() => onSelectedSizeChange?.(s.value)}
                className={`rounded-lg border px-3 py-1.5 text-xs font-medium ${
                  selectedSize === s.value ? "border-blue-500 bg-blue-600 text-white" : "border-blender-border bg-blender-surface text-zinc-300"
                }`}
              >
                {s.priceLabel ? `${s.label} = ${s.priceLabel}` : s.label}
              </button>
            ))}
          </div>
        </div>
      )}

      {purchaseOptions.length > 0 && (
        <div className="space-y-2 rounded-xl border border-blender-border bg-blender-surface/50 p-3">
          {purchaseOptions.map((opt) => (
            <button
              key={opt.id}
              type="button"
              onClick={() => onSelectedPurchaseChange?.(opt.id)}
              className="flex w-full items-center justify-between rounded-lg px-3 py-2 text-xs text-zinc-200"
            >
              <span>{opt.label}</span>
              <span className="font-bold">{opt.price}</span>
            </button>
          ))}
        </div>
      )}

      {showQuantity && (
        <div className="flex items-center gap-4">
          <div className="inline-flex items-center rounded-lg border border-blender-border bg-blender-surface">
            <button type="button" onClick={() => onQuantityChange?.(Math.max(1, quantity - 1))} className="px-3 py-1.5 text-xs font-bold text-zinc-300">
              −
            </button>
            <span className="min-w-8 text-center text-xs font-semibold text-zinc-100">{quantity}</span>
            <button type="button" onClick={() => onQuantityChange?.(quantity + 1)} className="px-3 py-1.5 text-xs font-bold text-zinc-300">
              +
            </button>
          </div>
          {stockCountText && <span className="text-xs text-zinc-400">{stockCountText}</span>}
        </div>
      )}

      {subtotalText && (
        <p className="text-xs text-zinc-300">
          Subtotal: <span className="font-bold text-zinc-100">{subtotalText}</span>
        </p>
      )}
      {(deliveryText || urgencyText) && (
        <div className="flex flex-wrap items-center gap-3 text-xs">
          {deliveryText && <span className="text-zinc-300">{deliveryText}</span>}
          {urgencyText && <span className="font-semibold text-rose-400">{urgencyText}</span>}
        </div>
      )}
    </>
  );
}
