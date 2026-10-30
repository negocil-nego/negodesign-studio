import type { MenuTitleProps } from "../data/types";
import { StyleComponent } from "../../../../../../../../components/local/data/style.component";
import Text from "../../../../../../atoms/text/ui/Text";

export default function MenuTitle({ title, link, description, href, variant = "item" }: MenuTitleProps) {
  const classes = variant === "featured" ? StyleComponent.menu.title.featured : StyleComponent.menu.title.item;
  return <div className={classes.root}>
    {title && (href ? <a className={classes.titleLink} href={href}><Text {...title} /></a> : <div className={classes.title}><Text {...title} /></div>)}
    {description && <div className={classes.description}><Text {...description} /></div>}
    {link && (variant === "featured" && href ? <a className={classes.link} href={href}><Text {...link} /><span aria-hidden="true">→</span></a> : <div className={classes.link}><Text {...link} /></div>)}
  </div>;
}
