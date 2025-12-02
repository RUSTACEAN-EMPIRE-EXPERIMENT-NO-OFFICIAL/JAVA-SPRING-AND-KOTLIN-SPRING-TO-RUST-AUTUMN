use crate::mapper::spring_to_rust_mapper::map_annotation;

pub fn refactor_java_to_rust(line: &str) -> String {
    if let Some(mapped) = map_annotation(line.trim()) {
        return mapped.to_string();
    }

    // 간단한 변환 예시
    if line.contains("public class") {
        return "// Rust: mod + struct".to_string();
    }

    if line.contains("String") {
        return line.replace("String", "&str");
    }

    "// no mapping yet".to_string()
}
