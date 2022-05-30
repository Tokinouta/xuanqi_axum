use mongodb::{
    bson::Document,
    options::{ClientOptions, Credential, ServerAddress},
    Client, Database,
};

pub async fn create_client() -> Client {
    // 尽管这里面看上去都是同步API，但是实际还是需要一个异步环境来执行。否则会报错。
    // 千万注意这点⬆
    // 以及async是可以没有await的😂
    let credential = Credential::builder()
        .username(Some("mongoadmin".to_string()))
        .password(Some("secret".to_string()))
        .build();
    // Parse a connection string into an options struct.
    let client_options = ClientOptions::builder()
        .hosts(vec![ServerAddress::parse("localhost:27017").expect("msg")])
        .app_name(Some("My App".to_string()))
        .credential(credential)
        .build();

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("failed to connect");
    client
}

pub async fn list_database_names(client: &Client) -> Vec<String> {
    // List the names of the databases in that deployment.
    let names = client
        .list_database_names(None, None)
        .await
        .expect("failed to list");

    for db_name in names.iter() {
        println!("{}", db_name);
    }

    names
}

pub async fn create_database(client: &Client, name: &str) -> Option<Database> {
    // 这里实际只创建了一个条目，并没有真正写入mongodb
    // 需要往里面写入一些document才能让他实际出现在数据库中。
    let databases = list_database_names(client).await;
    if !databases.contains(&name.to_string()) {
        println!("ready to create");
        Some(client.database(name))
    } else {
        None
    }
}

pub async fn create_item(db: Database, collection: &str, items: Vec<Document>) {
    let collection = db.collection::<Document>(collection);
    collection
        .insert_many(items, None)
        .await
        .expect("failed to insert");
}

#[cfg(test)]
mod tests {
    use mongodb::bson::{doc, Document};
    use futures::stream::{StreamExt, TryStreamExt};

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    // 注意私有的函数也可以被测试！
    use super::super::*;
    use super::*;

    #[test]
    fn test_mongodb() {
        // 这里注意client和使用它的函数需要在同一个运行环境里，不能由两个block_on函数分别执行
        // 否则第二个block_on可能获取不到第一个的一些信息，导致报错“Server selection timeout: No available servers.”。
        let a = || async {
            let client = create_client().await;
            list_database_names(&client).await
        };
        let res = tokio_test::block_on(a());
        assert_eq!(res, vec!["admin", "config", "local"])
    }

    #[test]
    fn test_create_database() {
        let name = "rarara";
        let a = || async {
            let client = create_client().await;
            let db = create_database(&client, name).await.unwrap();

            // 创建一个collection用于存储数据
            let collection = db.collection::<Document>("books");
            // 待写入的数据
            let docs = vec![
                doc! { "title": "1984", "author": "George Orwell" },
                doc! { "title": "Animal Farm", "author": "George Orwell" },
                doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
            ];

            // Insert some documents into the "rarara.books" collection.
            // 写入完成之后才真正能够在数据库中获取到rarara库
            collection.insert_many(docs, None).await.expect("msg");
            list_database_names(&client).await
        };
        let res = tokio_test::block_on(a());
        assert!(res.contains(&name.to_string()));
        let b = || async {
            let client = create_client().await;
            client
                .database(name)
                .drop(None) // 删除rarara，毕竟是一个测试用的库
                .await
                .expect("no such database");
            list_database_names(&client).await
        };
        let res = tokio_test::block_on(b());
        assert!(!res.contains(&name.to_string()));
    }

    #[test]
    fn test_customized_add() {
        // 生成测试数据
        let repo = Repo {
            _id: 0 as u64,
            name: "rarara".to_string(),
            owner: String::from("ra"),
            public_status: PublicStatus::Private,
            modifiers: vec![String::from("ra"), String::from("ra"), String::from("ra")],
        };

        let item = Item {
            _id: 1 as u64,
            repo: repo.name(),
            proposer: String::from("ra"),
            authority: Authority::USER_READ
                | Authority::USER_WRITE
                | Authority::GROUP_READ
                | Authority::GROUP_WRITE
                | Authority::OTHER_READ
                | Authority::OTHER_READ,
            approvement: 0,
            itemtype: ItemType::Item,
            name: "Test".to_string(),
            description: "Test Item".to_string(),
            description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
            word_vector: vec![0.0, 0.0, 0.0],
            content: Some(Box::new(Item {
                _id: 2 as u64,
                repo: repo.name(),
                proposer: String::from("ra"),
                authority: Authority::USER_READ | Authority::OTHER_READ,
                approvement: 0,
                itemtype: ItemType::File,
                name: "Test sub".to_string(),
                description: "Test Sub Item".to_string(),
                description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
                word_vector: vec![1.0, 2.0, 3.0],
                content: None,
            })),
        };

        let a = || async {
            let name = "rarara";
            let client = create_client().await;
            let db = create_database(&client, name).await.unwrap();

            // 创建一个collection用于存储数据
            let collection = db.collection::<Item>("books");
            // 待写入的数据
            let docs = vec![&item];

            // Insert some documents into the "rarara.books" collection.
            // 写入完成之后才真正能够在数据库中获取到rarara库
            collection.insert_many(docs, None).await.expect("msg");
            list_database_names(&client).await;
            let res = collection.find(doc!{ "proposer": { "$in": [ "ra", "rara" ] } }, None).await.unwrap();
            client
                .database(name)
                .drop(None) // 删除rarara，毕竟是一个测试用的库
                .await
                .expect("no such database");
            let res: Vec<Item> = res.try_collect().await.unwrap();

            res
        };
        let res = tokio_test::block_on(a());
        assert!(res.contains(&item));
    }
}
