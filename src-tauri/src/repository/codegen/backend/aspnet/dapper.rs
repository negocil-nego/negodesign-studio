use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{editable_columns, primary_key_column, to_pascal_case},
};
use std::collections::BTreeMap;

pub fn generate_dapper_repo(
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

    let repo_src = format!(
        "using System;\n\
using System.Data;\n\
using System.Linq;\n\
using System.Threading.Tasks;\n\
using App.Features.{pascal}.Dtos;\n\
using Dapper;\n\n\
namespace App.Features.{pascal}.Repositories;\n\n\
public class {pascal}Repository(IDbConnection connection)\n\
{{\n\
    public async Task<PageResponse<{pascal}ResponseDto>> FindAllAsync(int page = 0, int size = 10)\n\
    {{\n\
        var offset = Math.Max(0, page) * Math.Max(1, size);\n\
        var data = (await connection.QueryAsync<{pascal}ResponseDto>(\n\
            \"SELECT * FROM {table_name} LIMIT @Size OFFSET @Offset\",\n\
            new {{ Size = size, Offset = offset }}\n\
        )).ToList();\n\
        var total = await connection.ExecuteScalarAsync<long>(\"SELECT COUNT(*) FROM {table_name}\");\n\
        return new PageResponse<{pascal}ResponseDto>(data, total, page, size);\n\
    }}\n\n\
    public async Task<{pascal}ResponseDto> InsertAsync({pascal}RequestDto dto)\n\
    {{\n\
        await connection.ExecuteAsync(\n\
            \"INSERT INTO {table_name} ({cols_sql}) VALUES ({params_sql})\",\n\
            dto\n\
        );\n\
        return await connection.QuerySingleAsync<{pascal}ResponseDto>(\n\
            \"SELECT * FROM {table_name} ORDER BY {pk_col} DESC LIMIT 1\"\n\
        );\n\
    }}\n\n\
    public async Task<{pascal}ResponseDto> UpdateAsync({pk_cs} id, {pascal}RequestDto dto)\n\
    {{\n\
        var parameters = new DynamicParameters(dto);\n\
        parameters.Add(\"Id\", id);\n\
        await connection.ExecuteAsync(\n\
            \"UPDATE {table_name} SET {set_sql} WHERE {pk_col} = @Id\",\n\
            parameters\n\
        );\n\
        return await connection.QuerySingleAsync<{pascal}ResponseDto>(\n\
            \"SELECT * FROM {table_name} WHERE {pk_col} = @Id\",\n\
            new {{ Id = id }}\n\
        );\n\
    }}\n\n\
    public async Task DeleteAsync({pk_cs} id)\n\
    {{\n\
        await connection.ExecuteAsync(\n\
            \"DELETE FROM {table_name} WHERE {pk_col} = @Id\",\n\
            new {{ Id = id }}\n\
        );\n\
    }}\n\
}}\n"
    );

    files.insert(
        format!("{base_dir}/Repositories/{pascal}Repository.cs"),
        repo_src,
    );
}

