use crate::model::migration::{DatabaseColumn, DatabaseTable};

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
        // para que a dedupe os una antes de gerar as classes.
        assert_eq!(to_snake_case("Address"), to_snake_case("address"));
        assert_eq!(
            to_snake_case("AgentAddress"),
            to_snake_case("agent_address")
        );
    }
}

