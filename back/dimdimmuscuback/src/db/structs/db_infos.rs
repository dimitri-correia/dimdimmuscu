#[derive(Clone)]
pub struct DbInfos {
    pub pool: sqlx::PgPool,
}
