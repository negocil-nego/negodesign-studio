// @ts-nocheck
import { useState } from "react";
import Button from "../../../atoms/button/variant/default/ui/Button";
import { StyleComponent } from "../../../../data/style.component";
import type { FooterProps } from "../data/types";
import FooterBottom from "./FooterBottom";
import FooterMain from "./FooterMain";

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

      <FooterMain
        variant={variant}
        logoProps={logoProps}
        logoText={logoText}
        logoSubtitle={logoSubtitle}
        descriptionProps={descriptionProps}
        openingHours={openingHours}
        contactItems={contactItems}
        columns={columns}
        newsletterProps={newsletterProps}
        socialLinks={socialLinks}
        email={email}
        onEmailChange={setEmail}
        onSubmit={submitNewsletter}
      />

      <FooterBottom paymentBadges={paymentBadges} bottomLinks={bottomLinks} copyrightText={copyrightText} />
    </footer>
  );
}
