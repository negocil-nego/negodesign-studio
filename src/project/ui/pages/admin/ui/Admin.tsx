import AdminCrud from "./AdminCrud";
import type { AdminProps } from "../data/types";

export default function Admin(props: AdminProps) {
  return <AdminCrud {...props} />;
}

