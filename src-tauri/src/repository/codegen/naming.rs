use crate::model::migration::{DatabaseColumn, DatabaseTable};

pub fn clean_table_stem(raw: &str) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    for prefix in ["tb_", "tbl_", "t_"] {
        if let Some(stripped) = lower.strip_prefix(prefix) {
            if !stripped.is_empty() {
                return stripped.to_string();
            }
        }
    }
    lower
}

pub fn split_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in input.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
            if !current.is_empty() {
                words.push(current.to_ascii_lowercase());
                current.clear();
            }
        } else if ch.is_ascii_uppercase() && !current.is_empty() {
            words.push(current.to_ascii_lowercase());
            current.clear();
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

