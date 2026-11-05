import type { DataTableColumn, DataTableRow, DataTableVariant } from "./types";

export type DataTableExample = {
  variant: DataTableVariant;
  label: string;
  columns: DataTableColumn[];
  rows: DataTableRow[];
};

export const dataTableExamples: DataTableExample[] = [
  {
    variant: "01",
    label: "Acções",
    columns: ["Alex", "Role", "Experience", "Company", "Score", "Date", "Action"].map((label) => ({ label, key: label.toLowerCase() })),
    rows: [
      { id: "maya", avatar: "/img/programmer-01.jpg", cells: ["Maya Thompson", "Whole seller", "3 years", "ACE Homes LLC  +2", "12,054", "12/24/2024", ""] },
      { id: "ethan", avatar: "/img/programmer-02.jpg", cells: ["Ethan Carter", "Property manager", "3 years", "A&H Realty", "12,054", "12/24/2024", ""] },
      { id: "sophie", avatar: "/img/ux-01.jpg", cells: ["Sophie Lee", "Investor", "3 years", "NestHub LLC  +1", "12,054", "12/24/2024", ""] },
      { id: "liam", avatar: "/img/programmer-01.jpg", cells: ["Liam Johnson", "Whole seller", "3 years", "PrimeKey Co.", "12,054", "12/24/2024", ""] },
      { id: "olivia", avatar: "/img/programmer-02.jpg", cells: ["Olivia Brown", "Fix & Flip", "3 years", "4Walls Group", "12,054", "12/24/2024", ""] },
      { id: "noah", avatar: "/img/ux-01.jpg", cells: ["Noah Davis", "Whole seller", "3 years", "Urban Flow  +1", "12,054", "12/24/2024", ""] },
    ],
  },
  {
    variant: "02",
    label: "Envios",
    columns: ["Shipment ID", "Description", "Location", "Type", "Priority", "Carrier", "Contact"].map((label) => ({ label, key: label.toLowerCase() })),
    rows: [
      { id: "shp842", badge: "Urgent", cells: ["#SHP-842", "Pallet of Electronics (45 units)", "Zone A - Dock 4", "Outbound", "Urgent", "FedEx Freight", "dispatch@fedex.com"] },
      { id: "shp841", badge: "Urgent", cells: ["#SHP-841", "Medical Supplies Restock", "Cold Storage B", "Inbound", "Urgent", "DHL Express", "med@dhl.com"] },
      { id: "shp840", badge: "High", cells: ["#SHP-840", "Office Furniture (200 lbs)", "Receiving Bay 2", "Inbound", "High", "UPS Supply", "freight@ups.com"] },
      { id: "shp839", badge: "Medium", cells: ["#SHP-839", "Seasonal Clothing Apparel", "Zone C - Aisle 12", "Outbound", "Medium", "XPO Logistics", "routing@xpo.com"] },
      { id: "shp838", badge: "Medium", cells: ["#SHP-838", "Warehouse Racking Equipment", "Staging Area", "Transfer", "Medium", "Internal Fleet", "fleet@warehouse.org"] },
    ],
  },
  {
    variant: "03",
    label: "Utilizadores",
    columns: ["Full name", "Email", "Role", "Status", "Joined date", "2F Auth", "Actions"].map((label) => ({ label, key: label.toLowerCase() })),
    rows: [
      { id: "user-liam", avatar: "/img/programmer-01.jpg", cells: ["Liam Smith", "smith@example.com", "Project Manager", "Active", "24 Jun 2024, 9:23 pm", "Enabled"] },
      { id: "user-noah", avatar: "/img/programmer-02.jpg", cells: ["Noah Anderson", "anderson@example.com", "UX Designer", "Active", "15 Mar 2023, 2:45 pm", "Enabled"] },
      { id: "user-isabella", avatar: "/img/ux-01.jpg", cells: ["Isabella Garcia", "garcia@example.com", "Front-End Developer", "Inactive", "10 Apr 2022, 11:30 am", "Enabled"] },
      { id: "user-william", avatar: "/img/programmer-01.jpg", cells: ["William Clark", "clark@example.com", "Product Owner", "Active", "28 Feb 2023, 6:15 am", "Enabled"] },
      { id: "user-james", avatar: "/img/programmer-02.jpg", cells: ["James Hall", "hall@example.com", "Business Analyst", "Active", "19 May 2024, 7:55 am", "Enabled"] },
    ],
  },
  {
    variant: "04",
    label: "Clientes",
    columns: ["Customer", "Phone", "Country", "Balance", "Total invoice", "Created On", "Status", "Action"].map((label) => ({ label, key: label.toLowerCase() })),
    rows: [
      { id: "customer-darnell", avatar: "/img/programmer-01.jpg", cells: ["Darnell Steward", "(+33) 77 55 87 24", "🇺🇸 USA", "$167.77", "0", "May 9, 2014", "Active", ""] },
      { id: "customer-ralph", avatar: "/img/programmer-02.jpg", cells: ["Ralph Edwards", "(+33) 66 55 51 05 09", "🇨🇦 Canada", "$5,872.78", "5", "April 28, 2016", "Failed", ""] },
      { id: "customer-leslie", avatar: "/img/ux-01.jpg", cells: ["Leslie Alexander", "(+33) 66 55 53 19 16", "🇬🇧 UK", "-$2,782.78", "4", "December 19, 2013", "Active", ""] },
      { id: "customer-courtney", avatar: "/img/programmer-01.jpg", cells: ["Courtney Henry", "(+33) 74 55 85 77 21", "🇩🇪 Germany", "$329.78", "0", "December 2, 2018", "Failed", ""] },
      { id: "customer-floyd", avatar: "/img/programmer-02.jpg", cells: ["Floyd Miles", "(+33) 70 55 55 55 29", "🇫🇷 France", "$12.98", "1", "September 24, 2017", "Active", ""] },
    ],
  },
  {
    variant: "05",
    label: "Comissões",
    columns: ["Name", "Trading Zones", "Type", "ACN", "Commission % (Buying / Selling)", "Commission % (Buying / Leasing)"].map((label) => ({ label, key: label.toLowerCase() })),
    rows: [
      { id: "partner-nick", cells: ["Nick Johnston", "4A · 1A", "Company", "2664 7548 6521", "0% / 0%", "0% / 0%"] },
      { id: "partner-alice", cells: ["Alice Smith", "5B · 2A", "Individual", "1234 5678 9101", "10% / 5%", "5% / 2%"] },
      { id: "partner-jordan", cells: ["Jordan Lee", "3C · 1B", "Company", "9876 5432 1098", "2% / 1%", "1% / 0%"] },
      { id: "partner-emma", cells: ["Emma Brown", "6D · 3C", "Individual", "4567 8901 2345", "5% / 3%", "3% / 1%"] },
      { id: "partner-liam", cells: ["Liam Johnson", "No Zones", "Co-owner", "3210 1234 5678", "15% / 10%", "10% / 5%"] },
    ],
  },
];

export const dataTableExampleByVariant = Object.fromEntries(
  dataTableExamples.map((example) => [example.variant, example]),
) as Record<DataTableVariant, DataTableExample>;
