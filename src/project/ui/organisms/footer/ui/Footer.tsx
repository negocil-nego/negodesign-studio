// @ts-nocheck
import { useState } from "react";
import Button from "../../../atoms/button/variant/default/ui/Button";
import Icon from "../../../atoms/icon/ui/Icon";
import Image from "../../../atoms/image/ui/Image";
import Text from "../../../atoms/text/ui/Text";
import { StyleComponent } from "../../../../data/style.component";
import type { FooterProps } from "../data/types";

export default function Footer({
  variant = "01",
  logoProps,
  logoText = "Negodesign Store",
  logoSubtitle,
  descriptionProps,
  openingHours = [],
  contactItems = [],
  columns = [],
  newsletterProps,
  socialLinks = [],
  paymentBadges = [],
  bottomLinks = [],
  copyrightText = "© 2026 Negodesign Studio. All rights reserved.",
  className = "",
}: FooterProps) {
  const S = StyleComponent.footer;
  const [email, setEmail] = useState("");

  const submitNewsletter = (e: React.FormEvent) => {
    e.preventDefault();
    newsletterProps?.onSubmit?.(email);
    setEmail("");
  };

  return (
    <footer className={`${S.root} ${className}`}>
      {variant === "02" && newsletterProps?.bannerTitle && (
        <div className={S.bannerWrap}>
          <div className={S.bannerInner}>
            <h3 className={S.bannerTitle}>{newsletterProps.bannerTitle}</h3>
            <form className={S.bannerForm} onSubmit={submitNewsletter}>
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder={newsletterProps.placeholder ?? "Drop your email here"}
                className={S.input}
              />
              <Button
                type="submit"
                text={newsletterProps.buttonProps?.text ?? "Subscribe"}
                {...newsletterProps.buttonProps}
              />
            </form>
          </div>
        </div>
      )}

      <div className={S.mainWrap}>
        <div className={S.brandCol}>
          <div className={S.brandRow}>
            {logoProps && (
              <Image {...logoProps} className={logoProps.className ?? "size-8 rounded-lg object-cover"} />
            )}
            <div>
              <span className={S.brandTitle}>{logoText}</span>
              {logoSubtitle && <p className={S.brandSubtitle}>{logoSubtitle}</p>}
            </div>
          </div>

          {descriptionProps && (
            <p className={S.description}>
              <Text {...descriptionProps} />
            </p>
          )}

          {openingHours.length > 0 && (
            <div className="space-y-1 text-xs text-zinc-400">
              {openingHours.map((line) => (
                <p key={line}>{line}</p>
              ))}
            </div>
          )}

          {contactItems.length > 0 && (
            <ul className={S.contactList}>
              {contactItems.map((item) => (
                <li key={item.text} className={S.contactItem}>
                  {item.icon && <Icon name={item.icon} className="size-4 text-blue-400 shrink-0" />}
                  <span>{item.text}</span>
                </li>
              ))}
            </ul>
          )}

          {socialLinks.length > 0 && (
            <div className={S.socialRow}>
              {socialLinks.map((social) => (
                <a
                  key={social.icon}
                  href={social.href ?? "#"}
                  aria-label={social.label ?? social.icon}
                  className={S.socialBtn}
                >
                  <Icon name={social.icon} className="size-4" />
                </a>
              ))}
            </div>
          )}
        </div>

        {columns.map((col) => (
          <div key={col.title} className={S.linksCol}>
            <h4 className={S.colTitle}>{col.title}</h4>
            <ul className={S.linkList}>
              {col.links.map((link) => (
                <li key={link.label}>
                  <a href={link.href ?? "#"} className={S.linkItem}>
                    {link.label}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        ))}

        {newsletterProps && variant !== "02" && (
          <div className={S.newsletterCol}>
            <h4 className={S.colTitle}>{newsletterProps.title ?? "Newsletter"}</h4>
            {newsletterProps.description && <p className={S.description}>{newsletterProps.description}</p>}
            <form className="flex flex-col gap-2.5 sm:flex-row" onSubmit={submitNewsletter}>
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder={newsletterProps.placeholder ?? "Your email address"}
                className={S.input}
              />
              <button type="submit" className={S.submitBtn}>
                {newsletterProps.buttonProps?.text ?? "Subscribe"}
              </button>
            </form>
          </div>
        )}
      </div>

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
    </footer>
  );
}
