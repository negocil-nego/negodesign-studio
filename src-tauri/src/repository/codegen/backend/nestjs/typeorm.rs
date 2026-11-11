use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{primary_key_column, sql_to_ts_type, to_camel_case},
};
use std::collections::BTreeMap;

pub fn generate_typeorm_repository(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    kebab: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_camel = to_camel_case(&pk.name);
    let pk_ts = sql_to_ts_type(&pk.data_type);

    let entity_cols = table
        .columns
        .iter()
        .map(|c| {
            let camel = to_camel_case(&c.name);
            let ts = sql_to_ts_type(&c.data_type);
            if c.primary_key {
                format!("  @PrimaryGeneratedColumn({{ name: '{}' }})\n  {camel}: {ts};", c.name)
            } else {
                format!(
                    "  @Column({{ name: '{}', nullable: {} }})\n  {camel}: {ts};",
                    c.name, c.nullable
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let entity_src = format!(
        "import {{ Column, Entity, PrimaryGeneratedColumn }} from 'typeorm';\n\n\
@Entity('{table_name}')\n\
export class {pascal}Entity {{\n\
{entity_cols}\n\
}}\n",
        table_name = table.name
    );
    files.insert(
        format!("{base_dir}/entities/{kebab}.entity.ts"),
        entity_src,
    );

    let repo_src = format!(
        "import {{ Injectable }} from '@nestjs/common';\n\
import {{ InjectRepository }} from '@nestjs/typeorm';\n\
import {{ Repository }} from 'typeorm';\n\
import {{ {pascal}Entity }} from './entities/{kebab}.entity';\n\
import type {{ {pascal}RequestDto, {pascal}ResponseDto, PageResponse }} from './dto/{kebab}.dto';\n\n\
@Injectable()\n\
export class {pascal}Repository {{\n\
  constructor(\n\
    @InjectRepository({pascal}Entity)\n\
    private readonly ormRepo: Repository<{pascal}Entity>,\n\
  ) {{}}\n\n\
  async findAll(page = 0, size = 10): Promise<PageResponse<{pascal}ResponseDto>> {{\n\
    const skip = Math.max(0, page) * Math.max(1, size);\n\
    const [data, total] = await this.ormRepo.findAndCount({{ skip, take: size }});\n\
    return {{ data, total, page, size }};\n\
  }}\n\n\
  async insert(dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    const entity = this.ormRepo.create(dto as Partial<{pascal}Entity>);\n\
    return this.ormRepo.save(entity);\n\
  }}\n\n\
  async update(id: {pk_ts}, dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    await this.ormRepo.update({{ {pk_camel}: id }} as any, dto as any);\n\
    return this.ormRepo.findOneByOrFail({{ {pk_camel}: id }} as any);\n\
  }}\n\n\
  async delete(id: {pk_ts}): Promise<void> {{\n\
    await this.ormRepo.delete({{ {pk_camel}: id }} as any);\n\
  }}\n\
}}\n"
    );

    files.insert(format!("{base_dir}/{kebab}.repository.ts"), repo_src);
}

