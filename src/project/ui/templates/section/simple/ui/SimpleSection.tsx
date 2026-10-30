import type { SimpleProps } from "../data/types";
import SectionForm from "../../../../organisms/section/form/ui/SectionForm";
export default function SimpleSection({ sectionFormProps }: SimpleProps) { return <section className="h-full min-h-full"><SectionForm {...sectionFormProps} /></section>; }
