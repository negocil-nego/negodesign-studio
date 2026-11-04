import Menu from "../../../../organisms/menu/simple/ui/Menu";
import Button from "../../../../atoms/button/variant/default/ui/Button";
import Text from "../../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../../../../components/local/data/style.component";
import type { SectionTextProps } from "../../../../organisms/section/hero/shared/data/types";
import type { PageHeroDualProps } from "../data/types";
function HeroText({ data }: { data: SectionTextProps }) {
 const s=StyleComponent.pages.hero.dual;
 return <section className={s.content}>{data.titleProps&&<div className={s.eyebrow}><Text {...data.titleProps} className={s.eyebrowText}/></div>}{data.subtitleProps&&<div role="heading" aria-level={1}><Text {...data.subtitleProps} className={s.titleText}/></div>}{data.descriptionProps&&<Text {...data.descriptionProps} className={s.descriptionText}/>} {!!data.buttonsProps?.length&&<div className="relative z-20 mt-6 flex flex-wrap gap-3">{data.buttonsProps.map((b,i)=><Button key={i} {...b}/>)}</div>}{data.imageUrl&&<img src={data.imageUrl} alt={data.imageAlt??""} className={s.media}/>}</section>
}
export default function PageHeroDual({menuProps,leftSectionHereProps,rightSectionHereProps}:PageHeroDualProps){return <header className={StyleComponent.pages.hero.dual.root}><div className={StyleComponent.pages.hero.dual.backdrop} aria-hidden="true"/>{menuProps&&<aside className={StyleComponent.pages.hero.dual.navigation}><Menu {...menuProps} style={menuProps.style??"ghost"}/></aside>}<div className={StyleComponent.pages.hero.dual.layout}>{[leftSectionHereProps,rightSectionHereProps].map((section,i)=>section&&<div className={StyleComponent.pages.hero.dual.section} key={i}>{section.textSectionProps&&<HeroText data={section.textSectionProps}/>}</div>)}</div></header>}
