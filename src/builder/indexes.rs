use crate::builder::index::Index;

#[derive(Debug)]
pub struct Indexes;
impl Indexes {
    /// Builder to define multiple index.
    /// ## Parameter
    /// * `items` - The a vector of `Index` struct.
    /// 
    /// ```ignore
    /// let Indexes: Vec<Index> = Vec::from([
    ///     Index {       
    ///         name: "index1",
    ///         table: "mytable",
    ///         kind: Kind::Number,
    ///         ..Index::default()
    ///     },
    ///     Index {       
    ///         name: "index2",
    ///         table: "mytable",
    ///         kind: Kind::String,
    ///         ..Index::default()
    ///     }
    /// ]);
    /// match Indexes::build(Indexes) {
    ///     Ok(stmts) => {
    ///         assert!(stmts.len()>0);                      
    ///         assert!(db.client.query(stmts).await.is_ok());
    ///     }
    ///     _ => assert!(false,"Expecting queries")
    /// }
    /// ```
    /// 
    pub fn build( items: Vec<Index>) -> Result<String,String>{
        let mut stmts: Vec<String> = Vec::new();
        for index in items {
            match Index::build(index) {
                Ok(stmt) => stmts.push(stmt),
                Err(error) => { return  Err(error) }
            }
        }
        Ok(
            format!("{};",stmts.join(";").to_string())
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;
    use crate::builder::table::Table;
    use crate::builder::field::Field;
    use crate::builder::fields::Fields;
    use crate::builder::prelude::*;
    use crate::builder::index::Index;
    
    #[tokio::test]
    async fn test_builder(){
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        let table_name: &str = "test_table_indexes";
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
                match Fields::build(fields) {
                    Ok(stmts) => {
                        assert!(stmts.len()>0);                      
                        assert!(db.client.query(stmts).await.is_ok());

                        let indexes: Vec<Index> = Vec::from([
                            Index {
                                name: "myindex1",
                                table: table_name,
                                cols: vec![Idiom::from("field_index_1")],
                                index: IndexType::Uniq,
                                comment: "MyComment",
                                if_not_exists: true
                            },
                            Index {
                                name: "myindex2",
                                table: table_name,
                                cols: vec![Idiom::from("field_index_1"),Idiom::from("field_index_2")],
                                index: IndexType::Uniq,
                                comment: "MyComment",
                                if_not_exists: true
                            },
                            Index {
                                name: "myindex3",
                                table: table_name,
                                cols: vec![Idiom::from("field_index_2")],
                                index: IndexType::Uniq,
                                comment: "MyComment",
                                if_not_exists: true
                            }
                        ]);

                        match Indexes::build(indexes) {
                            Ok(stmts)=>{
                                assert!(stmts.len()>0);                      
                                assert!(db.client.query(stmts).await.is_ok());
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