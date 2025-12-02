pub fn map_annotation(java: &str) -> Option<&'static str> {
    match java {
        "@RestController" => Some("// mapped: Rust uses modules + functions"),
        "@GetMapping" => Some("// mapped: Axum GET route"),
        "@Service" => Some("// mapped: Rust uses structs + impl"),
        _ => None,
    }
}
