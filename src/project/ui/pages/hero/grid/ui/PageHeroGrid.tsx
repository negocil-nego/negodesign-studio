import Menu from "../../../../organisms/menu/simple/ui/Menu";
import Button from "../../../../atoms/button/variant/default/ui/Button";
import Text from "../../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../../../data/style.component";
import {
  HERO_GRID_COLS,
  HERO_GRID_POS,
  HERO_GRID_ROWS,
  type PageHeroGridCardProps,
  type PageHeroGridProps,
} from "../data/types";

function CardContent({ card }: { card: PageHeroGridCardProps }) {
  const p = card.sectionCardProps;
  if (!p) return null;
  const s = p.tone === "light" ? StyleComponent.pages.hero.gridLight : StyleComponent.pages.hero.grid;
  const pos = HERO_GRID_POS[p.contentPosition ?? "end"];
  return (
    <section className={`${s.content} ${pos}`}>
      {p.titleProps && (
        <div className={s.eyebrow}>
          <Text {...p.titleProps} className={s.eyebrowText} />
        </div>
      )}
      {p.subtitleProps && (
        <div role="heading" aria-level={2}>
          <Text {...p.subtitleProps} className={s.titleText} />
        </div>
      )}
      {p.descriptionProps && <Text {...p.descriptionProps} className={s.descriptionText} />}
      {!!p.buttonsProps?.length && (
        <div className="relative z-20 mt-4 flex flex-wrap justify-start gap-3">
          {p.buttonsProps.map((b, i) => (
            <Button key={i} {...b} className={b.className ?? s.action} />
          ))}
        </div>
      )}
    </section>
  );
}

export default function PageHeroGrid({
  menuProps,
  backgroundColor = "#17161c",
  cards = [],
}: PageHeroGridProps) {
  const g = StyleComponent.pages.hero.grid;
  return (
    <section className={g.root} style={{ backgroundColor }} aria-label="Hero em mosaico">
      {menuProps && (
        <aside className={g.navigation}>
          <Menu {...menuProps} style={menuProps.style ?? "ghost-inverse"} />
        </aside>
      )}
      <div className={g.tiles}>
        {cards.map((card) => (
          <article
            key={card.id}
            className={`${g.card} ${HERO_GRID_COLS[card.colSpan ?? 12]} ${HERO_GRID_ROWS[card.rowSpan ?? 2]} ${card.className ?? ""}`}
            style={{ backgroundColor: card.backgroundColor ?? "#28262e" }}
            aria-label={card.sectionCardProps?.subtitleProps?.text ?? card.id}
          >
            {card.imageUrl && (
              <img
                src={card.imageUrl}
                alt={card.imageAlt ?? ""}
                loading="lazy"
                className={`${card.imageMode === "contain" ? g.imageContain : g.imageCover} ${card.imageClassName ?? ""}`}
              />
            )}
            {card.showOverlay && <div className={g.overlay} aria-hidden="true" />}
            <CardContent card={card} />
          </article>
        ))}
      </div>
    </section>
  );
}
