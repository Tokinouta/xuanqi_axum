use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client,
};

// #[test]
pub async fn test_mongodb() {
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
}
