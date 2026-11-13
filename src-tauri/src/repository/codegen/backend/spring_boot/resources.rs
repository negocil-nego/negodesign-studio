use crate::model::{
    codegen::{DatabaseEngine, FrontendFramework},
    migration::DatabaseConnection,
};
use std::collections::BTreeMap;

pub const RSA_PRIVATE_KEY_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCbSNTvvh0vJYE3
5DXsxxI2mUOf2XBT35CHhXF0V7C0gyC8IonYNGf1OKpmyeV3DQn/21Ff+g6jIzVp
Es+MA0HpHE+xUkjP7pUyvDhETKK5pzM83gM8QiotOm6VuqIUzjHztEv9y7ftVRFq
E3alCcy6KItZVOo531A2eq4wa8gfSMXUeAvZtrzTRCKXZhjHuTwjI/AUsjJNcbap
l9hXMct3qIilnQ4i1Pf1U938A52jxGC2h+pQWDt/SUNp4bK4JMxj2pRbsy1P5h+Y
44JaY4cBb9YiLUQ9f77axvTpyLp8i+a5VmYCXSwoKI3BFDBa+WUel5N4062I/b5T
2vsXIbAbAgMBAAECggEABBNNQ12B9pLNaWIb6DLxT22GUNOsFwdvC5988si6YoJu
gYGGjJ4TfGQYVMpHs3AsKwLeh95QNAfghvlvaQVM3YlYrhaywoJIeCRodaJtQuIY
fih1fUdfRACclmFnj5Q5y2OX+vk1wozRAC+ylXJEZT5bKeUIoKwztrpRiAPMJMYB
s2sEsMUuALokCIoNxct1CTvVa5P7bqVf8oIQwSwmjSnGZK8kxnRvCK7N2UwV+Tio
y52xvzx3NYsOA6GNocFf96+oywpK256TfG/CS9fJtoz7jNHt+75WFEYWAkuicN1B
q8WO9BAQf1BibPJ/4CKf0RwjL89d9tO4IEbKL+d7AQKBgQDLgVJPIQXknlhR9S/N
gnk7k5DaY3QoaiO+HM9TXkeL2y7nl18O+P4+Pqx1luommU4cmsH991MY+dhME9R/
UrC4NZa9NF4hTt9AcKfCWDqLWTA4w1b+hK7Yhe1yc5Xpf6NlfZcAMF8of9qQ9foC
O/VSBHOqC1/VMslWF8mUTkWh6wKBgQDDVzUzOjLQLvJAKQuuHvbyw9X20l+GhR6N
Z5Jz0hUnmguOO9rznSgREEcuCSC3vUUltGhAJ0bax8iMXnNprQsKf0m4/AICygqT
sd8QcJWzRZ1d/sP+oFvwqMTtkVcF5lR7IHnuglyXZX6MEwB1K73UoUqQune/mdLs
/4D13NpukQKBgCARPb4YTwxqYzJu91xj3eSkaSPwngIsT3uKoKTba1y/ahG2SZVb
aPy39YcGA9mXOLbLMBxr151M6Wd7jHWPuycb0qf304leG1HVLqoyKu+AW1fmeBPd
JowZ9oOdcRb9NOsuMhgmhygjlXZYPFAT0KFe8DbwWeaHfTBiyTHkb5aHAoGAQQf1
ZBqJ9BjM+WMwykTwvZ1IzCT5PFVw+G50qS5VcJuuISn4GIZHYnbi24aaZ9ERBh5r
XoA07Hz6XRDMp2++bz41EuqC2N7eapEay+RPUiFo5HP+V/SW5yWoedaa/BHUKjGD
pk2r+ZMGcCebTWbEidcwdcI+P4E8YT4mdrOR3KECgYANO7+97y4mjor/AdOyqjKB
tiNrrO49brMAy6FcKL1O0u3U1IFuQ/B/RoKdCU5+0lWKmsMzVYSiTfQTXS/fAQW4
EuVI/WpOllh5b7+bxSRWWCX4IQ7eSdDlaU5LnEsfqZ3QSqtWvm72h14ynprlAF3h
588KMuxysBoeBcAg5TVK/g==
-----END PRIVATE KEY-----
"#;

pub const RSA_PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAm0jU774dLyWBN+Q17McS
NplDn9lwU9+Qh4VxdFewtIMgvCKJ2DRn9TiqZsnldw0J/9tRX/oOoyM1aRLPjANB
6RxPsVJIz+6VMrw4REyiuaczPN4DPEIqLTpulbqiFM4x87RL/cu37VURahN2pQnM
uiiLWVTqOd9QNnquMGvIH0jF1HgL2ba800Qil2YYx7k8IyPwFLIyTXG2qZfYVzHL
d6iIpZ0OItT39VPd/AOdo8RgtofqUFg7f0lDaeGyuCTMY9qUW7MtT+YfmOOCWmOH
AW/WIi1EPX++2sb06ci6fIvmuVZmAl0sKCiNwRQwWvllHpeTeNOtiP2+U9r7FyGw
GwIDAQAB
-----END PUBLIC KEY-----
"#;

pub fn generate_resources(
    files: &mut BTreeMap<String, String>,
    app_name: &str,
    db_conn: Option<&DatabaseConnection>,
    db_engine: &DatabaseEngine,
    frontend_framework: &FrontendFramework,
) {
    let cors_origin = match frontend_framework {
        FrontendFramework::Sveltekit => "http://localhost:5173",
        FrontendFramework::Nuxt => "http://localhost:3000",
        FrontendFramework::Nextjs => "http://localhost:3000",
    };

    let (db_url, db_user, db_pass) = if let Some(conn) = db_conn {
        let driver_lower = conn.driver.to_ascii_lowercase();
        let url = if driver_lower == "mysql" {
            format!(
                "jdbc:mysql://{}:{}/{}?useSSL=false&serverTimezone=UTC",
                conn.host, conn.port, conn.database
            )
        } else {
            format!(
                "jdbc:postgresql://{}:{}/{}",
                conn.host, conn.port, conn.database
            )
        };
        (url, conn.user.clone(), conn.password.clone())
    } else {
        match db_engine {
            DatabaseEngine::Mysql => (
                "jdbc:mysql://localhost:3306/negodesign?useSSL=false&serverTimezone=UTC".to_string(),
                "root".to_string(),
                "root".to_string(),
            ),
            DatabaseEngine::Postgres => (
                "jdbc:postgresql://localhost:5432/negodesign".to_string(),
                "postgres".to_string(),
                "postgres".to_string(),
            ),
            DatabaseEngine::Sqlite => (
                "jdbc:sqlite:negodesign.db".to_string(),
                "".to_string(),
                "".to_string(),
            ),
        }
    };

    let application_yaml = format!(
        r#"server:
  port: ${{PORT:8080}}
  error:
    include-stacktrace: never
    include-binding-errors: always
    include-message: always

spring:
  application:
    name: {app_name}
  profiles:
    active: ${{SPRING_PROFILES_ACTIVE:local}}
  threads:
    virtual:
      enabled: true
  mvc:
    problem-details:
      enabled: true
  servlet:
    multipart:
      max-file-size: 20MB
      max-request-size: 20MB

springdoc:
  pathsToMatch: /api/v1/admin/**,/api/v1/pub/**

scalar:
  enabled: true

rsa:
  private-key: ${{RSA_PRIVATE_KEY:classpath:certs/rsa/private.pem}}
  public-key: ${{RSA_PUBLIC_KEY:classpath:certs/rsa/public.pem}}

app:
  cors:
    allowed-origins: ${{APP_CORS_ALLOWED_ORIGINS:{cors_origin}}}
"#
    );

    let application_local_yml = format!(
        r#"spring:
  datasource:
    url: ${{DB_URL:{db_url}}}
    username: ${{DB_USERNAME:{db_user}}}
    password: ${{DB_PASSWORD:{db_pass}}}
  jpa:
    hibernate:
      ddl-auto: update
    show-sql: false
"#
    );

    files.insert("src/main/resources/application.yaml".to_string(), application_yaml);
    files.insert("src/main/resources/application-local.yml".to_string(), application_local_yml);
    files.insert("src/main/resources/certs/rsa/private.pem".to_string(), RSA_PRIVATE_KEY_PEM.to_string());
    files.insert("src/main/resources/certs/rsa/public.pem".to_string(), RSA_PUBLIC_KEY_PEM.to_string());
}
