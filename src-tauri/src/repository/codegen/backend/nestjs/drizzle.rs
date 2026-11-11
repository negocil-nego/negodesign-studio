use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{primary_key_column, sql_to_ts_type, to_camel_case},
};
use std::collections::BTreeMap;

pub fn generate_drizzle_repository(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    kebab: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let table_var = to_camel_case(&table.name);
    let pk = primary_key_column(table);
    let pk_camel = to_camel_case(&pk.name);
    let pk_ts = sql_to_ts_type(&pk.data_type);

    let src = format!(
        "import {{ Injectable }} from '@nestjs/common';\n\
import {{ eq, sql }} from 'drizzle-orm';\n\
import type {{ {pascal}RequestDto, {pascal}ResponseDto, PageResponse }} from './dto/{kebab}.dto';\n\n\
@Injectable()\n\
export class {pascal}Repository {{\n\
  constructor(private readonly db: any, private readonly schema: any) {{}}\n\n\
  async findAll(page = 0, size = 10): Promise<PageResponse<{pascal}ResponseDto>> {{\n\
    const offset = Math.max(0, page) * Math.max(1, size);\n\
    const tableRef = this.schema.{table_var};\n\
    const data = await this.db.select().from(tableRef).limit(size).offset(offset);\n\
    const [countRow] = await this.db.select({{ count: sql<number>`count(*)` }}).from(tableRef);\n\
    return {{ data, total: Number(countRow?.count ?? 0), page, size }};\n\
  }}\n\n\
  async insert(dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    const tableRef = this.schema.{table_var};\n\
    const [created] = await this.db.insert(tableRef).values(dto).returning();\n\
    return created;\n\
  }}\n\n\
  async update(id: {pk_ts}, dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    const tableRef = this.schema.{table_var};\n\
    const [updated] = await this.db\n\
      .update(tableRef)\n\
      .set(dto)\n\
      .where(eq(tableRef.{pk_camel}, id))\n\
      .returning();\n\
    return updated;\n\
  }}\n\n\
  async delete(id: {pk_ts}): Promise<void> {{\n\
    const tableRef = this.schema.{table_var};\n\
    await this.db.delete(tableRef).where(eq(tableRef.{pk_camel}, id));\n\
  }}\n\
}}\n"
    );

    files.insert(format!("{base_dir}/{kebab}.repository.ts"), src);
}

