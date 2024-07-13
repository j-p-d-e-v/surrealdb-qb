
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::{ Root, Jwt };
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

#[derive(Debug)]
pub struct Db{
    pub client: Surreal<Client>,
    pub jwt: Jwt,
}

impl Db {
    /// Establish a SurrealDB Client connection.
    /// Note: This is just a simple way to connect to SurrealDB Instance.
    /// ## Parameters
    /// * `address` - The host:port address.
    /// * `username` - The registered username of the user.
    /// * `password` - The registered password of the user.
    /// * `namespace` - The namespace of the database.
    /// * `database` - The database to select.
    /// ## Returns
    /// * `client` - A SurrealDB client instance.
    /// * `jwt` - A jwt value.
    /// 
    pub async fn new<'a>(address: &str, username: &str, password: &str, namespace: &str, database: &str) -> Self {
        match Surreal::new::<Ws>(address).await {
            Ok(client) => {
                match client.signin(Root { username, password }).await {
                    Ok(jwt) => {

                        if let Err(error) = client.use_ns(namespace).use_db(database).await {
                            panic!("Unable to connect namespace {} / database: {}. ERROR: {:#?}",namespace,database,error);
                        }
                        return Self {
                            client,
                            jwt
                        }
                    }
                    Err(error) => {

                        panic!("Unable to signin to the database. ERROR:{:#?}",error);
                    }
                }
            }
            Err(error) => {
                panic!("Unable to connect to the database. ERROR:{:#?}",error);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use surrealdb::sql::Thing;
    use super::*;
    #[derive(Debug, Deserialize)]
    struct MyTestRecord {
        #[allow(dead_code)]
        id: Thing,
    }
    #[derive(Debug, Serialize)]
    struct MyTest {
        #[allow(dead_code)]
        name: String,
    }
    #[tokio::test]
    async fn test_connection() {
    
        let db = Db::new("127.0.0.1:6080","root","root","test","test").await;
        let created: Option<MyTestRecord> = db.client.create(("test","test")).content(MyTest {
            name: "test".to_string()
        }).await.unwrap();
        assert!(created.is_some());
        let deleted: Option<MyTestRecord> = db.client.delete(("test","test")).await.unwrap();
        assert!(deleted.is_some());
    
    }
}