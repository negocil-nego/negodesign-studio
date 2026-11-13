use super::*;

#[test]
fn classifies_restful_endpoints() {
    assert_eq!(
        classify_operation("GET", "/api/users", None),
        ("list".to_string(), "restful".to_string())
    );
    assert_eq!(
        classify_operation("GET", "/api/users/{id}", None),
        ("get".to_string(), "restful".to_string())
    );
    assert_eq!(
        classify_operation("POST", "/api/users", None),
        ("create".to_string(), "restful".to_string())
    );
    assert_eq!(
        classify_operation("PUT", "/api/users/{id}", None),
        ("update".to_string(), "restful".to_string())
    );
    assert_eq!(
        classify_operation("PATCH", "/api/users/{id}", None),
        ("update".to_string(), "restful".to_string())
    );
    assert_eq!(
        classify_operation("DELETE", "/api/users/{id}", None),
        ("delete".to_string(), "restful".to_string())
    );
}

#[test]
fn classifies_named_method_endpoints() {
    assert_eq!(
        classify_operation("GET", "/api/users/list", None),
        ("list".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("GET", "/api/users/all", None),
        ("list".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("POST", "/api/users/create", None),
        ("create".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("POST", "/api/users/add", None),
        ("create".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("PUT", "/api/users/update/{id}", None),
        ("update".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("PATCH", "/api/users/edit/{id}", None),
        ("update".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("PUT", "/api/users/save", None),
        ("update".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("DELETE", "/api/users/delete/{id}", None),
        ("delete".to_string(), "named".to_string())
    );
    assert_eq!(
        classify_operation("DELETE", "/api/users/remove/{id}", None),
        ("delete".to_string(), "named".to_string())
    );
}
