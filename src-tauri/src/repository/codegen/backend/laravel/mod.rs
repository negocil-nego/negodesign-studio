pub mod eloquent;
pub mod query_builder;
pub mod raw_sql;

use crate::{
    model::migration::DatabaseTable,
    repository::codegen::naming::{to_kebab_case, to_pascal_case},
};
use std::collections::BTreeMap;

pub fn generate(
    tables: &[DatabaseTable],
    orm_strategy: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();
    let mut api_routes = String::from("<?php\n\nuse Illuminate\\Support\\Facades\\Route;\n\n");

    for table in tables {
        let pascal = to_pascal_case(&table.name);
        let kebab = to_kebab_case(&table.name);
        let base_dir = format!("app/Features/{pascal}");

        match orm_strategy {
            "query-builder" => query_builder::generate_query_builder_repo(&mut files, &base_dir, &pascal, table),
            "raw-sql" => raw_sql::generate_raw_sql_repo(&mut files, &base_dir, &pascal, table),
            _ => eloquent::generate_eloquent_repo(&mut files, &base_dir, &pascal, table),
        }

        files.insert(
            format!("{base_dir}/Services/{pascal}Service.php"),
            build_service(&pascal),
        );
        files.insert(
            format!("{base_dir}/Controllers/{pascal}Controller.php"),
            build_controller(&pascal),
        );

        api_routes.push_str(&format!(
            "Route::prefix('{kebab}')->group(function () {{\n\
    Route::get('/', [\\App\\Features\\{pascal}\\Controllers\\{pascal}Controller::class, 'findAll']);\n\
    Route::post('/', [\\App\\Features\\{pascal}\\Controllers\\{pascal}Controller::class, 'insert']);\n\
    Route::put('/{{id}}', [\\App\\Features\\{pascal}\\Controllers\\{pascal}Controller::class, 'update']);\n\
    Route::delete('/{{id}}', [\\App\\Features\\{pascal}\\Controllers\\{pascal}Controller::class, 'delete']);\n\
}});\n\n"
        ));
    }

    files.insert("routes/negodesign_api.php".to_string(), api_routes);
    Ok(files)
}

fn build_service(pascal: &str) -> String {
    format!(
        "<?php\n\n\
namespace App\\Features\\{pascal}\\Services;\n\n\
use App\\Features\\{pascal}\\Repositories\\{pascal}Repository;\n\n\
class {pascal}Service\n\
{{\n\
    public function __construct(private readonly {pascal}Repository $repository) {{}}\n\n\
    public function findAll(int $page = 0, int $size = 10): array\n\
    {{\n\
        return $this->repository->findAll($page, $size);\n\
    }}\n\n\
    public function insert(array $data): mixed\n\
    {{\n\
        return $this->repository->insert($data);\n\
    }}\n\n\
    public function update(int|string $id, array $data): mixed\n\
    {{\n\
        return $this->repository->update($id, $data);\n\
    }}\n\n\
    public function delete(int|string $id): void\n\
    {{\n\
        $this->repository->delete($id);\n\
    }}\n\
}}\n"
    )
}

fn build_controller(pascal: &str) -> String {
    format!(
        "<?php\n\n\
namespace App\\Features\\{pascal}\\Controllers;\n\n\
use App\\Features\\{pascal}\\Services\\{pascal}Service;\n\
use Illuminate\\Http\\JsonResponse;\n\
use Illuminate\\Http\\Request;\n\n\
class {pascal}Controller\n\
{{\n\
    public function __construct(private readonly {pascal}Service $service) {{}}\n\n\
    public function findAll(Request $request): JsonResponse\n\
    {{\n\
        $page = (int) $request->query('page', 0);\n\
        $size = (int) $request->query('size', 10);\n\
        return response()->json($this->service->findAll($page, $size));\n\
    }}\n\n\
    public function insert(Request $request): JsonResponse\n\
    {{\n\
        return response()->json($this->service->insert($request->all()), 201);\n\
    }}\n\n\
    public function update(Request $request, int|string $id): JsonResponse\n\
    {{\n\
        return response()->json($this->service->update($id, $request->all()));\n\
    }}\n\n\
    public function delete(int|string $id): JsonResponse\n\
    {{\n\
        $this->service->delete($id);\n\
        return response()->json(null, 204);\n\
    }}\n\
}}\n"
    )
}

