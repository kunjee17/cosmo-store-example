mod index_aggregate;
use anyhow::Result;
use tokio::fs::{File, OpenOptions};
use cosmo_store::common::i64_event_version::EventVersion;
use cosmo_store::traits::event_store::EventStore;
use cosmo_store::types::event_read::EventRead;
use cosmo_store::types::event_read_range::EventsReadRange;
use cosmo_store::types::expected_version::ExpectedVersion;
use cosmo_store_sqlx_sqlite::event_store_sqlx_sqlite::EventStoreSQLXSqlite;
use cosmo_store_util::aggregate::{Aggregate, make_handler};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use index_aggregate::{State, Event, Command, IndexAggregate};


#[derive(Clone, Debug, Deserialize, Serialize)]
struct Meta {}

#[tokio::main]
async fn main() -> Result<()> {
    // Create database file
    let _ = OpenOptions::new().create(true).append(true).open("sample.db").await?;

    // Create database pool
    let pool = SqlitePoolOptions::new().connect("sqlite:sample.db").await?;
    const INDEX_AGGREGATE : IndexAggregate  = IndexAggregate::init();

    let store = EventStoreSQLXSqlite::new(&pool, "index").await?;
    let stream_id = Uuid::new_v4().as_simple().to_string();
    let _ = make_handler(&INDEX_AGGREGATE, &store, &Command::AddOrg, &stream_id, &EventsReadRange::AllEvents, &ExpectedVersion::Any).await?;
    let _ = make_handler(&INDEX_AGGREGATE, &store, &Command::AddEmployee, &stream_id, &EventsReadRange::AllEvents, &ExpectedVersion::Any).await?;
    let _ = make_handler(&INDEX_AGGREGATE, &store, &Command::AddEmployee, &stream_id, &EventsReadRange::AllEvents, &ExpectedVersion::Any).await?;

    let events: Vec<EventRead<Event, Meta, EventVersion>> = store.get_events(&stream_id, &EventsReadRange::AllEvents).await?;

    let state : State =
        events
        .iter()
        .fold(INDEX_AGGREGATE.init(), |a, b| INDEX_AGGREGATE.apply(a, &b.data));

    println!("Hello, world!!");
    println!("{:?}", state);
    Ok(())
}
