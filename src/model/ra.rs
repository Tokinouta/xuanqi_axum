use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client,
};

// #[test]
pub async fn connect_mongodb() -> Vec<String> {
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

    // List the names of the databases in that deployment.
    for db_name in client
        .list_database_names(None, None)
        .await
        .expect("failed to list")
    {
        println!("{}", db_name);
    }

    client
        .list_database_names(None, None)
        .await
        .expect("failed to list")
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    // 注意私有的函数也可以被测试！
    use super::*;

    #[test]
    fn test_mongodb() {
        let res = tokio_test::block_on(connect_mongodb());
        assert_eq!(res, vec!["admin", "config", "local"])
    }
}
