pub fn insert(table: &str, keys: &[&str], returns: Option<&[&str]>) -> String {
    let key_list = keys.join(", ");
    let placeholders = vec!["?"; keys.len()].join(", ");

    let mut query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table, key_list, placeholders
    );

    if let Some(ret) = returns {
        let return_list = ret.join(", ");
        query.push_str(&format!(" RETURNING {}", return_list));
    }

    query
}
