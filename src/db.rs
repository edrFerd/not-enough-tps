use sqlx::{Pool, Postgres, QueryBuilder};

use crate::data::SendingData;

pub async fn init_and_check_db() -> anyhow::Result<Pool<Postgres>> {
    let cfg = crate::config::get_cfg();
    let db_cfg = &cfg.database;

    if db_cfg.user.contains("@") || db_cfg.password.contains("@") {
        panic!("Fuck off @") // 你看见这个源代码的时候，去前端用 encodeURIComponent("@") 拿 value 去
    }

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_cfg.user, db_cfg.password, db_cfg.host, db_cfg.port, db_cfg.database
    );

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(db_cfg.max_connections)
        .connect(&db_url)
        .await?;

    check_db(&pool).await?;

    Ok(pool)
}

const MAIN_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS tps.main (
    -- 记录编号：自增 bigint 主键
    id BIGSERIAL PRIMARY KEY,

    -- 终端编号(Rust: u32)
    data_id int4 NOT NULL,

    -- 电力状态数据
    voltage DOUBLE PRECISION NOT NULL,
    current DOUBLE PRECISION NOT NULL,
    power DOUBLE PRECISION NOT NULL,
    power_factor DOUBLE PRECISION NOT NULL,
    frequency DOUBLE PRECISION NOT NULL,
    total_active_power DOUBLE PRECISION NOT NULL,
    total_reactive_power DOUBLE PRECISION NOT NULL,

    -- 创建时间
    create_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
"#;

const CHECK_SQL: &str = r#"
SELECT EXISTS (
    SELECT 1
    FROM pg_catalog.pg_namespace n
    JOIN pg_catalog.pg_class c ON c.relnamespace = n.oid
    WHERE n.nspname = 'tps'
      AND c.relname = 'main'
      AND c.relkind = 'r'
) AS exists;
"#;

async fn check_db(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    // select 1 确认能用
    sqlx::query("SELECT 1").execute(pool).await?;

    // 检查表是否存在，不存在则创建 schema 和表
    let exists: (bool,) = sqlx::query_as(CHECK_SQL).fetch_one(pool).await?;
    if !exists.0 {
        // 确认 schema 存在
        sqlx::query("CREATE SCHEMA IF NOT EXISTS tps")
            .execute(pool)
            .await?;
        // 创建表
        sqlx::query(MAIN_SQL).execute(pool).await?;
    }

    Ok(())
}

pub async fn insert_batch(
    pool: &sqlx::Pool<Postgres>,
    batch: &[SendingData],
) -> anyhow::Result<()> {
    if batch.is_empty() {
        return Ok(());
    }

    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        INSERT INTO tps.main (
            data_id,
            voltage,
            current,
            power,
            power_factor,
            frequency,
            total_active_power,
            total_reactive_power
        )
        "#,
    );

    qb.push_values(batch, |mut b, d| {
        b.push_bind(d.id as i32)
            .push_bind(d.voltage)
            .push_bind(d.current)
            .push_bind(d.power)
            .push_bind(d.power_factor)
            .push_bind(d.frequency)
            .push_bind(d.total_active_power)
            .push_bind(d.total_reactive_power);
    });

    qb.build().execute(pool).await?;
    Ok(())
}

pub async fn insert_data_to_db(pool: &Pool<Postgres>, data: &SendingData) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO tps.main (
            data_id,
            voltage,
            current,
            power,
            power_factor,
            frequency,
            total_active_power,
            total_reactive_power
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        "#,
    )
    .bind(data.id as i32)
    .bind(data.voltage)
    .bind(data.current)
    .bind(data.power)
    .bind(data.power_factor)
    .bind(data.frequency)
    .bind(data.total_active_power)
    .bind(data.total_reactive_power)
    .execute(pool)
    .await?;
    Ok(())
}
