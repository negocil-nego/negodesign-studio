pub mod dapper;
pub mod ef_core;
pub mod raw_sql;

use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{
        editable_columns, primary_key_column, sql_to_csharp_type, to_kebab_case, to_pascal_case,
    },
};
use std::collections::BTreeMap;

pub fn generate(
    tables: &[DatabaseTable],
    orm_strategy: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();

    for table in tables {
        let pascal = to_pascal_case(&table.name);
        let kebab = to_kebab_case(&table.name);
        let pk = primary_key_column(table);
        let pk_cs = sql_to_csharp_type(&pk.data_type, false);
        let base_dir = format!("Features/{pascal}");

        files.insert(
            format!("{base_dir}/Dtos/{pascal}Dtos.cs"),
            build_dtos(&pascal, table),
        );

        match orm_strategy {
            "dapper" => dapper::generate_dapper_repo(&mut files, &base_dir, &pascal, &pk_cs, table),
            "raw-sql" => raw_sql::generate_raw_sql_repo(&mut files, &base_dir, &pascal, &pk_cs, table),
            _ => ef_core::generate_ef_core_repo(&mut files, &base_dir, &pascal, &pk_cs, table),
        }

        files.insert(
            format!("{base_dir}/Services/{pascal}Service.cs"),
            build_service(&pascal, &pk_cs),
        );
        files.insert(
            format!("{base_dir}/Controllers/{pascal}Controller.cs"),
            build_controller(&pascal, &kebab, &pk_cs),
        );
    }

    Ok(files)
}

fn build_dtos(pascal: &str, table: &DatabaseTable) -> String {
    let req_props = editable_columns(table)
        .iter()
        .map(|c| {
            format!(
                "    public {} {} {{ get; set; }}",
                sql_to_csharp_type(&c.data_type, c.nullable),
                to_pascal_case(&c.name)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let res_props = table
        .columns
        .iter()
        .map(|c| {
            format!(
                "    public {} {} {{ get; set; }}",
                sql_to_csharp_type(&c.data_type, c.nullable),
                to_pascal_case(&c.name)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "using System;\nusing System.Collections.Generic;\n\n\
namespace App.Features.{pascal}.Dtos;\n\n\
public class {pascal}RequestDto\n{{\n{req_props}\n}}\n\n\
public class {pascal}ResponseDto\n{{\n{res_props}\n}}\n\n\
public record PageResponse<T>(IReadOnlyList<T> Data, long Total, int Page, int Size);\n"
    )
}

fn build_service(pascal: &str, pk_cs: &str) -> String {
    format!(
        "using System.Threading.Tasks;\n\
using App.Features.{pascal}.Dtos;\n\
using App.Features.{pascal}.Repositories;\n\n\
namespace App.Features.{pascal}.Services;\n\n\
public class {pascal}Service({pascal}Repository repository)\n\
{{\n\
    public Task<PageResponse<{pascal}ResponseDto>> FindAllAsync(int page = 0, int size = 10) =>\n\
        repository.FindAllAsync(page, size);\n\n\
    public Task<{pascal}ResponseDto> InsertAsync({pascal}RequestDto dto) =>\n\
        repository.InsertAsync(dto);\n\n\
    public Task<{pascal}ResponseDto> UpdateAsync({pk_cs} id, {pascal}RequestDto dto) =>\n\
        repository.UpdateAsync(id, dto);\n\n\
    public Task DeleteAsync({pk_cs} id) =>\n\
        repository.DeleteAsync(id);\n\
}}\n"
    )
}

fn build_controller(pascal: &str, kebab: &str, pk_cs: &str) -> String {
    format!(
        "using System.Threading.Tasks;\n\
using App.Features.{pascal}.Dtos;\n\
using App.Features.{pascal}.Services;\n\
using Microsoft.AspNetCore.Mvc;\n\n\
namespace App.Features.{pascal}.Controllers;\n\n\
[ApiController]\n\
[Route(\"api/{kebab}\")]\n\
public class {pascal}Controller({pascal}Service service) : ControllerBase\n\
{{\n\
    [HttpGet]\n\
    public Task<PageResponse<{pascal}ResponseDto>> FindAll([FromQuery] int page = 0, [FromQuery] int size = 10) =>\n\
        service.FindAllAsync(page, size);\n\n\
    [HttpPost]\n\
    public Task<{pascal}ResponseDto> Insert([FromBody] {pascal}RequestDto dto) =>\n\
        service.InsertAsync(dto);\n\n\
    [HttpPut(\"{{id}}\")]\n\
    public Task<{pascal}ResponseDto> Update([FromRoute] {pk_cs} id, [FromBody] {pascal}RequestDto dto) =>\n\
        service.UpdateAsync(id, dto);\n\n\
    [HttpDelete(\"{{id}}\")]\n\
    public async Task<IActionResult> Delete([FromRoute] {pk_cs} id)\n\
    {{\n\
        await service.DeleteAsync(id);\n\
        return NoContent();\n\
    }}\n\
}}\n"
    )
}

