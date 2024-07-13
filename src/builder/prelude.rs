pub use surrealdb::sql::{
    Ident, Idiom, Strand, Value, Kind, Permission, Permissions, View, ChangeFeed,
    Relation, Idioms
};
pub use crate::builder::table::TableKind;
pub type IndexType = surrealdb::sql::Index;