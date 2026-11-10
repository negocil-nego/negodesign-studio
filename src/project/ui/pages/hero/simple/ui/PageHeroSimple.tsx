import Menu from "../../../../organisms/menu/simple/ui/Menu";
import Button from "../../../../atoms/button/variant/default/ui/Button";
import Text from "../../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../../../data/style.component";
import type { PageHeroSimpleProps } from "../data/types";

export default function PageHeroSimple({ menuProps, sectionHeroProps }: PageHeroSimpleProps) {
  const hero = sectionHeroProps ?? {};
  const light = hero.style === "light";
  const text = hero.textSectionProps;
  const styles = light ? StyleComponent.pages.hero.light : StyleComponent.pages.hero.simple;
  const opacity = Math.min(1, Math.max(0, hero.backgroundOverlayOpacity ?? 0.55));

  return (
    <header className={styles.root} style={{ backgroundColor: light ? "#eef5f6" : hero.backgroundColor ?? "#10212b" }}>
      {light ? (
        <div className={StyleComponent.pages.hero.light.backdrop} aria-hidden="true" />
      ) : (
        hero.backgroundMediaUrl && (
          <div
            className={StyleComponent.pages.hero.simple.background}
            style={{ backgroundImage: `url("${hero.backgroundMediaUrl}")` }}
            aria-hidden="true"
          />
        )
      )}
      {!light && (
        <>
          <div className={StyleComponent.pages.hero.simple.overlay} aria-hidden="true" />
          <div className={StyleComponent.pages.hero.simple.shade} style={{ opacity }} aria-hidden="true" />
        </>
      )}
      {menuProps && (
        <aside className={styles.navigation}>
          <Menu {...menuProps} style={light ? "ghost" : "ghost-inverse"} />
        </aside>
      )}
      {text && (
        <section className={styles.content}>
          {text.titleProps && (
            <div className={styles.eyebrow}>
              <Text {...text.titleProps} className={styles.eyebrowText} />
            </div>
          )}
          {text.subtitleProps && (
            <div role="heading" aria-level={1}>
              <Text {...text.subtitleProps} className={styles.titleText} />
            </div>
          )}
          {text.descriptionProps && <Text {...text.descriptionProps} className={styles.descriptionText} />}
          {!!text.buttonsProps?.length && (
            <div className="relative z-20 mt-6 flex flex-wrap justify-center gap-3">
              {text.buttonsProps.map((button, i) => (
                <Button
                  key={i}
                  {...button}
                  className={button.className ?? (light ? StyleComponent.pages.hero.light.action : undefined)}
                />
              ))}
            </div>
          )}
          {text.imageUrl && (
            <img
              src={text.imageUrl}
              alt={text.imageAlt ?? ""}
              className={light ? StyleComponent.pages.hero.light.illustration : StyleComponent.pages.hero.dual.media}
            />
          )}
        </section>
      )}
    </header>
  );
}
