use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn landing_page() -> impl IntoResponse {
    let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to Dim Dim Muscu</title>
    <script src="https://unpkg.com/htmx.org"></script>
    <!-- Add your CSS framework link here for mobile-first design -->
</head>
<body>
    <h1>Welcome to Dim Dim Muscu!</h1>
    <!-- Your HTMX-enhanced content goes here -->
</body>
</html>
"#;
    (StatusCode::OK, Html(html_content)).into_response()
}
