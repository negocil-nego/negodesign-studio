import type { LoginDualSectionProps } from "../data/types";
import SectionForm from "../../../../organisms/section/form/ui/SectionForm";
export default function DualSection({ leftSectionForm, rightSectionForm }: LoginDualSectionProps) { return <section className="grid h-full min-h-full grid-cols-1 md:grid-cols-2"><SectionForm {...leftSectionForm} /><SectionForm {...rightSectionForm} /></section>; }
