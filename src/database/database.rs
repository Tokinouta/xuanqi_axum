use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::entities::{Item, User};

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

pub async fn verify_user(pool: &PgPool, user: &User) -> Result<bool, sqlx::Error> {
    let res = sqlx::query!(
        "select id from user_info where name = $1 and authentication = $2;",
        user.name,
        user.auth.as_bytes()
    )
    .fetch_one(pool)
    .await;
    println!("{:?}", res);
    res.map(|_| true)
}

/// 使用数据库的unique约束检查是否已有user
pub async fn add_user(pool: &PgPool, user: &User) -> Result<(), Box<dyn std::error::Error>> {
    match sqlx::query!(
        "insert into user_info (name, authentication) values ($1, $2);",
        user.name,
        user.auth.as_bytes()
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()), // Ok(PgQueryResult { rows_affected: 1 })
        Err(e) => Err(Box::new(e)), // Err(Database(PgDatabaseError { severity: Error, code: "23505", message: "duplicate key value violates unique constraint \"unique_table_name\"", detail: Some("Key (name)=(喵喵喵喵喵) already exists."), hint: None, position: None, where: None, schema: Some("public"), table: Some("user_info"), column: None, data_type: None, constraint: Some("unique_table_name"), file: Some("nbtinsert.c"), line: Some(664), routine: Some("_bt_check_unique") }))
    }
}

pub async fn create_item(pool: &PgPool, items: &Item) -> Result<(), sqlx::Error> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(pool)
        .await
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    // 注意私有的函数也可以被测试！
    use super::*;

    #[test]
    fn test_mongodb() {
        // 这里注意client和使用它的函数需要在同一个运行环境里，不能由两个block_on函数分别执行
        // 否则第二个block_on可能获取不到第一个的一些信息，导致报错“Server selection timeout: No available servers.”。
        let a = || async {
            let client = create_client().await;
            sqlx::query!("select * from user_info limit 1;",)
                .fetch_one(&client)
                .await
                .unwrap()
        };
        let res = Runtime::new().unwrap().block_on(a());
        // assert_eq!(res, vec!["admin", "config", "local"]);
        println!("{:?}", res);
    }

    #[test]
    fn test_add_user() {
        let user = User::new(
            "喵喵喵喵喵".to_string(),
            "de76dcb3a1adae3951c99b210d7ffb6ee2ba4faa08d4323e8099c12c9221cc2c".to_string(),
        );

        let a = || async {
            let client = create_client().await;
            add_user(&client, &user).await
        };
        let res = Runtime::new().unwrap().block_on(a());
        assert!(res.is_err());
    }

    #[test]
    fn test_verify_suer() {
        // 生成测试数据
        let user = User::new(
            "喵喵喵喵喵".to_string(),
            "de76dcb3a1adae3951c99b210d7ffb6ee2ba4faa08d4323e8099c12c9221cc2c".to_string(),
        );

        let a = || async {
            let client = create_client().await;
            verify_user(&client, &user).await.unwrap()
        };
        let res = Runtime::new().unwrap().block_on(a());
        assert_eq!(res, true);
    }

    // #[test]
    // fn test_create_database() {
    //     let name = "rarara";
    //     let a = || async {
    //         let client = create_client().await;
    //         let db = create_database(&client, name).await.unwrap();

    //         // 创建一个collection用于存储数据
    //         let collection = db.collection::<Document>("books");
    //         // 待写入的数据
    //         let docs = vec![
    //             doc! { "title": "1984", "author": "George Orwell" },
    //             doc! { "title": "Animal Farm", "author": "George Orwell" },
    //             doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    //         ];

    //         // Insert some documents into the "rarara.books" collection.
    //         // 写入完成之后才真正能够在数据库中获取到rarara库
    //         collection.insert_many(docs, None).await.expect("msg");
    //         list_database_names(&client).await
    //     };
    //     let res = Runtime::new().unwrap().block_on(a());
    //     assert!(res.contains(&name.to_string()));
    //     let b = || async {
    //         let client = create_client().await;
    //         client
    //             .database(name)
    //             .drop(None) // 删除rarara，毕竟是一个测试用的库
    //             .await
    //             .expect("no such database");
    //         list_database_names(&client).await
    //     };
    //     let res = Runtime::new().unwrap().block_on(b());
    //     assert!(!res.contains(&name.to_string()));
    // }

    // #[test]
    // fn test_customized_add() {
    //     // 生成测试数据
    //     let repo = Repo {
    //         _id: None,
    //         name: "rarara".to_string(),
    //         owner: String::from("ra"),
    //         public_status: PublicStatus::Private,
    //         modifiers: vec![String::from("ra"), String::from("ra"), String::from("ra")],
    //     };

    //     let item = Item {
    //         _id: None,
    //         repo: repo.name(),
    //         proposer: String::from("ra"),
    //         authority: Authority::USER_READ
    //             | Authority::USER_WRITE
    //             | Authority::GROUP_READ
    //             | Authority::GROUP_WRITE
    //             | Authority::OTHER_READ
    //             | Authority::OTHER_READ,
    //         approvement: 0,
    //         itemtype: ItemType::Item,
    //         name: "Test".to_string(),
    //         description: "Test Item".to_string(),
    //         description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
    //         word_vector: vec![0.0, 0.0, 0.0],
    //         content: Some(Box::new(Item {
    //             _id: None,
    //             repo: repo.name(),
    //             proposer: String::from("ra"),
    //             authority: Authority::USER_READ | Authority::OTHER_READ,
    //             approvement: 0,
    //             itemtype: ItemType::File,
    //             name: "Test sub".to_string(),
    //             description: "Test Sub Item".to_string(),
    //             description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
    //             word_vector: vec![1.0, 2.0, 3.0],
    //             content: None,
    //         })),
    //     };

    //     let a = || async {
    //         let name = "6692dc25-b144-459b-9a1d-53ad7490683c";
    //         let client = create_client().await;
    //         let db = client.database(name);

    //         // 创建一个collection用于存储数据
    //         let collection = db.collection::<Item>("books");
    //         // 待写入的数据
    //         let docs = vec![&item];

    //         // Insert some documents into the "rarara.books" collection.
    //         // 写入完成之后才真正能够在数据库中获取到rarara库
    //         collection.insert_many(docs, None).await.expect("msg");
    //         list_database_names(&client).await;
    //         let res = collection
    //             .find(doc! { "proposer": { "$in": [ "ra", "rara" ] } }, None)
    //             .await
    //             .unwrap();
    //         client
    //             .database(name)
    //             .drop(None) // 删除rarara，毕竟是一个测试用的库
    //             .await
    //             .expect("no such database");
    //         let res: Vec<Item> = res.try_collect().await.unwrap();

    //         res
    //     };
    //     let res = Runtime::new().unwrap().block_on(a());
    //     assert!(res.contains(&item));
    // }

}
