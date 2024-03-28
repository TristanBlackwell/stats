use super::schema::{collectors, events};
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

/// A collector is an instance of the Stats script on a website e.g.
/// a user visiting the site. It holds various metadata such as
/// location information and browser details.
#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = collectors)]
pub struct Collector {
    pub id: String,
    pub origin: String,
    pub country: String,
    pub city: String,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub timestamp: NaiveDateTime,
}

/// An event is an action invoked by a collector. It has a name (`enter`, `exit`, `navigate`)
/// and various metadata
///
/// This particular type is an event stored in the database. See `NewEvent` for
/// an incoming event to be stored.
#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(Collector, foreign_key = collector_id))]
#[diesel(table_name = events)]
pub struct Event {
    pub id: String,
    pub url: String,
    pub referrer: Option<String>,
    pub name: String,
    pub timestamp: NaiveDateTime,
    pub collector_id: String,
}

#[derive(Deserialize)]
pub struct EventRequest {
    pub url: String,
    pub name: String,
    pub collector_id: String,
}

/// This Event type is an event yet to be stored in the database. See `Event` for
/// previously stored events.
#[derive(Insertable, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub id: String,
    pub url: String,
    pub referrer: Option<String>,
    pub name: String,
    pub timestamp: NaiveDateTime,
    pub collector_id: String,
}
