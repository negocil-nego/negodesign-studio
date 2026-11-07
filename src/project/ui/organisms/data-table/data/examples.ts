import type { DataTableVariant } from "./types";
import type { TextProps } from "../../../atoms/text/data/types";

export type DataTableExample = {
  variant: DataTableVariant;
  label: string;
  columns: TextProps[];
  rows: TextProps[];
};

function t(text: string, className?: string): TextProps {
  return className ? { text, className } : { text };
}

export const dataTableExamples: DataTableExample[] = [
  {
    variant: "01",
    label: "Acções",
    columns: ["Name", "Role", "Experience", "Company", "Score", "Date", "Action"].map((l) => t(l)),
    rows: [
      t("Maya Thompson"),  t("Whole seller"),     t("3 years"), t("ACE Homes LLC  +2"), t("12,054"), t("12/24/2024"), t(""),
      t("Ethan Carter"),   t("Property manager"), t("3 years"), t("A&H Realty"),        t("12,054"), t("12/24/2024"), t(""),
      t("Sophie Lee"),     t("Investor"),         t("3 years"), t("NestHub LLC  +1"),   t("12,054"), t("12/24/2024"), t(""),
      t("Liam Johnson"),   t("Whole seller"),     t("3 years"), t("PrimeKey Co."),      t("12,054"), t("12/24/2024"), t(""),
      t("Olivia Brown"),   t("Fix & Flip"),       t("3 years"), t("4Walls Group"),      t("12,054"), t("12/24/2024"), t(""),
      t("Noah Davis"),     t("Whole seller"),     t("3 years"), t("Urban Flow  +1"),    t("12,054"), t("12/24/2024"), t(""),
    ],
  },
  {
    variant: "02",
    label: "Envios",
    columns: ["Shipment ID", "Description", "Location", "Type", "Priority", "Carrier", "Contact"].map((l) => t(l)),
    rows: [
      t("#SHP-842"), t("Pallet of Electronics (45 units)"), t("Zone A - Dock 4"),   t("Outbound"), t("Urgent", "rounded-full bg-red-50 px-2 py-1 text-[10px] text-red-600"),       t("FedEx Freight"), t("dispatch@fedex.com"),
      t("#SHP-841"), t("Medical Supplies Restock"),          t("Cold Storage B"),    t("Inbound"),  t("Urgent", "rounded-full bg-red-50 px-2 py-1 text-[10px] text-red-600"),       t("DHL Express"),   t("med@dhl.com"),
      t("#SHP-840"), t("Office Furniture (200 lbs)"),        t("Receiving Bay 2"),   t("Inbound"),  t("High",   "rounded-full bg-orange-50 px-2 py-1 text-[10px] text-orange-600"), t("UPS Supply"),    t("freight@ups.com"),
      t("#SHP-839"), t("Seasonal Clothing Apparel"),         t("Zone C - Aisle 12"), t("Outbound"), t("Medium", "rounded-full bg-emerald-50 px-2 py-1 text-[10px] text-emerald-700"), t("XPO Logistics"), t("routing@xpo.com"),
      t("#SHP-838"), t("Warehouse Racking Equipment"),       t("Staging Area"),      t("Transfer"), t("Medium", "rounded-full bg-emerald-50 px-2 py-1 text-[10px] text-emerald-700"), t("Internal Fleet"), t("fleet@warehouse.org"),
    ],
  },
  {
    variant: "03",
    label: "Utilizadores",
    columns: ["Full name", "Email", "Role", "Status", "Joined date", "2F Auth", "Actions"].map((l) => t(l)),
    rows: [
      t("Liam Smith"),      t("smith@example.com"),    t("Project Manager"),     t("Active",   "rounded-md border px-2 py-1 text-emerald-700"), t("24 Jun 2024, 9:23 pm"),  t("Enabled", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
      t("Noah Anderson"),   t("anderson@example.com"), t("UX Designer"),         t("Active",   "rounded-md border px-2 py-1 text-emerald-700"), t("15 Mar 2023, 2:45 pm"),  t("Enabled", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
      t("Isabella Garcia"), t("garcia@example.com"),   t("Front-End Developer"), t("Inactive", "rounded-md border px-2 py-1 text-red-600"),     t("10 Apr 2022, 11:30 am"), t("Enabled", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
      t("William Clark"),   t("clark@example.com"),    t("Product Owner"),       t("Active",   "rounded-md border px-2 py-1 text-emerald-700"), t("28 Feb 2023, 6:15 am"),  t("Enabled", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
      t("James Hall"),      t("hall@example.com"),     t("Business Analyst"),    t("Active",   "rounded-md border px-2 py-1 text-emerald-700"), t("19 May 2024, 7:55 am"),  t("Enabled", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
    ],
  },
  {
    variant: "04",
    label: "Clientes",
    columns: ["Customer", "Phone", "Country", "Balance", "Total invoice", "Created On", "Status", "Action"].map((l) => t(l)),
    rows: [
      t("Darnell Steward"),  t("(+33) 77 55 87 24"),    t("🇺🇸 USA"),     t("$167.77"),    t("0"), t("May 9, 2014"),        t("Active", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
      t("Ralph Edwards"),    t("(+33) 66 55 51 05 09"), t("🇨🇦 Canada"),  t("$5,872.78"),  t("5"), t("April 28, 2016"),     t("Failed", "rounded px-2 py-1 bg-rose-100 text-rose-700"),   t(""),
      t("Leslie Alexander"), t("(+33) 66 55 53 19 16"), t("🇬🇧 UK"),      t("-$2,782.78"), t("4"), t("December 19, 2013"),  t("Active", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
      t("Courtney Henry"),   t("(+33) 74 55 85 77 21"), t("🇩🇪 Germany"), t("$329.78"),    t("0"), t("December 2, 2018"),   t("Failed", "rounded px-2 py-1 bg-rose-100 text-rose-700"),   t(""),
      t("Floyd Miles"),      t("(+33) 70 55 55 55 29"), t("🇫🇷 France"),  t("$12.98"),     t("1"), t("September 24, 2017"), t("Active", "rounded px-2 py-1 bg-amber-100 text-amber-900"), t(""),
    ],
  },
  {
    variant: "05",
    label: "Comissões",
    columns: ["Name", "Trading Zones", "Type", "ACN", "Commission % (Buying / Selling)", "Commission % (Buying / Leasing)"].map((l) => t(l)),
    rows: [
      t("Nick Johnston"),  t("4A · 1A", "rounded-full bg-emerald-100 px-2 py-1 text-emerald-700"), t("Company"),    t("2664 7548 6521"), t("0% / 0%"),   t("0% / 0%"),
      t("Alice Smith"),    t("5B · 2A", "rounded-full bg-emerald-100 px-2 py-1 text-emerald-700"), t("Individual"), t("1234 5678 9101"), t("10% / 5%"),  t("5% / 2%"),
      t("Jordan Lee"),     t("3C · 1B", "rounded-full bg-emerald-100 px-2 py-1 text-emerald-700"), t("Company"),    t("9876 5432 1098"), t("2% / 1%"),   t("1% / 0%"),
      t("Emma Brown"),     t("6D · 3C", "rounded-full bg-emerald-100 px-2 py-1 text-emerald-700"), t("Individual"), t("4567 8901 2345"), t("5% / 3%"),   t("3% / 1%"),
      t("Liam Johnson"),   t("No Zones"),                                                           t("Co-owner"),   t("3210 1234 5678"), t("15% / 10%"), t("10% / 5%"),
    ],
  },
];

export const dataTableExampleByVariant = Object.fromEntries(
  dataTableExamples.map((e) => [e.variant, e]),
) as Record<DataTableVariant, DataTableExample>;
