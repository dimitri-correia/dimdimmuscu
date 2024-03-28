pub fn insert(table: &str, keys: &[&str], returns: Option<Vec<&str>>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_no_return() {
        let table = "users";
        let keys = vec!["name", "age"];
        let query = insert(table, &keys, None);

        assert_eq!(query, "INSERT INTO users (name, age) VALUES (?, ?)");
    }

    #[test]
    fn test_insert_with_return() {
        let table = "users";
        let keys = ["name", "age"];
        let returns = Some(vec!["id"]);
        let query = insert(table, &keys, returns);

        assert_eq!(
            query,
            "INSERT INTO users (name, age) VALUES (?, ?) RETURNING id"
        );
    }
}
