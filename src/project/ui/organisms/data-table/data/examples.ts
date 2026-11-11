import type { DataTableActionItem, DataTableBadgeVariant, DataTableVariant, DataTextProps } from "./types";

export type DataTableExample = {
  variant: DataTableVariant;
  label: string;
  columns: DataTextProps[];
  rows: DataTextProps[];
};

function t(text: string, className?: string): DataTextProps {
  return className ? { text, className } : { text };
}

function badge(text: string, badgeVariant: DataTableBadgeVariant = "success", dot = true): DataTextProps {
  return { text, type: "badge", badgeVariant, dot };
}

function dropdown(actions?: (string | DataTableActionItem)[]): DataTextProps {
  return {
    text: "",
    type: "dropdown",
    actions: actions ?? [
      { label: "Edit", value: "edit", icon: "Edit02" },
      { label: "Details", value: "details", icon: "View" },
      { label: "Delete", value: "delete", icon: "Delete02", variant: "danger" },
    ],
  };
}

export const dataTableExamples: DataTableExample[] = [
  {
    variant: "01",
    label: "Acções",
    columns: [
      t("Name"),
      t("Role"),
      t("Experience"),
      t("Company"),
      t("Score"),
      t("Status", undefined),
      { text: "Action", type: "dropdown" },
    ],
    rows: [
      t("Maya Thompson"),  badge("Whole seller", "primary", false),     t("3 years"), t("ACE Homes LLC  +2"), t("12,054"), badge("Active", "success"),   dropdown(),
      t("Ethan Carter"),   badge("Property manager", "info", false),    t("3 years"), t("A&H Realty"),        t("12,054"), badge("Active", "success"),   dropdown(),
      t("Sophie Lee"),     badge("Investor", "warning", false),         t("3 years"), t("NestHub LLC  +1"),   t("12,054"), badge("Inactive", "danger"),  dropdown(),
      t("Liam Johnson"),   badge("Whole seller", "primary", false),     t("3 years"), t("PrimeKey Co."),      t("12,054"), badge("Active", "success"),   dropdown(),
      t("Olivia Brown"),   badge("Fix & Flip", "neutral", false),       t("3 years"), t("4Walls Group"),      t("12,054"), badge("Active", "success"),   dropdown(),
      t("Noah Davis"),     badge("Whole seller", "primary", false),     t("3 years"), t("Urban Flow  +1"),    t("12,054"), badge("Inactive", "danger"),  dropdown(),
    ],
  },
  {
    variant: "02",
    label: "Envios",
    columns: ["Shipment ID", "Description", "Location", "Type", "Priority", "Carrier", "Contact"].map((l) => t(l)),
    rows: [
      t("#SHP-842"), t("Pallet of Electronics (45 units)"), t("Zone A - Dock 4"),   t("Outbound"), badge("Urgent", "danger"),  t("FedEx Freight"), t("dispatch@fedex.com"),
      t("#SHP-841"), t("Medical Supplies Restock"),          t("Cold Storage B"),    t("Inbound"),  badge("Urgent", "danger"),  t("DHL Express"),   t("med@dhl.com"),
      t("#SHP-840"), t("Office Furniture (200 lbs)"),        t("Receiving Bay 2"),   t("Inbound"),  badge("High", "warning"),   t("UPS Supply"),    t("freight@ups.com"),
      t("#SHP-839"), t("Seasonal Clothing Apparel"),         t("Zone C - Aisle 12"), t("Outbound"), badge("Medium", "success"), t("XPO Logistics"), t("routing@xpo.com"),
      t("#SHP-838"), t("Warehouse Racking Equipment"),       t("Staging Area"),      t("Transfer"), badge("Medium", "success"), t("Internal Fleet"), t("fleet@warehouse.org"),
    ],
  },
  {
    variant: "03",
    label: "Utilizadores",
    columns: ["Full name", "Email", "Role", "Status", "Joined date", "2F Auth", "Actions"].map((l, idx) =>
      idx === 6 ? { text: l, type: "dropdown" } : t(l),
    ),
    rows: [
      t("Liam Smith"),      { text: "smith@example.com", type: "email" },    t("Project Manager"),     badge("Active", "success"),   t("24 Jun 2024, 9:23 pm"),  badge("Enabled", "warning", false), dropdown(),
      t("Noah Anderson"),   { text: "anderson@example.com", type: "email" }, t("UX Designer"),         badge("Active", "success"),   t("15 Mar 2023, 2:45 pm"),  badge("Enabled", "warning", false), dropdown(),
      t("Isabella Garcia"), { text: "garcia@example.com", type: "email" },   t("Front-End Developer"), badge("Inactive", "danger"),  t("10 Apr 2022, 11:30 am"), badge("Enabled", "warning", false), dropdown(),
      t("William Clark"),   { text: "clark@example.com", type: "email" },    t("Product Owner"),       badge("Active", "success"),   t("28 Feb 2023, 6:15 am"),  badge("Enabled", "warning", false), dropdown(),
      t("James Hall"),      { text: "hall@example.com", type: "email" },     t("Business Analyst"),    badge("Active", "success"),   t("19 May 2024, 7:55 am"),  badge("Enabled", "warning", false), dropdown(),
    ],
  },
  {
    variant: "04",
    label: "Clientes",
    columns: ["Customer", "Phone", "Country", "Balance", "Total invoice", "Created On", "Status", "Action"].map((l, idx) =>
      idx === 7 ? { text: l, type: "dropdown" } : t(l),
    ),
    rows: [
      t("Darnell Steward"),  t("(+33) 77 55 87 24"),    t("🇺🇸 USA"),     t("$167.77"),    t("0"), t("May 9, 2014"),        badge("Active", "success"), dropdown(),
      t("Ralph Edwards"),    t("(+33) 66 55 51 05 09"), t("🇨🇦 Canada"),  t("$5,872.78"),  t("5"), t("April 28, 2016"),     badge("Failed", "danger"),  dropdown(),
      t("Leslie Alexander"), t("(+33) 66 55 53 19 16"), t("🇬🇧 UK"),      t("-$2,782.78"), t("4"), t("December 19, 2013"),  badge("Active", "success"), dropdown(),
      t("Courtney Henry"),   t("(+33) 74 55 85 77 21"), t("🇩🇪 Germany"), t("$329.78"),    t("0"), t("December 2, 2018"),   badge("Failed", "danger"),  dropdown(),
      t("Floyd Miles"),      t("(+33) 70 55 55 55 29"), t("🇫🇷 France"),  t("$12.98"),     t("1"), t("September 24, 2017"), badge("Active", "success"), dropdown(),
    ],
  },
  {
    variant: "05",
    label: "Comissões",
    columns: ["Name", "Trading Zones", "Type", "ACN", "Commission % (Buying / Selling)", "Commission % (Buying / Leasing)"].map((l) => t(l)),
    rows: [
      t("Nick Johnston"),  badge("4A · 1A", "success", false), t("Company"),    t("2664 7548 6521"), t("0% / 0%"),   t("0% / 0%"),
      t("Alice Smith"),    badge("5B · 2A", "success", false), t("Individual"), t("1234 5678 9101"), t("10% / 5%"),  t("5% / 2%"),
      t("Jordan Lee"),     badge("3C · 1B", "success", false), t("Company"),    t("9876 5432 1098"), t("2% / 1%"),   t("1% / 0%"),
      t("Emma Brown"),     badge("6D · 3C", "success", false), t("Individual"), t("4567 8901 2345"), t("5% / 3%"),   t("3% / 1%"),
      t("Liam Johnson"),   badge("No Zones", "neutral", false), t("Co-owner"),  t("3210 1234 5678"), t("15% / 10%"), t("10% / 5%"),
    ],
  },
];

export const dataTableExampleByVariant = Object.fromEntries(
  dataTableExamples.map((e) => [e.variant, e]),
) as Record<DataTableVariant, DataTableExample>;
