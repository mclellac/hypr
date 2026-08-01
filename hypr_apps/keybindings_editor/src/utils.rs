pub fn get_display_label(mods: &str, key: &str) -> String {
    let mut label_parts = Vec::new();
    let cleaned_mods = mods.to_uppercase().replace(",", " ");
    let mod_list: Vec<&str> = cleaned_mods.split_whitespace().collect();

    for m in mod_list {
        match m {
            "$MAINMOD" | "SUPER" => label_parts.push("Super"),
            "SHIFT" => label_parts.push("Shift"),
            "ALT" => label_parts.push("Alt"),
            "CTRL" | "CONTROL" => label_parts.push("Ctrl"),
            _ => label_parts.push(m),
        }
    }

    label_parts.push(key);
    label_parts.join(" + ")
}
