use crate::builder::prelude::{Permissions, Ident, View, ChangeFeed, Strand, Kind};
use surrealdb::sql::statements::DefineTableStatement;
use surrealdb::sql::TableType;
use surrealdb::sql::Relation;

#[derive(Debug)]
pub enum TableKind {
    Any,
    Normal,
    Relation
}

impl Default for TableKind {
    fn default() -> Self {
        TableKind::Any
    }
}

/// # Table
/// 
/// * `name` - The table name.
/// * `kind` - The type of the table which is value of enum `TableKind`. Default: TableType::Any.
/// * `drop` - Marking a table as DROP disallows creating or updating records.
/// * `schema_less` - `true` for SCHEMALESS or `false` for SCHEMAFULL.
/// * `changefeed` - `true` to enable historical changes of the table.
/// * `include_original` - Only applicable if `changefeed` is `true`. Include the original state of the table.
/// * `permissions` - The permissions for select, create, update and delete. Default: Full Permissions.
/// * `comment` - Comment on the table.
/// * `relation_in` - Only applicable if `TableKind` enum is `TableKind::Relation`. Set to the incoming table.
/// * `relation_out` - Only applicable if `TableKind` enum is `TableKind::Relation`. Set to the outgoing table.
/// * `if_not_exists` - Create or define the table if not exists.
/// * `view` - The query to execute as a the view of the table.  See: <https://surrealdb.com/docs/surrealdb/surrealql/statements/define/table#pre-computed-table-views>
#[derive(Debug, Default)]
pub struct Table<'a>{
    pub name: &'a str,
    pub kind: TableKind,
    pub drop: bool,
    pub schema_less: bool,
    pub changefeed: u64,
    pub include_original: bool,
    pub permissions: Permissions,
    pub comment: &'a str,
    pub relation_in: &'a str,
    pub relation_out: &'a str,
    pub if_not_exists: bool,
    pub view: Option<View>
}
impl <'a> Table<'a> {
    /// # Build a DEFINE TABLE Statement
    /// ## Parameter
    /// * `table` - The `Table` struct.
    pub fn build(
        item: Table
    ) -> Result<String,String> {        

        if item.name.len() == 0 {
            return Err("Table name is required.".to_string());
        }
        
        let mut stmt: DefineTableStatement = DefineTableStatement::default();        
        stmt.name = Ident::from(item.name.to_string());
        stmt.drop=  item.drop;
        stmt.full = !item.schema_less;
        stmt.kind = match item.kind {
            TableKind::Any => TableType::Any,
            TableKind::Normal => TableType::Normal,
            TableKind::Relation => {                
                let mut relation: Relation = Relation::default();
                if item.relation_in.len() > 0 {
                    relation.from = Some(
                        Kind::Record(
                            vec![
                                surrealdb::sql::Table::from(item.relation_in.to_string())
                            ] 
                        )
                    );
                }
                if item.relation_out.len() > 0 {
                    relation.to = Some(
                        Kind::Record(
                            vec![
                                surrealdb::sql::Table::from(item.relation_out.to_string())
                            ] 
                        )
                    );
                }
                TableType::Relation(relation)
            }
        };
        stmt.view = item.view;
        stmt.permissions = item.permissions;
        stmt.if_not_exists = item.if_not_exists;
        if item.comment.len() > 0 {
            stmt.comment = Some(Strand::from(item.comment));
        }
        if item.changefeed > 0 {            
            let mut changefeed = ChangeFeed::default();
            changefeed.expiry = core::time::Duration::from_secs(item.changefeed);
            changefeed.store_original = item.include_original;
            stmt.changefeed = Some(changefeed);
        }
        Ok(stmt.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;
    use crate::builder::prelude::*;


    //#[test]
    //fn test_table() {
    //    use surrealdb::sql::Table;
    //    use surrealdb::sql::Relation;
    //    let mut relation: Relation = Relation::default();
    //    relation.from = Some(
    //        Kind::Record(
    //            vec![
    //                Table::from("test123".to_string())
    //            ] 
    //        )
    //    );
    //    relation.to = Some(
    //        Kind::Record(
    //            vec![
    //                Table::from("test123".to_string())
    //            ] 
    //        )
    //    );
    //    println!("{:?}",relation);
    //    println!("{}",Table::from("test123".to_string()));
    //}
    
    #[tokio::test]
    async fn test_build(){
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        // Expecting Success
        match Table::build(Table {       
            name: "test_table_define",
            kind: TableKind::Normal,
            if_not_exists: true,
            drop: true,
            schema_less: true,
            changefeed: 3000,
            include_original: false,
            permissions: Permissions::full(),
            comment: "My comment",
            ..Table::default()
        }) {
            Ok(stmt) => {
                assert!(stmt.len() > 0);
                assert!(db.client.query(stmt).await.is_ok());
            },
            Err(error) => {    
                assert!(false,"ERROR: {}",error);
            }
        }
        // Expecting Error
        match Table::build(Table {            
            name: "",
            kind: TableKind::Normal,
            if_not_exists: true,
            drop: true,
            schema_less: true,
            changefeed: 3000,
            include_original: true,
            permissions: Permissions::full(),
            comment: "My comment",
            ..Table::default()  
        }) {
            Err(error) => assert!(error.len() > 0),
            Ok(stmt) => {
                assert!(false,"Query should {} failed.",stmt);
            },
        }
        debug_assert!(
            db.client.query(
                Table::build(Table {       
                    name: "test_table_in",
                    kind: TableKind::Normal,
                    if_not_exists: true,
                    drop: true,
                    schema_less: true,
                    changefeed: 3000,
                    include_original: false,
                    permissions: Permissions::full(),
                    comment: "My comment",
                    ..Table::default()
                }).unwrap()
            ).await.is_ok()
        );
        debug_assert!(
            db.client.query(
                Table::build(Table {       
                    name: "test_table_out",
                    kind: TableKind::Normal,
                    if_not_exists: true,
                    drop: true,
                    schema_less: true,
                    changefeed: 3000,
                    include_original: false,
                    permissions: Permissions::full(),
                    comment: "My comment",
                    ..Table::default()
                }).unwrap()
            ).await.is_ok()
        );
        let stmt = db.client.query(
            Table::build(Table {       
                name: "test_table_relation",
                kind: TableKind::Relation,
                if_not_exists: true,
                drop: true,
                schema_less: true,
                changefeed: 3000,
                include_original: false,
                permissions: Permissions::full(),
                comment: "My comment",
                relation_in: "test_table_in",
                relation_out: "test_table_out",
                ..Table::default()
            }).unwrap()
        ).await.ok();
        println!("{:?}",stmt);
        let mut permissions = Permissions::default();
        permissions.select = Permission::Full;
        permissions.update = Permission::Full;
        permissions.create = Permission::None;
        permissions.delete = Permission::None;

        assert!(db.client.query(
            Table::build(Table {       
                name: "test_table_permission",
                kind: TableKind::Relation,
                if_not_exists: true,
                drop: true,
                schema_less: true,
                changefeed: 3000,
                include_original: false,
                permissions: permissions,
                comment: "My comment",
                relation_in: "test_table_in",
                relation_out: "test_table_out",
                ..Table::default()
            }).unwrap()
        ).await.is_ok());
    }

}