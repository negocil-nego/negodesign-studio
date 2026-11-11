pub mod drizzle;
pub mod prisma;
pub mod typeorm;

use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, primary_key_column, sql_to_ts_type, to_camel_case, to_kebab_case,
        to_pascal_case,
    },
};
use std::collections::BTreeMap;

pub fn generate(
    tables: &[DatabaseTable],
    orm_strategy: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();

    for table in tables {
        let kebab = to_kebab_case(&table.name);
        let pascal = to_pascal_case(&table.name);
        let pk = primary_key_column(table);
        let pk_ts = sql_to_ts_type(&pk.data_type);
        let base_dir = format!("src/features/{kebab}");

        files.insert(
            format!("{base_dir}/dto/{kebab}.dto.ts"),
            build_dto(&pascal, table),
        );

        match orm_strategy {
            "drizzle" => drizzle::generate_drizzle_repository(&mut files, &base_dir, &kebab, &pascal, table),
            "typeorm" => typeorm::generate_typeorm_repository(&mut files, &base_dir, &kebab, &pascal, table),
            _ => prisma::generate_prisma_repository(&mut files, &base_dir, &kebab, &pascal, table),
        }

        files.insert(
            format!("{base_dir}/{kebab}.service.ts"),
            build_service(&kebab, &pascal, pk_ts),
        );
        files.insert(
            format!("{base_dir}/{kebab}.controller.ts"),
            build_controller(&kebab, &pascal, pk_ts),
        );
        files.insert(
            format!("{base_dir}/{kebab}.module.ts"),
            build_module(&kebab, &pascal),
        );
    }

    Ok(files)
}

fn build_dto(pascal: &str, table: &DatabaseTable) -> String {
    let edit_fields = editable_columns(table)
        .iter()
        .map(|c| {
            let opt = if c.nullable { "?" } else { "" };
            format!("  {}{opt}: {};", to_camel_case(&c.name), sql_to_ts_type(&c.data_type))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let all_fields = table
        .columns
        .iter()
        .map(|c| {
            let opt = if c.nullable { "?" } else { "" };
            format!("  {}{opt}: {};", to_camel_case(&c.name), sql_to_ts_type(&c.data_type))
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "export interface {pascal}RequestDto {{\n{edit_fields}\n}}\n\n\
export interface {pascal}ResponseDto {{\n{all_fields}\n}}\n\n\
export interface PageResponse<T> {{\n  data: T[];\n  total: number;\n  page: number;\n  size: number;\n}}\n"
    )
}

fn build_service(kebab: &str, pascal: &str, pk_ts: &str) -> String {
    format!(
        "import {{ Injectable }} from '@nestjs/common';\n\
import {{ {pascal}Repository }} from './{kebab}.repository';\n\
import type {{ {pascal}RequestDto, {pascal}ResponseDto, PageResponse }} from './dto/{kebab}.dto';\n\n\
@Injectable()\n\
export class {pascal}Service {{\n\
  constructor(private readonly repository: {pascal}Repository) {{}}\n\n\
  findAll(page = 0, size = 10): Promise<PageResponse<{pascal}ResponseDto>> {{\n\
    return this.repository.findAll(page, size);\n\
  }}\n\n\
  insert(dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    return this.repository.insert(dto);\n\
  }}\n\n\
  update(id: {pk_ts}, dto: {pascal}RequestDto): Promise<{pascal}ResponseDto> {{\n\
    return this.repository.update(id, dto);\n\
  }}\n\n\
  delete(id: {pk_ts}): Promise<void> {{\n\
    return this.repository.delete(id);\n\
  }}\n\
}}\n"
    )
}

fn build_controller(kebab: &str, pascal: &str, pk_ts: &str) -> String {
    let id_cast = if pk_ts == "number" { "Number(id)" } else { "id" };
    format!(
        "import {{ Body, Controller, Delete, Get, Param, Post, Put, Query }} from '@nestjs/common';\n\
import {{ {pascal}Service }} from './{kebab}.service';\n\
import type {{ {pascal}RequestDto }} from './dto/{kebab}.dto';\n\n\
@Controller('api/{kebab}')\n\
export class {pascal}Controller {{\n\
  constructor(private readonly service: {pascal}Service) {{}}\n\n\
  @Get()\n\
  findAll(@Query('page') page = '0', @Query('size') size = '10') {{\n\
    return this.service.findAll(Number(page), Number(size));\n\
  }}\n\n\
  @Post()\n\
  insert(@Body() dto: {pascal}RequestDto) {{\n\
    return this.service.insert(dto);\n\
  }}\n\n\
  @Put(':id')\n\
  update(@Param('id') id: string, @Body() dto: {pascal}RequestDto) {{\n\
    return this.service.update({id_cast}, dto);\n\
  }}\n\n\
  @Delete(':id')\n\
  delete(@Param('id') id: string) {{\n\
    return this.service.delete({id_cast});\n\
  }}\n\
}}\n"
    )
}

fn build_module(kebab: &str, pascal: &str) -> String {
    format!(
        "import {{ Module }} from '@nestjs/common';\n\
import {{ {pascal}Controller }} from './{kebab}.controller';\n\
import {{ {pascal}Service }} from './{kebab}.service';\n\
import {{ {pascal}Repository }} from './{kebab}.repository';\n\n\
@Module({{\n\
  controllers: [{pascal}Controller],\n\
  providers: [{pascal}Service, {pascal}Repository],\n\
  exports: [{pascal}Service],\n\
}})\n\
export class {pascal}Module {{}}\n"
    )
}

