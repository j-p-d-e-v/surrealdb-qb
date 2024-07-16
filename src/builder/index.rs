use crate::builder::prelude::{ Idioms, Idiom, Ident, IndexType };
use surrealdb::sql::statements::DefineIndexStatement;
/// # Index
/// 
/// * `name` - The index name.
/// * `table` - The table name.
/// * `cols` - A list of fields/columns to index.
/// * `index` - The type of indexing. See: <https://docs.rs/surrealdb/latest/surrealdb/sql/statements/struct.DefineIndexStatement.html>
/// * `comment` - Comment on the index.
/// * `if_not_exists` - Create or define the index if not exists.
#[derive(Debug, Clone, Default)]
pub struct Index<'a>{
    pub name: &'a str,
    pub table: &'a str,
    pub cols: Vec<Idiom>,
    pub index: IndexType,
    pub comment: &'a str,
    pub if_not_exists: bool,
}
impl <'a> Index<'a> {
    /// # Build a DEFINE Index Statement
    /// ## Parameter
    /// * `item` - The `Index` struct.
    pub fn build(
        item: Index
    ) -> Result<String,String> {
        // Index =======================================================
        if item.name.len() == 0 {
            return Err("Index name is required.".to_string());
        }
        let mut stmt: DefineIndexStatement = DefineIndexStatement::default();        
        stmt.name           = Ident::from(item.name.to_string());
        stmt.what           = Ident::from(item.table.to_string());
        stmt.index          = item.index;
        stmt.if_not_exists  = item.if_not_exists;
        if item.cols.len() > 0 {
            let mut idioms: Idioms = Idioms::default();
            idioms.0    = item.cols;
            stmt.cols   = idioms;
        }
        Ok(stmt.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;
    use crate::builder::table::Table;
    use crate::builder::field::Field;
    use crate::builder::query::Query;
    use crate::builder::prelude::*;
    
    #[tokio::test]
    async fn test_builder(){
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        let table_name: &str = "test_table_index";
        match Table::build(Table {       
            name: table_name,
            kind: TableKind::Normal,
            if_not_exists: true,
            permissions: Permissions::full(),
            ..Table::default()
        }) {
            Ok(stmt) => {
                assert!(db.client.query(stmt).await.is_ok());
                let fields: Vec<Field> = Vec::from([
                    Field {       
                        name: "field_index_1",
                        table: table_name,
                        kind: Kind::Number,
                        flexible: true,
                        default: Some(
                            Value::Number(
                                surrealdb::sql::Number::Int(1)
                            )
                        ),
                        if_not_exists: true,
                        permissions: Permissions::full(),
                        ..Field::default()
                    },
                    Field {       
                        name: "field_index_2",
                        table: table_name,
                        kind: Kind::String,
                        default: Some(
                            Value::Strand(
                                surrealdb::sql::Strand::from("mydefaultvalue")
                            )
                        ),
                        if_not_exists: true,
                        permissions: Permissions::full(),
                        ..Field::default()
                    }
                ]);
                match Query::new(fields).build() {
                    Ok(stmts) => {
                        assert!(stmts.len()>0);                      
                        assert!(db.client.query(stmts).await.is_ok());

                        match Index::build(Index {
                            name: "myindex2",
                            table: table_name,
                            cols: vec![Idiom::from("field_index_1"),Idiom::from("field_index_2")],
                            index: IndexType::Uniq,
                            comment: "MyComment",
                            if_not_exists: true
                        }) {
                            Ok(stmt) => {
                                assert!(stmt.len()>0);                      
                                assert!(db.client.query(stmt).await.is_ok());
                            }
                            Err(error) => {
                                assert!(false,"{}",error)
                            }
                        }
                    }
                    _ => assert!(false,"Expecting queries")
                }
            },
            Err(error) => {    
                assert!(false,"ERROR: {}",error);
            }
        }
    }

}