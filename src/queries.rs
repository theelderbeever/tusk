use std::borrow::Cow;
use std::convert::From;

use clap::{Args, Subcommand, ValueEnum};
use serde::Serialize;
use sqlx::PgConnection;
use tabled::{Table, Tabled};

use crate::{config::ConnectionProfile, types::TimeUnit};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = false)]
pub struct QueryCommand {
    #[arg(short, long, default_value = "table", global = true)]
    output: Output,
    #[command(subcommand)]
    query: Query,
}

impl QueryCommand {
    pub async fn exec(self, profile: ConnectionProfile) {
        let mut conn = profile.connect().await;
        self.query.exec(&mut conn, self.output).await;
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Output {
    Json,
    Table,
    Yaml,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Query {
    OlderThan {
        #[arg(
            help = "Filter for queries older than. Should be a number and unit (5s, 5min, 5h, 5d)"
        )]
        older_than: TimeUnit,
    },
    Kill {
        pid: i32,
    },
}

impl Query {
    pub async fn exec(self, conn: &mut PgConnection, display: Output) {
        match self {
            Self::OlderThan { older_than } => {
                display_query(OlderThanQuery::fetch(conn, older_than).await, display)
            }
            Self::Kill { pid } => display_query(KillPidQuery::execute(conn, pid).await, display),
        }
    }
}

pub fn display_query<T>(result: Vec<T>, format: Output)
where
    T: Sized + Serialize + Tabled,
{
    println!(
        "{}",
        match format {
            Output::Json => serde_json::to_string(&result).unwrap(),
            Output::Yaml => serde_yaml::to_string(&result).unwrap(),
            Output::Table => Table::new(result).to_string(),
        }
    );
}

#[derive(sqlx::FromRow, Debug, Serialize, Tabled)]
pub struct KillPidQuery {
    pub pid: i32,
    pub succeeded: bool,
}

impl KillPidQuery {
    pub async fn execute(conn: &mut PgConnection, pid: i32) -> Vec<Self> {
        let query = format!("SELECT {pid} AS pid, pg_terminate_backend({pid}) AS succeeded");

        sqlx::query_as::<_, Self>(&query)
            .fetch_all(conn)
            .await
            .unwrap()
    }
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct OlderThanQuery {
    pub pid: i32,
    pub duration: String,
    pub query: String,
    pub state: String,
}

impl OlderThanQuery {
    pub async fn fetch(conn: &mut PgConnection, duration: crate::types::TimeUnit) -> Vec<Self> {
        let query = format!(
            r#"
            SELECT
                pid,
                (now() - pg_stat_activity.query_start)::TEXT AS duration,
                query,
                state
            FROM
                pg_stat_activity
            WHERE
                (now() - pg_stat_activity.query_start) > interval '{duration}'
            "#
        );

        sqlx::query_as::<_, Self>(&query)
            .fetch_all(conn)
            .await
            .unwrap()
    }
}

impl Tabled for OlderThanQuery {
    const LENGTH: usize = 42;
    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let pid = Cow::Owned(self.pid.to_string());
        let duration = Cow::Owned(self.duration.to_string());
        let query = Cow::Owned(format!("{}...", &self.query[0..42].to_string()));
        let state = Cow::Owned(self.state.to_string());

        vec![pid, duration, query, state]
    }
    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            Cow::Borrowed("pid"),
            Cow::Borrowed("duration"),
            Cow::Borrowed("query"),
            Cow::Borrowed("state"),
        ]
    }
}
