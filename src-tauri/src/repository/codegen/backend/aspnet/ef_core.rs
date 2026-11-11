use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{editable_columns, to_pascal_case},
};
use std::collections::BTreeMap;

pub fn generate_ef_core_repo(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    pascal: &str,
    pk_cs: &str,
    table: &DatabaseTable,
) {
    let assign_fields = editable_columns(table)
        .iter()
        .map(|c| {
            let prop = to_pascal_case(&c.name);
            format!("        entity.{prop} = dto.{prop};")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let repo_src = format!(
        "using System;\n\
using System.Linq;\n\
using System.Threading.Tasks;\n\
using App.Features.{pascal}.Dtos;\n\
using Microsoft.EntityFrameworkCore;\n\n\
namespace App.Features.{pascal}.Repositories;\n\n\
public class {pascal}Repository(DbContext db)\n\
{{\n\
    public async Task<PageResponse<{pascal}ResponseDto>> FindAllAsync(int page = 0, int size = 10)\n\
    {{\n\
        var set = db.Set<{pascal}ResponseDto>();\n\
        var skip = Math.Max(0, page) * Math.Max(1, size);\n\
        var data = await set.AsNoTracking().Skip(skip).Take(size).ToListAsync();\n\
        var total = await set.LongCountAsync();\n\
        return new PageResponse<{pascal}ResponseDto>(data, total, page, size);\n\
    }}\n\n\
    public async Task<{pascal}ResponseDto> InsertAsync({pascal}RequestDto dto)\n\
    {{\n\
        var entity = new {pascal}ResponseDto();\n\
{assign_fields}\n\
        db.Set<{pascal}ResponseDto>().Add(entity);\n\
        await db.SaveChangesAsync();\n\
        return entity;\n\
    }}\n\n\
    public async Task<{pascal}ResponseDto> UpdateAsync({pk_cs} id, {pascal}RequestDto dto)\n\
    {{\n\
        var entity = await db.Set<{pascal}ResponseDto>().FindAsync(id)\n\
            ?? throw new InvalidOperationException(\"{pascal} not found\");\n\
{assign_fields}\n\
        await db.SaveChangesAsync();\n\
        return entity;\n\
    }}\n\n\
    public async Task DeleteAsync({pk_cs} id)\n\
    {{\n\
        var entity = await db.Set<{pascal}ResponseDto>().FindAsync(id);\n\
        if (entity is not null)\n\
        {{\n\
            db.Set<{pascal}ResponseDto>().Remove(entity);\n\
            await db.SaveChangesAsync();\n\
        }}\n\
    }}\n\
}}\n"
    );

    files.insert(
        format!("{base_dir}/Repositories/{pascal}Repository.cs"),
        repo_src,
    );
}

