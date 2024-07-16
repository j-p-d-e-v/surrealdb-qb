use crate::builder::table::Table;
use crate::builder::field::Field;
use crate::builder::index::Index;
use crate::builder::param::Param;

/// Build query statements.
#[derive(Debug, Clone)]
pub struct Query<T> {
    pub items: Vec<T>,
    pub stmts: Vec<String>
}

impl<T> Query<T> {
    /// ## Examples
    /// 
    /// ### Fields
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
    /// match Query::new(fields).build(fields) {
    ///     Ok(stmts) => {
    ///         assert!(stmts.len()>0);                      
    ///         assert!(db.client.query(stmts).await.is_ok());
    ///     }
    ///     _ => assert!(false,"Expecting queries")
    /// }
    /// ```
    /// 
    /// ### Indexes
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
    /// match Query::new(indexes).build(fields) {
    ///     Ok(stmts) => {
    ///         assert!(stmts.len()>0);                      
    ///         assert!(db.client.query(stmts).await.is_ok());
    ///     }
    ///     _ => assert!(false,"Expecting queries")
    /// }
    /// ```
    /// 
    /// ### Param
    /// 
    /// ```ignore
    /// let params: Vec<Param> = Vec::from([
    ///     Param  {       
    ///         name: "myparam_via_query_string",
    ///         value: Value::from("myparam_via_query_string".to_string()),
    ///         permission: Permission::Full,
    ///         comment: "hello myparam_via_query_string",
    ///         if_not_exists: true,
    ///     },
    ///     Param  {       
    ///         name: "myparam_via_query_number",
    ///         value: Value::from(1),
    ///         permission: Permission::Full,
    ///         comment: "hello myparam_via_query_string",
    ///         if_not_exists: true,
    ///     },
    ///     Param {       
    ///         name: "myparam_via_query_array",
    ///         value: Value::from(vec!["hello","world"]),
    ///         permission: Permission::Full,
    ///         comment: "hello myparam_via_query_string",
    ///         if_not_exists: true,
    ///     },
    ///     Param {       
    ///         name: "myparam_via_query_object",
    ///         value: Value::from(HashMap::from([
    ///             ("number",Value::from(1)),
    ///             ("bool",Value::from(true)),
    ///             ("str",Value::from("hello world")),
    ///             ("array",Value::from(vec!["hello","world1234567"])),
    ///         ])),
    ///         permission: Permission::Full,
    ///         comment: "hello myparam_via_query_string",
    ///         if_not_exists: false,
    ///     },
    /// ]);
    /// match Query::new(params).build() {
    ///     Ok(stmts) => {
    ///         assert!(stmts.len()>0);                      
    ///         assert!(db.client.query(stmts).await.is_ok());
    ///     }
    ///     _ => assert!(false,"Expecting param queries")
    /// }
    /// ```
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            stmts: Vec::new()
        }
    }

    pub fn get_statement(&mut self, item: Result<String,String>) -> Result<(),String> {
        match item {
            Ok(stmt) => {
                self.stmts.push(stmt);
                Ok(())
            },
            Err(error) => { return  Err(format!("QUERY STATEMENT ERROR: {}",error)) }
        }
    }

    pub fn to_string(&self) -> String {
        format!("{};",self.stmts.join(";").to_string())
    }
}

impl<'a> Query<Field<'a>>{
    pub fn build(&mut self) -> Result<String,String>{
        for item in self.items.clone() {
            self.get_statement(Field::build(item))?
        }
        Ok(self.to_string())
    }
}

impl<'a> Query<Index<'a>>{
    pub fn build(&mut self) -> Result<String,String>{
        for item in self.items.clone() {
            self.get_statement(Index::build(item))?
        }
        Ok(self.to_string())
    }
}

impl<'a> Query<Table<'a>>{
    pub fn build(&mut self) -> Result<String,String>{
        for item in self.items.clone() {
            self.get_statement(Table::build(item))?
        }
        Ok(self.to_string())
    }
}

impl<'a> Query<Param<'a>>{
    pub fn build(&mut self) -> Result<String,String>{
        for item in self.items.clone() {
            self.get_statement(Param::build(item))?
        }
        Ok(self.to_string())
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use crate::db::Db;
    use crate::builder::prelude::*;

    #[tokio::test]
    async fn test_fields(){
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
                match Query::new(fields).build() {
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
    #[tokio::test]
    async fn test_indexes(){
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
                match Query::new(fields).build() {
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

                        match Query::new(indexes).build() {
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

    #[tokio::test]
    async fn test_tables(){
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        let tables: Vec<Table> = Vec::from([
            Table {       
                name: "my_table_normal_1",
                kind: TableKind::Normal,
                if_not_exists: true,
                permissions: Permissions::full(),
                ..Table::default()
            },
            Table {       
                name: "my_table_any_1",
                kind: TableKind::Any,
                if_not_exists: true,
                permissions: Permissions::full(),
                ..Table::default()
            },
            Table {       
                name: "my_table_normal_2",
                kind: TableKind::Normal,
                if_not_exists: true,
                permissions: Permissions::full(),
                ..Table::default()
            },
        ]);
        match Query::new(tables).build() {
            Ok(stmts) => {
                assert!(stmts.len()>0);                      
                assert!(db.client.query(stmts).await.is_ok());
            }
            _ => assert!(false,"Expecting table queries")
        }
    }

    
    #[tokio::test]
    async fn test_params(){
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        let params: Vec<Param> = Vec::from([
            Param  {       
                name: "myparam_via_query_string",
                value: Value::from("myparam_via_query_string".to_string()),
                permission: Permission::Full,
                comment: "hello myparam_via_query_string",
                if_not_exists: true,
            },
            Param  {       
                name: "myparam_via_query_number",
                value: Value::from(1),
                permission: Permission::Full,
                comment: "hello myparam_via_query_string",
                if_not_exists: true,
            },
            Param {       
                name: "myparam_via_query_array",
                value: Value::from(vec!["hello","world"]),
                permission: Permission::Full,
                comment: "hello myparam_via_query_string",
                if_not_exists: true,
            },
            Param {       
                name: "myparam_via_query_object",
                value: Value::from(HashMap::from([
                    ("number",Value::from(1)),
                    ("bool",Value::from(true)),
                    ("str",Value::from("hello world")),
                    ("array",Value::from(vec!["hello","world1234567"])),
                ])),
                permission: Permission::Full,
                comment: "hello myparam_via_query_string",
                if_not_exists: false,
            },
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