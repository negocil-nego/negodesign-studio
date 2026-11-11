use crate::{model::migration::DatabaseTable, repository::codegen::naming::primary_key_column};
use std::collections::BTreeMap;

pub fn generate_query_builder_repo(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let pk_name = &pk.name;
    let table_name = &table.name;

    let repo_src = format!(
        "<?php\n\n\
namespace App\\Features\\{pascal}\\Repositories;\n\n\
use Illuminate\\Support\\Facades\\DB;\n\n\
class {pascal}Repository\n\
{{\n\
    public function findAll(int $page = 0, int $size = 10): array\n\
    {{\n\
        $offset = max(0, $page) * max(1, $size);\n\
        $data = DB::table('{table_name}')->offset($offset)->limit($size)->get();\n\
        $total = DB::table('{table_name}')->count();\n\
        return [\n\
            'data' => $data,\n\
            'total' => $total,\n\
            'page' => $page,\n\
            'size' => $size,\n\
        ];\n\
    }}\n\n\
    public function insert(array $data): mixed\n\
    {{\n\
        $id = DB::table('{table_name}')->insertGetId($data, '{pk_name}');\n\
        return DB::table('{table_name}')->where('{pk_name}', $id)->first();\n\
    }}\n\n\
    public function update(int|string $id, array $data): mixed\n\
    {{\n\
        DB::table('{table_name}')->where('{pk_name}', $id)->update($data);\n\
        return DB::table('{table_name}')->where('{pk_name}', $id)->first();\n\
    }}\n\n\
    public function delete(int|string $id): void\n\
    {{\n\
        DB::table('{table_name}')->where('{pk_name}', $id)->delete();\n\
    }}\n\
}}\n"
    );

    files.insert(
        format!("{base_dir}/Repositories/{pascal}Repository.php"),
        repo_src,
    );
}

