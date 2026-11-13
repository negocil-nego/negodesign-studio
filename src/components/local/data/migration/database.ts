import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

/** Motores de base de dados suportados na migração de projecto. */
export type DatabaseDriver = "mysql" | "postgres";

export type DatabaseConnection = {
  driver: DatabaseDriver;
  host: string;
  port: number;
  database: string;
  user: string;
  password: string;
};

export type SavedDatabaseConnection = Omit<DatabaseConnection, "password"> & {
  id: number;
  lastConnected: string;
};

export type DatabaseColumn = {
  name: string;
  dataType: string;
  nullable: boolean;
  primaryKey: boolean;
};

export type DatabaseTable = {
  name: string;
  columns: DatabaseColumn[];
};

export const databaseTables = writable<DatabaseTable[]>([]);
export const activeDatabaseConnection = writable<DatabaseConnection | null>(null);

export async function connectDatabase(
  connection: DatabaseConnection,
): Promise<DatabaseTable[]> {
  const tables = await invoke<DatabaseTable[]>("connect_database", { connection });
  activeDatabaseConnection.set(connection);
  databaseTables.set(tables);
  return tables;
}

export async function listSavedDatabaseConnections(): Promise<SavedDatabaseConnection[]> {
  return invoke<SavedDatabaseConnection[]>("list_saved_database_connections");
}

export async function connectSavedDatabase(id: number): Promise<DatabaseTable[]> {
  const tables = await invoke<DatabaseTable[]>("connect_saved_database", { id });
  databaseTables.set(tables);
  return tables;
}
