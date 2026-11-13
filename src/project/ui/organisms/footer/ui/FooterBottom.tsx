// @ts-nocheck
import { StyleComponent } from "../../../../data/style.component";
import type { FooterBottomProps } from "../data/types";

export default function FooterBottom({
  paymentBadges = [],
  bottomLinks = [],
  copyrightText = "© 2026 Negodesign Studio. All rights reserved.",
}: FooterBottomProps) {
  const S = StyleComponent.footer;

  return (
    <div className={S.bottomBar}>
      <div className={S.bottomInner}>
        <span>{copyrightText}</span>

        {bottomLinks.length > 0 && (
          <div className={S.bottomLinks}>
            {bottomLinks.map((link) => (
              <a key={link.label} href={link.href ?? "#"} className={S.linkItem}>
                {link.label}
              </a>
            ))}
          </div>
        )}

        {paymentBadges.length > 0 && (
          <div className={S.badgesRow}>
            {paymentBadges.map((badge) => (
              <span key={badge} className={S.badgePill}>
                {badge}
              </span>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
