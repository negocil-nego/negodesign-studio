use crate::model::migration::{DatabaseColumn, DatabaseTable};
use std::collections::{BTreeMap, BTreeSet};

pub fn clean_table_stem(raw: &str) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    for prefix in ["tb_", "tbl_", "t_"] {
        if let Some(stripped) = lower.strip_prefix(prefix) {
            if !stripped.is_empty() {
                // Preserva o caso original do resto do nome para que
                // split_words ainda consiga detectar fronteiras camelCase
                // (ex.: "TbAnnualBilling" → "AnnualBilling").
                return trimmed.get(prefix.len()..).unwrap_or(trimmed).to_string();
            }
        }
    }
    trimmed.to_string()
}

pub fn split_words(input: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = input.chars().collect();
    for (index, &ch) in chars.iter().enumerate() {
        if ch == '_' || ch == '-' || ch == ' ' {
            if !current.is_empty() {
                words.push(current.to_ascii_lowercase());
                current.clear();
            }
        } else if ch.is_ascii_uppercase() {
            let prev_is_upper = current
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_uppercase());
            let next_is_lower = chars
                .get(index + 1)
                .is_some_and(|c| c.is_ascii_lowercase());
            // Fronteira quando a maiúscula segue uma minúscula ("AgentAddress"),
            // ou quando termina um acrónimo seguido de palavra ("HTMLParser").
            // "FAQ" e "IPv4" mantêm-se numa só palavra.
            if !current.is_empty()
                && (!prev_is_upper || (next_is_lower && current.chars().count() > 1))
            {
                words.push(current.to_ascii_lowercase());
                current.clear();
            }
            current.push(ch);
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        words.push(current.to_ascii_lowercase());
    }
    words
}

pub fn to_pascal_case(input: &str) -> String {
    split_words(&clean_table_stem(input))
        .into_iter()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

pub fn to_camel_case(input: &str) -> String {
    let pascal = to_pascal_case(input);
    let mut chars = pascal.chars();
    match chars.next() {
        Some(first) => first.to_ascii_lowercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

pub fn to_snake_case(input: &str) -> String {
    split_words(&clean_table_stem(input)).join("_")
}

pub fn to_kebab_case(input: &str) -> String {
    split_words(&clean_table_stem(input)).join("-")
}

pub fn to_title_label(input: &str) -> String {
    split_words(&clean_table_stem(input))
        .into_iter()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Agrupa kebab-cases pelo primeiro segmento — as features relacionadas ficam
/// todas na mesma pasta/package: `agent`, `agent-address`, `agent-assignment`
/// → chave `agent`; `company-users` → `company`.
///
/// Quando nenhuma outra entrada partilha o primeiro segmento, a chave é o
/// próprio kebab (ex.: `prisma-migrations`, `messages`).
///
/// Se existirem **os dois** grupos de um par singular/plural, unificam-se no
/// singular: `roles` → `role`, `needs` → `need`, `addresses` → `address`
/// (assim nunca se criam `need/` e `needs/` ou `role/` e `roles/` em paralelo).
/// A unificação só acontece quando ambos já existem — nunca se parte uma
/// palavra que esteja sozinha (`messages` fica `messages`).
///
/// É a mesma regra usada pelo frontend (`src/lib/features/<grupo>/`) e pelo
/// backend Spring (`modules.<grupo>` / `feature.<grupo>`), para os nomes
/// ficarem alinhados dos dois lados.
pub fn feature_group_keys(kebabs: &[String]) -> BTreeMap<String, String> {
    let mut first_segment_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for kebab in kebabs {
        let first = kebab.split('-').next().unwrap_or(kebab);
        *first_segment_counts.entry(first).or_insert(0) += 1;
    }

    let mut keys: BTreeMap<String, String> = kebabs
        .iter()
        .map(|kebab| {
            let first = kebab.split('-').next().unwrap_or(kebab);
            let key = if first_segment_counts.get(first).copied().unwrap_or(0) > 1 {
                first.to_string()
            } else {
                kebab.clone()
            };
            (kebab.clone(), key)
        })
        .collect();

    // Unifica pares singular/plural quando ambos os grupos existem.
    // Tenta primeiro a forma em "-es" (addresses → address) e depois a em
    // "-s" (roles → role, needs → need).
    let distinct: BTreeSet<String> = keys.values().cloned().collect();
    let mut remap: BTreeMap<String, String> = BTreeMap::new();
    for key in &distinct {
        for candidate in [key.strip_suffix("es"), key.strip_suffix('s')]
            .into_iter()
            .flatten()
        {
            if !candidate.is_empty() && distinct.contains(candidate) {
                remap.insert(key.clone(), candidate.to_string());
                break;
            }
        }
    }

        // Resolve encadeamentos até ponto fixo ("xies" → "xie" → …).
        // As arestas encurtam sempre o nome, por isso o ciclo termina.
    loop {
        let snapshot = remap.clone();
        let mut changed = false;
        for (from, to) in remap.iter_mut() {
            if let Some(next) = snapshot.get(to) {
                if next != from {
                    *to = next.clone();
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    for key in keys.values_mut() {
        if let Some(target) = remap.get(key) {
            *key = target.clone();
        }
    }
    keys
}

pub fn primary_key_column(table: &DatabaseTable) -> DatabaseColumn {
    table
        .columns
        .iter()
        .find(|c| c.primary_key)
        .or_else(|| table.columns.first())
        .cloned()
        .unwrap_or(DatabaseColumn {
            name: "id".to_string(),
            data_type: "bigint".to_string(),
            nullable: false,
            primary_key: true,
        })
}

pub fn editable_columns(table: &DatabaseTable) -> Vec<&DatabaseColumn> {
    table
        .columns
        .iter()
        .filter(|c| {
            let lower = c.name.to_ascii_lowercase();
            !c.primary_key
                && !matches!(
                    lower.as_str(),
                    "created_at" | "updated_at" | "deleted_at"
                )
        })
        .collect()
}

pub fn sql_to_java_type(data_type: &str) -> &'static str {
    let t = data_type.to_ascii_lowercase();
    if t.contains("bigint") || t.contains("int8") || t.contains("bigserial") {
        "Long"
    } else if t.contains("int") || t.contains("serial") {
        "Integer"
    } else if t.contains("dec") || t.contains("num") || t.contains("money") {
        "java.math.BigDecimal"
    } else if t.contains("float") || t.contains("double") || t.contains("real") {
        "Double"
    } else if t.contains("bool") || t.contains("bit") {
        "Boolean"
    } else if t.contains("timestamp") || t.contains("datetime") {
        "java.time.LocalDateTime"
    } else if t.contains("date") {
        "java.time.LocalDate"
    } else if t.contains("uuid") {
        "java.util.UUID"
    } else {
        "String"
    }
}

pub fn sql_to_ts_type(data_type: &str) -> &'static str {
    let t = data_type.to_ascii_lowercase();
    if t.contains("int")
        || t.contains("serial")
        || t.contains("dec")
        || t.contains("num")
        || t.contains("float")
        || t.contains("double")
        || t.contains("real")
    {
        "number"
    } else if t.contains("bool") || t.contains("bit") {
        "boolean"
    } else {
        "string"
    }
}

pub fn sql_to_csharp_type(data_type: &str, nullable: bool) -> String {
    let t = data_type.to_ascii_lowercase();
    let base = if t.contains("bigint") || t.contains("int8") {
        "long"
    } else if t.contains("int") || t.contains("serial") {
        "int"
    } else if t.contains("dec") || t.contains("num") || t.contains("money") {
        "decimal"
    } else if t.contains("float") || t.contains("double") || t.contains("real") {
        "double"
    } else if t.contains("bool") || t.contains("bit") {
        "bool"
    } else if t.contains("date") || t.contains("time") {
        "DateTime"
    } else if t.contains("uuid") {
        "Guid"
    } else {
        return if nullable {
            "string?".to_string()
        } else {
            "string".to_string()
        };
    };
    if nullable {
        format!("{base}?")
    } else {
        base.to_string()
    }
}

pub fn is_badge_column(col: &DatabaseColumn) -> bool {
    let name = col.name.to_ascii_lowercase();
    let dtype = col.data_type.to_ascii_lowercase();
    dtype.contains("bool")
        || name.contains("status")
        || name.contains("state")
        || name.contains("role")
        || name.contains("type")
        || name.contains("active")
}

pub fn infer_form_component(col: &DatabaseColumn) -> (&'static str, &'static str) {
    let name = col.name.to_ascii_lowercase();
    let dtype = col.data_type.to_ascii_lowercase();
    if name.contains("email") {
        ("input-email", "email")
    } else if name.contains("pass") {
        ("input-password", "password")
    } else if name.contains("phone") || name.contains("tel") {
        ("input-phone", "tel")
    } else if name.contains("desc") || name.contains("bio") || name.contains("note") || dtype == "text" {
        ("textarea", "text")
    } else if dtype.contains("bool") {
        ("toggle", "checkbox")
    } else if dtype.contains("date") || dtype.contains("time") {
        ("input-date", "date")
    } else if dtype.contains("int")
        || dtype.contains("dec")
        || dtype.contains("num")
        || dtype.contains("float")
        || dtype.contains("double")
    {
        ("input-number", "number")
    } else {
        ("input-text", "text")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_table_stem_preserves_original_case() {
        assert_eq!(clean_table_stem("AnnualBilling"), "AnnualBilling");
        assert_eq!(clean_table_stem("Tb_AnnualBilling"), "AnnualBilling");
        assert_eq!(clean_table_stem("tb_roles"), "roles");
        assert_eq!(clean_table_stem("  address  "), "address");
    }

    #[test]
    fn snake_case_splits_camel_and_keeps_snake() {
        assert_eq!(to_snake_case("AgentAddress"), "agent_address");
        assert_eq!(to_snake_case("agent_address"), "agent_address");
        assert_eq!(to_snake_case("LogsActions"), "logs_actions");
        assert_eq!(to_snake_case("AnnualBilling"), "annual_billing");
        assert_eq!(to_snake_case("address"), "address");
        assert_eq!(to_snake_case("_prisma_migrations"), "prisma_migrations");
        assert_eq!(to_snake_case("TB_ROLES"), "roles");
    }

    #[test]
    fn snake_case_keeps_acronyms_together() {
        assert_eq!(to_snake_case("FAQ"), "faq");
        assert_eq!(to_snake_case("FaqItem"), "faq_item");
        assert_eq!(to_snake_case("IPv4"), "ipv4");
        assert_eq!(to_snake_case("HTMLParser"), "html_parser");
    }

    #[test]
    fn pascal_case_round_trips() {
        assert_eq!(to_pascal_case("AnnualBilling"), "AnnualBilling");
        assert_eq!(to_pascal_case("address"), "Address");
        assert_eq!(to_pascal_case("agent_address"), "AgentAddress");
        assert_eq!(to_camel_case("agent_address"), "agentAddress");
    }

    #[test]
    fn case_variant_tables_share_the_same_key() {
        // "Address" e "address" têm de colidir na mesma chave canónica
        // para que a dedupe os unir antes de gerar as classes.
        assert_eq!(to_snake_case("Address"), to_snake_case("address"));
        assert_eq!(
            to_snake_case("AgentAddress"),
            to_snake_case("agent_address")
        );
    }

    #[test]
    fn feature_group_keys_group_related_tables() {
        let kebabs: Vec<String> = [
            "agent",
            "agent-address",
            "agent-assignment",
            "annual-billing",
            "annual-billing-tier",
            "company",
            "company-users",
            "messages",
            "prisma-migrations",
        ]
        .iter()
        .map(|value| value.to_string())
        .collect();

        let groups = feature_group_keys(&kebabs);

        // Relacionadas ficam todas na mesma feature
        assert_eq!(groups["agent"], "agent");
        assert_eq!(groups["agent-address"], "agent");
        assert_eq!(groups["agent-assignment"], "agent");
        assert_eq!(groups["annual-billing"], "annual");
        assert_eq!(groups["annual-billing-tier"], "annual");
        assert_eq!(groups["company"], "company");
        assert_eq!(groups["company-users"], "company");

        // Sem partilhas do primeiro segmento, mantêm o kebab completo
        assert_eq!(groups["messages"], "messages");
        assert_eq!(groups["prisma-migrations"], "prisma-migrations");
    }

    #[test]
    fn feature_group_keys_unifies_plural_and_singular() {
        let kebabs: Vec<String> = [
            "need",
            "need-read",
            "need-response",
            "needs",
            "role",
            "roles",
            "address",
            "addresses",
            "messages",
            "company",
            "product",
            "products",
        ]
        .iter()
        .map(|value| value.to_string())
        .collect();

        let groups = feature_group_keys(&kebabs);

        // Pares singular/plural unificam-se no singular — nunca pastas
        // "need/" e "needs/" ou "role/" e "roles/" em paralelo.
        assert_eq!(groups["need"], "need");
        assert_eq!(groups["needs"], "need");
        assert_eq!(groups["need-read"], "need");
        assert_eq!(groups["need-response"], "need");
        assert_eq!(groups["role"], "role");
        assert_eq!(groups["roles"], "role");
        assert_eq!(groups["address"], "address");
        assert_eq!(groups["addresses"], "address");
        assert_eq!(groups["product"], "product");
        assert_eq!(groups["products"], "product");

        // Sem contraparte singular, mantém o nome original
        assert_eq!(groups["messages"], "messages");
        assert_eq!(groups["company"], "company");
    }
}

