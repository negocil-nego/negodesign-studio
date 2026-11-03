import type { ReactNode } from "react";
import type { SectionGridProps } from "../data/types";
type Props = Omit<SectionGridProps, "children"> & { children: ReactNode };

export default function SectionGrid({ titleProps, descriptionProps, className = "", children }: Props) {
  return (
    <section className={`w-full min-w-0 py-6 ${className}`.trim()}>
      {(titleProps || descriptionProps) && (
        <header className="mb-6">
          {titleProps && (
            <h2 className="text-2xl font-semibold leading-tight text-zinc-100 sm:text-3xl">
              {titleProps.text}
            </h2>
          )}
          {descriptionProps && (
            <p className="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">
              {descriptionProps.text}
            </p>
          )}
        </header>
      )}
      <div className="grid grid-cols-1 gap-2 sm:grid-cols-2 md:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 [&>*]:w-full [&>*]:min-w-0">
        {children}
      </div>
    </section>
  );
}
