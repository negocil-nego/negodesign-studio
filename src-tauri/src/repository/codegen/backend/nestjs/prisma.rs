use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{primary_key_column, sql_to_ts_type, to_camel_case},
};
use std::collections::BTreeMap;

pub fn generate_prisma_repository(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    kebab: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let model_camel = to_camel_case(&table.name);
    let pk = primary_key_column(table);
    let pk_camel = to_camel_case(&pk.name);
    let pk_ts = sql_to_ts_type(&pk.data_type);

    let src = format!(
        "import {{ Injectable }} from '@nestjs/common';\n\
import type {{ {pascal}RequestDto, {pascal}ResponseDto, PageResponse }} from './dto/{kebab}.dto';\n\n\
@Injectable()\n\
export class {pascal}Repository {{\n\
  constructor(private readonly prisma: any) {{}}\n\n\
  async findAll(page = 0, size = 10): Promise<PageResponse<{pascal}ResponseDto>> {{\n\
    const skip = Math.max(0, page) * Math.max(1, size);\n\
    const [data, total] = await Promise.all([\n\
      this.prisma.{model_camel}.findMany({{ skip, take: size }}),\n\
      this.prisma.{model_camel}.count(),\n\
    ]);\n\
    return {{ data, total, page, size }};\n\
  }}\n\n\
  async insert(dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    return this.prisma.{model_camel}.create({{ data: dto }});\n\
  }}\n\n\
  async update(id: {pk_ts}, dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    return this.prisma.{model_camel}.update({{\n\
      where: {{ {pk_camel}: id }},\n\
      data: dto,\n\
    }});\n\
  }}\n\n\
  async delete(id: {pk_ts}): Promise<void> {{\n\
    await this.prisma.{model_camel}.delete({{ where: {{ {pk_camel}: id }} }});\n\
  }}\n\
}}\n"
    );

    files.insert(format!("{base_dir}/{kebab}.repository.ts"), src);
}

