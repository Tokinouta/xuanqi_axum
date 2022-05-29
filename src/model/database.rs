use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client,
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

pub async fn connect_mongodb(client: &Client) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    // 注意私有的函数也可以被测试！
    use super::*;

    #[test]
    fn test_mongodb() {
        // 这里注意client和使用它的函数需要在同一个运行环境里，不能由两个block_on函数分别执行
        // 否则第二个block_on可能获取不到第一个的一些信息，导致报错“Server selection timeout: No available servers.”。
        let a = || async {
            let client = create_client().await;
            connect_mongodb(&client).await
        };
        let res = tokio_test::block_on(a());
        assert_eq!(res, vec!["admin", "config", "local"])
    }
}
