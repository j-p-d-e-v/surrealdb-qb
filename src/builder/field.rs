use crate::builder::prelude::{ Permissions, Kind, Value, Strand, Idiom, Ident };
use surrealdb::sql::statements::DefineFieldStatement;
/// # Field
/// 
/// * `name` - The field name.
/// * `table` - The table name.
/// * `kind` - The type of the field which is value of enum `Kind`.
/// * `flexible` - Allow schemaless in Schemafull. See: <https://surrealdb.com/docs/surrealdb/surrealql/statements/define/field#flexible-data-types>
/// * `readonly` - Set the field to readonly mode.
/// * `default` - Assign default value to the field. Use `surrealdb::sql::Value` to set value.
/// * `assert` - Set validation on the field. See: <https://surrealdb.com/docs/surrealdb/surrealql/statements/define/field#asserting-rules-on-fields>
/// * `value` - Modify the passed value. See: <https://surrealdb.com/docs/surrealdb/surrealql/statements/define/field#alter-a-passed-value>
/// * `permissions` - The permissions for select, create, update and delete. Default: Full Permissions.
/// * `comment` - Comment on the field.
/// * `if_not_exists` - Create or define the field if not exists.
#[derive(Debug, Clone, Default)]
pub struct Field<'a>{
    pub name: &'a str,
    pub table: &'a str,
    pub kind: Kind,
    pub flexible: bool,
    pub readonly: bool,
    pub default: Option<Value>,
    pub assert: Option<Value>,
    pub value: Option<Value>,
    pub permissions: Permissions,
    pub comment: &'a str,
    pub if_not_exists: bool,
}
impl<'a> Field<'a> {
    /// # Build a DEFINE FIELD Statement
    /// ## Parameter
    /// * `item` - The `Field` struct.
    pub fn build(
        item: Field
    ) -> Result<String,String> {
        // FIELD =======================================================
        if item.name.len() == 0 {
            return Err("Field name is required.".to_string());
        }
        let mut stmt: DefineFieldStatement = DefineFieldStatement::default();        
        stmt.name = Idiom::from(item.name.to_string());
        stmt.what = Ident::from(item.table.to_string());
        stmt.flex = item.flexible;
        stmt.kind = Some(item.kind);
        stmt.readonly=  item.readonly;
        stmt.value = item.value;
        stmt.assert = item.assert;
        stmt.permissions = item.permissions;
        stmt.default = item.default;
        stmt.if_not_exists = item.if_not_exists;
        if item.comment.len() > 0 {
            stmt.comment = Some(Strand::from(item.comment));
        }
        Ok(stmt.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;
    use crate::builder::table::Table;
    use crate::builder::prelude::*;
    
    #[tokio::test]
    async fn test_builder(){
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
                assert!(db.client.query(stmt).await.is_ok(),"Create table failed");
                let mut field_stmts = String::new();
                match Field::build(Field {       
                    name: "field_str",
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
                }) {
                    Ok(stmt) => {   
                        assert!(stmt.len()>0);
                        field_stmts.push_str(format!("{};",stmt).as_str());
                    }
                    Err(error) => {                        
                        assert!(false,"ERROR: {}",error);
                    }
                }
                match Field::build(Field {       
                    name: "field_numeric",
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
                }) {
                    Ok(stmt) => {                        
                        assert!(stmt.len()>0);
                        field_stmts.push_str(format!("{};",stmt).as_str());
                    }
                    Err(error) => {                        
                        assert!(false,"ERROR: {}",error);
                    }
                }
                assert!(field_stmts.len()>0);
                assert!(db.client.query(field_stmts).await.is_ok());
            },
            Err(error) => {    
                assert!(false,"ERROR: {}",error);
            }
        }
        
    }

}