use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{editable_columns, primary_key_column},
};
use std::collections::BTreeMap;

pub fn generate_raw_sql_repo(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_name = &pk.name;
    let table_name = &table.name;
    let edit_cols = editable_columns(table);

    let cols_sql = edit_cols
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let placeholders = edit_cols
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");
    let set_clause = edit_cols
        .iter()
        .map(|c| format!("{} = ?", c.name))
        .collect::<Vec<_>>()
        .join(", ");
    let data_extract = edit_cols
        .iter()
        .map(|c| format!("$data['{}'] ?? null", c.name))
        .collect::<Vec<_>>()
        .join(", ");

    let repo_src = format!(
        "<?php\n\n\
namespace App\\Features\\{pascal}\\Repositories;\n\n\
use Illuminate\\Support\\Facades\\DB;\n\n\
class {pascal}Repository\n\
{{\n\
    public function findAll(int $page = 0, int $size = 10): array\n\
    {{\n\
        $offset = max(0, $page) * max(1, $size);\n\
        $data = DB::select('SELECT * FROM {table_name} LIMIT ? OFFSET ?', [$size, $offset]);\n\
        $countRow = DB::selectOne('SELECT COUNT(*) as total FROM {table_name}');\n\
        return [\n\
            'data' => $data,\n\
            'total' => (int) ($countRow->total ?? 0),\n\
            'page' => $page,\n\
            'size' => $size,\n\
        ];\n\
    }}\n\n\
    public function insert(array $data): mixed\n\
    {{\n\
        DB::insert('INSERT INTO {table_name} ({cols_sql}) VALUES ({placeholders})', [{data_extract}]);\n\
        return DB::selectOne('SELECT * FROM {table_name} ORDER BY {pk_name} DESC LIMIT 1');\n\
    }}\n\n\
    public function update(int|string $id, array $data): mixed\n\
    {{\n\
        DB::update('UPDATE {table_name} SET {set_clause} WHERE {pk_name} = ?', [{data_extract}, $id]);\n\
        return DB::selectOne('SELECT * FROM {table_name} WHERE {pk_name} = ?', [$id]);\n\
    }}\n\n\
    public function delete(int|string $id): void\n\
    {{\n\
        DB::delete('DELETE FROM {table_name} WHERE {pk_name} = ?', [$id]);\n\
    }}\n\
}}\n"
    );

    files.insert(
        format!("{base_dir}/Repositories/{pascal}Repository.php"),
        repo_src,
    );
}

