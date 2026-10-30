import type { SectionFormProps } from "../data/types";
import Button from "../../../../atoms/button/variant/default/ui/Button";
import Text from "../../../../atoms/text/ui/Text";
import Form from "../../../../molecules/form/ui/form";

export default function SectionForm({ isScreen, className = "", titleProps, subtitleProps, descriptionProps, formProps, buttonSubmitProps, buttonsLoginProps = [] }: SectionFormProps) {
  return <section className={`w-full ${isScreen ? "min-h-full flex items-center justify-center" : ""} ${className}`}><div className="w-full max-w-md"><div className="flex flex-col items-center gap-5"><div className="flex flex-col items-center gap-2">{titleProps && <Text {...titleProps} />}{subtitleProps && <Text {...subtitleProps} />}</div>{descriptionProps && <Text {...descriptionProps} />}</div>{formProps && <div className="mt-5"><Form {...formProps} /></div>}{buttonsLoginProps.length > 0 && <div className="mt-4 flex items-center justify-center gap-3">{buttonsLoginProps.map((button, i) => <Button key={i} {...button} />)}</div>}{buttonSubmitProps && <div className="mt-4"><Button {...buttonSubmitProps} /></div>}</div></section>;
}
