use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{editable_columns, primary_key_column, to_pascal_case},
};
use std::collections::BTreeMap;

pub fn generate_raw_sql_repo(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    pascal: &str,
    pk_cs: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_col = &pk.name;
    let table_name = &table.name;
    let edit_cols = editable_columns(table);

    let cols_sql = edit_cols
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let params_sql = edit_cols
        .iter()
        .map(|c| format!("@{}", to_pascal_case(&c.name)))
        .collect::<Vec<_>>()
        .join(", ");
    let set_sql = edit_cols
        .iter()
        .map(|c| format!("{} = @{}", c.name, to_pascal_case(&c.name)))
        .collect::<Vec<_>>()
        .join(", ");

    let param_lines = edit_cols
        .iter()
        .map(|c| {
            let prop = to_pascal_case(&c.name);
            format!(
                "        var p{prop} = cmd.CreateParameter(); p{prop}.ParameterName = \"@{prop}\"; p{prop}.Value = (object?)dto.{prop} ?? DBNull.Value; cmd.Parameters.Add(p{prop});"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let repo_src = format!(
        "using System;\n\
using System.Collections.Generic;\n\
using System.Data.Common;\n\
using System.Threading.Tasks;\n\
using App.Features.{pascal}.Dtos;\n\n\
namespace App.Features.{pascal}.Repositories;\n\n\
public class {pascal}Repository(DbConnection connection)\n\
{{\n\
    public async Task<PageResponse<{pascal}ResponseDto>> FindAllAsync(int page = 0, int size = 10)\n\
    {{\n\
        var offset = Math.Max(0, page) * Math.Max(1, size);\n\
        await using var cmd = connection.CreateCommand();\n\
        cmd.CommandText = $\"SELECT * FROM {table_name} LIMIT {{size}} OFFSET {{offset}}\";\n\
        var list = new List<{pascal}ResponseDto>();\n\
        await using (var reader = await cmd.ExecuteReaderAsync())\n\
        {{\n\
            while (await reader.ReadAsync())\n\
            {{\n\
                list.Add(new {pascal}ResponseDto());\n\
            }}\n\
        }}\n\
        await using var countCmd = connection.CreateCommand();\n\
        countCmd.CommandText = \"SELECT COUNT(*) FROM {table_name}\";\n\
        var total = Convert.ToInt64(await countCmd.ExecuteScalarAsync() ?? 0L);\n\
        return new PageResponse<{pascal}ResponseDto>(list, total, page, size);\n\
    }}\n\n\
    public async Task<{pascal}ResponseDto> InsertAsync({pascal}RequestDto dto)\n\
    {{\n\
        await using var cmd = connection.CreateCommand();\n\
        cmd.CommandText = \"INSERT INTO {table_name} ({cols_sql}) VALUES ({params_sql})\";\n\
{param_lines}\n\
        await cmd.ExecuteNonQueryAsync();\n\
        return new {pascal}ResponseDto();\n\
    }}\n\n\
    public async Task<{pascal}ResponseDto> UpdateAsync({pk_cs} id, {pascal}RequestDto dto)\n\
    {{\n\
        await using var cmd = connection.CreateCommand();\n\
        cmd.CommandText = \"UPDATE {table_name} SET {set_sql} WHERE {pk_col} = @Id\";\n\
{param_lines}\n\
        var pId = cmd.CreateParameter(); pId.ParameterName = \"@Id\"; pId.Value = id; cmd.Parameters.Add(pId);\n\
        await cmd.ExecuteNonQueryAsync();\n\
        return new {pascal}ResponseDto();\n\
    }}\n\n\
    public async Task DeleteAsync({pk_cs} id)\n\
    {{\n\
        await using var cmd = connection.CreateCommand();\n\
        cmd.CommandText = \"DELETE FROM {table_name} WHERE {pk_col} = @Id\";\n\
        var pId = cmd.CreateParameter(); pId.ParameterName = \"@Id\"; pId.Value = id; cmd.Parameters.Add(pId);\n\
        await cmd.ExecuteNonQueryAsync();\n\
    }}\n\
}}\n"
    );

    files.insert(
        format!("{base_dir}/Repositories/{pascal}Repository.cs"),
        repo_src,
    );
}

