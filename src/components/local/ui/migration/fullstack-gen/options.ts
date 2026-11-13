import type { BackendFramework, FrontendFramework } from "../../../data/migration";

export type OrmOption = { id: string; label: string; desc: string };

export const ormOptionsByBackend: Record<BackendFramework, OrmOption[]> = {
  "spring-boot": [
    { id: "spring-data-jpa", label: "Spring Data JPA", desc: "Entidades Jakarta Persistence + JpaRepository paginado" },
    { id: "spring-jdbc", label: "Spring JDBC (Nova API JdbcClient)", desc: "Consultas fluentes com org.springframework.jdbc.core.simple.JdbcClient" },
  ],
  nestjs: [
    { id: "prisma", label: "Prisma ORM", desc: "Repositórios tipados usando PrismaClient (findMany, create, update, delete)" },
    { id: "drizzle", label: "Drizzle ORM", desc: "Consultas SQL-like tipadas com Drizzle" },
    { id: "typeorm", label: "TypeORM", desc: "Entidades decoradas e Repository<T> do TypeORM" },
  ],
  laravel: [
    { id: "eloquent", label: "Eloquent ORM", desc: "Models Eloquent activos para CRUD e paginação" },
    { id: "query-builder", label: "Query Builder", desc: "Consultas fluentes com Illuminate\\Support\\Facades\\DB::table()" },
    { id: "raw-sql", label: "Raw SQL", desc: "Consultas nativas parametrizadas (DB::select, DB::insert, DB::update, DB::delete)" },
  ],
  "aspnet-core": [
    { id: "ef-core", label: "Entity Framework Core", desc: "DbContext e DbSet<T> assíncronos" },
    { id: "dapper", label: "Dapper", desc: "Micro-ORM de alta performance sobre IDbConnection" },
    { id: "raw-sql", label: "Raw SQL (ADO.NET)", desc: "DbConnection e DbCommand parametrizados" },
  ],
};

export const currentOrmOptions = (backend: BackendFramework): OrmOption[] =>
  ormOptionsByBackend[backend];

export const tanstackPackageLabel = (fw: FrontendFramework): string =>
  fw === "sveltekit"
    ? "@tanstack/svelte-query"
    : fw === "nuxt"
      ? "@tanstack/vue-query"
      : "@tanstack/react-query";

export const nativeHttpClientLabel = (fw: FrontendFramework): string =>
  fw === "nuxt" ? "useFetch / $fetch (Nativo Nuxt)" : "Fetch API (Nativo Web / SSR)";
