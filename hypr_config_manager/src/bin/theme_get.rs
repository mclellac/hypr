use hypr_config_manager::store::ThemeStore;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hypr-theme-get <key> [--default <value>]");
        std::process::exit(1);
    }

    let key = &args[1];

    // Check for --default
    let mut default_value = None;
    let mut i = 2;
    while i < args.len() {
        if args[i] == "--default" && i + 1 < args.len() {
            default_value = Some(&args[i + 1]);
            break;
        }
        i += 1;
    }

    let store = ThemeStore::global();
    let theme_name = store.current_theme();

    match store.get_theme_data(&theme_name) {
        Ok(theme_data) => {
            if let Some(val) = theme_data.get_raw_value(key) {
                match val {
                    yaml_serde::Value::String(s) => println!("{}", s),
                    yaml_serde::Value::Number(n) => println!("{}", n),
                    yaml_serde::Value::Bool(b) => println!("{}", b),
                    yaml_serde::Value::Null => {
                        if let Some(def) = default_value {
                            println!("{}", def);
                        } else {
                            std::process::exit(1);
                        }
                    }
                    _ => {
                        // For structured data, serialize to JSON
                        if let Ok(json_val) = serde_json::to_value(val)
                            && let Ok(json_str) = serde_json::to_string(&json_val)
                        {
                            println!("{}", json_str);
                        } else if let Some(def) = default_value {
                            println!("{}", def);
                        } else {
                            std::process::exit(1);
                        }
                    }
                }
            } else if let Some(def) = default_value {
                println!("{}", def);
            } else {
                std::process::exit(1);
            }
        }
        Err(_) => {
            if let Some(def) = default_value {
                println!("{}", def);
            } else {
                std::process::exit(1);
            }
        }
    }
}
