pub fn toggleable_item(item: &str, active: bool) -> String {
    format!("{} {}", if active { "■" } else { " " }, item)
}
