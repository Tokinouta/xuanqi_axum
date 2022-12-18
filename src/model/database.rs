use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::entities::{Item, User};

pub const DB_NAME: &str = "myApp";
pub const USERS_COLL: &str = "users";
pub const REPOS_COLL: &str = "repos";
pub const ITEMS_COLL: &str = "items";

#[derive(Debug)]
pub enum DatabaseError {
    Mongo(mongodb::error::Error),
    Bson(mongodb::bson::ser::Error),
}

pub async fn create_client() -> PgPool {
    // 尽管这里面看上去都是同步API，但是实际还是需要一个异步环境来执行。否则会报错。
    // 千万注意这点⬆(切换为postgresql之后这点待确认)
    // 以及async是可以没有await的😂

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    pool
}

pub async fn create_item(pool: PgPool, items: &Item) -> Result<(), sqlx::Error> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
}

pub async fn verify_user(pool: PgPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "select id from user_info where name = $1 and authentication = $2;",
        user.name,
        user.auth.as_bytes()
    )
    .fetch_one(&pool)
    .await
    .map(|a| {})
}

pub async fn add_user(pool: PgPool, user: &User) -> Result<(), Box<dyn std::error::Error>> {
    // 查询user数据库中有没有当前用户的信息，并返回查到的数量
    // 注意这里需要引入futures::stream::StreamExt才能使用count()函数
    let res = sqlx::query!(
        "select id from user_info where name = $1 and authentication = $2;",
        user.name,
        user.auth.as_bytes()
    )
    .fetch_one(&pool)
    .await;

    // 如果没有就新增这个用户
    if res.is_ok() {
        match sqlx::query!(
            "select id from user_info where name = $1 and authentication = $2;",
            user.name,
            user.auth.as_bytes()
        )
        .fetch_one(&pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    } else {
        Err("nothing")?
    }
}

// #[cfg(test)]
// mod tests {
//     use futures::stream::TryStreamExt;
//     use mongodb::bson::{doc, Document};
//     use tokio::runtime::Runtime;

//     use crate::entities::{Authority, Item, ItemType, PublicStatus, Repo};

//     // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
//     // 注意私有的函数也可以被测试！
//     use super::*;

//     #[test]
//     fn test_mongodb() {
//         // 这里注意client和使用它的函数需要在同一个运行环境里，不能由两个block_on函数分别执行
//         // 否则第二个block_on可能获取不到第一个的一些信息，导致报错“Server selection timeout: No available servers.”。
//         let a = || async {
//             let client = create_client().await;
//             list_database_names(&client).await
//         };
//         let res = Runtime::new().unwrap().block_on(a());
//         assert_eq!(res, vec!["admin", "config", "local"])
//     }

//     #[test]
//     fn test_create_database() {
//         let name = "rarara";
//         let a = || async {
//             let client = create_client().await;
//             let db = create_database(&client, name).await.unwrap();

//             // 创建一个collection用于存储数据
//             let collection = db.collection::<Document>("books");
//             // 待写入的数据
//             let docs = vec![
//                 doc! { "title": "1984", "author": "George Orwell" },
//                 doc! { "title": "Animal Farm", "author": "George Orwell" },
//                 doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//             ];

//             // Insert some documents into the "rarara.books" collection.
//             // 写入完成之后才真正能够在数据库中获取到rarara库
//             collection.insert_many(docs, None).await.expect("msg");
//             list_database_names(&client).await
//         };
//         let res = Runtime::new().unwrap().block_on(a());
//         assert!(res.contains(&name.to_string()));
//         let b = || async {
//             let client = create_client().await;
//             client
//                 .database(name)
//                 .drop(None) // 删除rarara，毕竟是一个测试用的库
//                 .await
//                 .expect("no such database");
//             list_database_names(&client).await
//         };
//         let res = Runtime::new().unwrap().block_on(b());
//         assert!(!res.contains(&name.to_string()));
//     }

//     #[test]
//     fn test_customized_add() {
//         // 生成测试数据
//         let repo = Repo {
//             _id: None,
//             name: "rarara".to_string(),
//             owner: String::from("ra"),
//             public_status: PublicStatus::Private,
//             modifiers: vec![String::from("ra"), String::from("ra"), String::from("ra")],
//         };

//         let item = Item {
//             _id: None,
//             repo: repo.name(),
//             proposer: String::from("ra"),
//             authority: Authority::USER_READ
//                 | Authority::USER_WRITE
//                 | Authority::GROUP_READ
//                 | Authority::GROUP_WRITE
//                 | Authority::OTHER_READ
//                 | Authority::OTHER_READ,
//             approvement: 0,
//             itemtype: ItemType::Item,
//             name: "Test".to_string(),
//             description: "Test Item".to_string(),
//             description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
//             word_vector: vec![0.0, 0.0, 0.0],
//             content: Some(Box::new(Item {
//                 _id: None,
//                 repo: repo.name(),
//                 proposer: String::from("ra"),
//                 authority: Authority::USER_READ | Authority::OTHER_READ,
//                 approvement: 0,
//                 itemtype: ItemType::File,
//                 name: "Test sub".to_string(),
//                 description: "Test Sub Item".to_string(),
//                 description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
//                 word_vector: vec![1.0, 2.0, 3.0],
//                 content: None,
//             })),
//         };

//         let a = || async {
//             let name = "6692dc25-b144-459b-9a1d-53ad7490683c";
//             let client = create_client().await;
//             let db = client.database(name);

//             // 创建一个collection用于存储数据
//             let collection = db.collection::<Item>("books");
//             // 待写入的数据
//             let docs = vec![&item];

//             // Insert some documents into the "rarara.books" collection.
//             // 写入完成之后才真正能够在数据库中获取到rarara库
//             collection.insert_many(docs, None).await.expect("msg");
//             list_database_names(&client).await;
//             let res = collection
//                 .find(doc! { "proposer": { "$in": [ "ra", "rara" ] } }, None)
//                 .await
//                 .unwrap();
//             client
//                 .database(name)
//                 .drop(None) // 删除rarara，毕竟是一个测试用的库
//                 .await
//                 .expect("no such database");
//             let res: Vec<Item> = res.try_collect().await.unwrap();

//             res
//         };
//         let res = Runtime::new().unwrap().block_on(a());
//         assert!(res.contains(&item));
//     }

//     #[test]
//     fn test_verify_suer() {
//         // 生成测试数据
//         let user = User::new("rarara".to_string(), "rarara".to_string());

//         let a = || async {
//             // let name = "rarara";
//             let client = create_client().await;

//             // 创建一个collection handler
//             let collection = client.database(DB_NAME).collection::<User>(USERS_COLL);
//             // 待写入的数据
//             let docs = vec![&user];

//             // Insert some documents into the "rarara.books" collection.
//             let res1 = verify_user(&client, &user).await.unwrap();
//             collection.insert_many(docs, None).await.expect("msg");
//             let res2 = verify_user(&client, &user).await.unwrap();
//             collection
//                 .delete_one(to_document(&user).unwrap(), None)
//                 .await
//                 .unwrap();
//             let res3 = verify_user(&client, &user).await.unwrap();
//             (res1, res2, res3)
//         };
//         let res = Runtime::new().unwrap().block_on(a());
//         assert_eq!(res, (false, true, false));
//     }
// }
