use bson::{doc, Document};
use mongodb::{
    options::{ClientOptions, FindOneOptions},
    Client,
};

use crate::{models::user_info::UserInfo, ports::db_port::DBPort};

#[derive(Clone)]
pub struct MongoAdapter {
    client: Client,
    db_name: String,
    collection_name: String,
}

impl MongoAdapter {
    pub async fn connect(uri: String, db_name: String, collection_name: String) -> Self {
        let client_options = ClientOptions::parse(uri)
            .await
            .expect("Cannot connect with MongoDB");

        let client = Client::with_options(client_options).unwrap();
        MongoAdapter {
            client,
            db_name,
            collection_name,
        }
    }
}

impl DBPort<UserInfo> for MongoAdapter {
    async fn get_by_id(&mut self, id: String) -> Result<UserInfo, String> {
        let db = self.client.database(self.db_name.as_str());
        let collection = db.collection::<Document>(self.collection_name.as_str());

        let doc = doc! {
            "username" : id
        };
        let options = FindOneOptions::builder().build();

        match collection.find_one(Some(doc.clone()), options).await {
            Ok(res) => {
                if res.is_some() {
                    let res = res.unwrap();
                    let user: UserInfo = UserInfo {
                        username: res.get("username").unwrap().as_str().unwrap().to_string(),
                        session_id: res.get("session_id").unwrap().as_str().unwrap().to_string(),
                        auth_id: res.get("auth_id").unwrap().as_str().unwrap().to_string(),
                        y1: res.get("y1").unwrap().as_str().unwrap().to_string(),
                        y2: res.get("y2").unwrap().as_str().unwrap().to_string(),
                        r1: res.get("r1").unwrap().as_str().unwrap().to_string(),
                        r2: res.get("r2").unwrap().as_str().unwrap().to_string(),
                        c: res.get("c").unwrap().as_str().unwrap().to_string(),
                        s: res.get("s").unwrap().as_str().unwrap().to_string(),
                    };

                    return Ok(user);
                } else {
                    return Err("Not Found".to_string());
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    async fn create(&mut self, entity: UserInfo) -> Result<UserInfo, String> {
        let db = self.client.database(self.db_name.as_str());
        let collection = db.collection::<Document>(self.collection_name.as_str());

        let bson = bson::to_bson(&entity).unwrap();
        let document = bson.as_document().unwrap().to_owned();

        return match collection.insert_one(document.clone(), None).await {
            Ok(_) => Ok(entity),
            Err(err) => Err(err.to_string()),
        };
    }

    async fn update(&mut self, entity: UserInfo) -> Result<UserInfo, String> {
        let db = self.client.database(self.db_name.as_str());
        let collection = db.collection::<Document>(self.collection_name.as_str());

        let query: Document = doc! { "username": entity.clone().username };

        println!("update entity {:?}", entity.clone());

        let bson = bson::to_bson(&entity).unwrap();
        let document = bson.as_document().unwrap().to_owned();

        return match collection
            .replace_one(query.clone(), document.clone(), None)
            .await
        {
            Ok(_) => {
                let res = collection.find_one(query, None).await.unwrap().unwrap();
                println!(
                    "r1 {}",
                    res.get("r1").unwrap().as_str().unwrap().to_string(),
                );
                let user: UserInfo = UserInfo {
                    username: res.get("username").unwrap().as_str().unwrap().to_string(),
                    session_id: res.get("session_id").unwrap().as_str().unwrap().to_string(),
                    auth_id: res.get("auth_id").unwrap().as_str().unwrap().to_string(),
                    y1: res.get("y1").unwrap().as_str().unwrap().to_string(),
                    y2: res.get("y2").unwrap().as_str().unwrap().to_string(),
                    r1: res.get("r1").unwrap().as_str().unwrap().to_string(),
                    r2: res.get("r2").unwrap().as_str().unwrap().to_string(),
                    c: res.get("c").unwrap().as_str().unwrap().to_string(),
                    s: res.get("s").unwrap().as_str().unwrap().to_string(),
                };
                Ok(user)
            }
            Err(err) => Err(err.to_string()),
        };
    }
}
