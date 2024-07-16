use crate::builder::prelude::{ Ident, Value, Permission };
use surrealdb::sql::statements::DefineParamStatement;
/// # Param
/// 
/// * `name` - The name of the parameter.
/// * `value` - The value of the parameter.
/// * `index` - The type of indexing. See: <https://docs.rs/surrealdb/latest/surrealdb/sql/statements/struct.DefineParamStatement.html>
/// * `comment` - Comment on the index.
/// * `permission` - The permission of the parameter. See: <https://docs.rs/surrealdb/latest/surrealdb/sql/enum.Permission.html>
/// * `if_not_exists` - Create or define the index if not exists.
#[derive(Debug, Clone, Default)]
pub struct Param<'a>{
    pub name: &'a str,
    pub value: Value,
    pub comment: &'a str,
    pub permission: Permission,
    pub if_not_exists: bool,
}
impl <'a> Param<'a> {
    /// # Build a DEFINE PARAM Statement
    /// ## Parameter
    /// * `item` - The `Param` struct.
    pub fn build(
        item: Param
    ) -> Result<String,String> {
        // Param =======================================================
        if item.name.len() == 0 {
            return Err("Param name is required.".to_string());
        }
        let mut stmt: DefineParamStatement = DefineParamStatement::default();        
        stmt.name           = Ident::from(item.name.to_string());
        stmt.value          = item.value;
        stmt.permissions    = item.permission;
        stmt.if_not_exists  = item.if_not_exists;
        Ok(stmt.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;
    use crate::builder::query::Query;
    
    #[tokio::test]
    async fn test_builder(){
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        let params: Vec<Param> = Vec::from([
            Param {       
                name: "myparam_string",
                value: Value::from("hello".to_string()),
                permission: Permission::Full,
                comment: "hello string",
                if_not_exists: true,
            },
            Param {       
                name: "myparam_bool",
                value: Value::from(true),
                permission: Permission::Full,
                comment: "hello boolean",
                if_not_exists: true,
            },
            Param {       
                name: "myparam_array",
                value: Value::from(vec![1,2,3,4]),
                permission: Permission::Full,
                comment: "hello array",
                if_not_exists: true,
            }
        ]);
        match Query::new(params).build() {
            Ok(stmts) => {
                assert!(stmts.len()>0);                      
                assert!(db.client.query(stmts).await.is_ok());

            }
            _ => assert!(false,"Expecting param queries")
        }
    }

}