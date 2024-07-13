use crate::builder::field::Field;

#[derive(Debug)]
pub struct Fields;
impl Fields {
    /// Builder to define multiple field(s).
    /// ## Parameter
    /// * `items` - The a vector of `Field` struct.
    /// 
    /// ```ignore
    /// let fields: Vec<Field> = Vec::from([
    ///     Field {       
    ///         name: "field1",
    ///         table: "mytable",
    ///         kind: Kind::Number,
    ///         ..Field::default()
    ///     },
    ///     Field {       
    ///         name: "field2",
    ///         table: "mytable",
    ///         kind: Kind::String,
    ///         ..Field::default()
    ///     }
    /// ]);
    /// match Fields::build(fields) {
    ///     Ok(stmts) => {
    ///         assert!(stmts.len()>0);                      
    ///         assert!(db.client.query(stmts).await.is_ok());
    ///     }
    ///     _ => assert!(false,"Expecting queries")
    /// }
    /// ```
    /// 
    pub fn build( items: Vec<Field>) -> Result<String,String>{
        let mut stmts: Vec<String> = Vec::new();
        for item in items {
            match Field::build(item) {
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
mod test {
    use super::*;
    use crate::db::Db;
    use crate::builder::table::Table;
    use crate::builder::prelude::{ Kind, Value, Permissions, TableKind };

    #[tokio::test]
    async fn test_build(){
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        let table_name: &str = "test_field_define";
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
                        name: "fields_numeric",
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
                        name: "fields_str",
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