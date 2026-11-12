// @ts-nocheck
import { useState } from "react";
import CardDescription from "../../../../../molecules/card/shared/card-description/ui/CardDescription";
import CardTitle from "../../../../../molecules/card/shared/card-title/ui/CardTitle";
import UnderlineNav from "../../../../../molecules/underline-nav/ui/UnderlineNav";
import type { ProductTabsProps } from "../data/types";

export default function ProductTabs({
  underlineNavProps,
  titleProps,
  descriptionProps,
  paragraphs = [],
  specs = [],
  highlights = [],
  tabContents = [],
  specsStyle = "inline",
  className = "",
}: ProductTabsProps) {
  const navItems = underlineNavProps?.items ?? [
    { value: "description", label: "Description" },
    { value: "reviews", label: "Reviews (150)" },
    { value: "support", label: "Support" },
  ];
  const [activeTab, setActiveTab] = useState(
    underlineNavProps?.value ?? navItems[0]?.value ?? "description",
  );

  const activeContent = tabContents.find((t) => t.value === activeTab);
  const currentTitle = activeContent?.titleProps ?? titleProps;
  const currentDescription = activeContent?.descriptionProps ?? descriptionProps;
  const currentParagraphs = activeContent?.paragraphs ?? paragraphs;
  const currentSpecs = activeContent?.specs ?? specs;
  const currentHighlights = activeContent?.highlights ?? highlights;

  return (
    <div className={`space-y-5 ${className}`}>
      <UnderlineNav
        items={navItems}
        value={activeTab}
        onValueChange={(val) => {
          setActiveTab(val);
          underlineNavProps?.onValueChange?.(val);
        }}
      />
      <div className="space-y-4 pt-1">
        {currentTitle && <CardTitle {...currentTitle} />}
        {currentSpecs.length > 0 &&
          (specsStyle === "table" ? (
            <div className="divide-y divide-blender-border/60 rounded-xl border border-blender-border bg-blender-surface/40">
              {currentSpecs.map((spec) => (
                <div key={spec.label} className="grid grid-cols-2 gap-4 px-4 py-2.5 text-xs">
                  <span className="text-zinc-400">{spec.label}</span>
                  <span className="font-medium text-zinc-200">{spec.value}</span>
                </div>
              ))}
            </div>
          ) : (
            <dl className="grid gap-2 text-xs sm:grid-cols-2">
              {currentSpecs.map((spec, idx) => (
                <div key={idx} className="flex items-baseline gap-2">
                  <dt className="font-medium text-zinc-400">{spec.label}:</dt>
                  <dd className="text-zinc-200">{spec.value}</dd>
                </div>
              ))}
            </dl>
          ))}
        {currentDescription && <CardDescription {...currentDescription} />}
        {currentParagraphs.length > 0 && (
          <div className="space-y-3 text-xs leading-relaxed text-zinc-400">
            {currentParagraphs.map((p) => (
              <p key={p}>{p}</p>
            ))}
          </div>
        )}
        {currentHighlights.length > 0 && (
          <ul className="list-disc space-y-1.5 pl-5 text-xs text-zinc-300">
            {currentHighlights.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}
