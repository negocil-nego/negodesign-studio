use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{editable_columns, primary_key_column},
};
use std::collections::BTreeMap;

pub fn generate_eloquent_repo(
    files: &mut BTreeMap<String, String>,
    base_dir: &str,
    pascal: &str,
    table: &DatabaseTable,
) {
    let pk = primary_key_column(table);
    let fillable = editable_columns(table)
        .iter()
        .map(|c| format!("'{}'", c.name))
        .collect::<Vec<_>>()
        .join(", ");

    let model_src = format!(
        "<?php\n\n\
namespace App\\Features\\{pascal}\\Models;\n\n\
use Illuminate\\Database\\Eloquent\\Model;\n\n\
class {pascal}Model extends Model\n\
{{\n\
    protected $table = '{table_name}';\n\
    protected $primaryKey = '{pk_name}';\n\
    public $timestamps = false;\n\
    protected $fillable = [{fillable}];\n\
}}\n",
        table_name = table.name,
        pk_name = pk.name
    );
    files.insert(format!("{base_dir}/Models/{pascal}Model.php"), model_src);

    let repo_src = format!(
        "<?php\n\n\
namespace App\\Features\\{pascal}\\Repositories;\n\n\
use App\\Features\\{pascal}\\Models\\{pascal}Model;\n\n\
class {pascal}Repository\n\
{{\n\
    public function findAll(int $page = 0, int $size = 10): array\n\
    {{\n\
        $offset = max(0, $page) * max(1, $size);\n\
        $data = {pascal}Model::query()->skip($offset)->take($size)->get();\n\
        $total = {pascal}Model::query()->count();\n\
        return [\n\
            'data' => $data,\n\
            'total' => $total,\n\
            'page' => $page,\n\
            'size' => $size,\n\
        ];\n\
    }}\n\n\
    public function insert(array $data): {pascal}Model\n\
    {{\n\
        return {pascal}Model::query()->create($data);\n\
    }}\n\n\
    public function update(int|string $id, array $data): {pascal}Model\n\
    {{\n\
        $record = {pascal}Model::query()->findOrFail($id);\n\
        $record->update($data);\n\
        return $record->fresh();\n\
    }}\n\n\
    public function delete(int|string $id): void\n\
    {{\n\
        {pascal}Model::query()->whereKey($id)->delete();\n\
    }}\n\
}}\n"
    );
    files.insert(
        format!("{base_dir}/Repositories/{pascal}Repository.php"),
        repo_src,
    );
}

