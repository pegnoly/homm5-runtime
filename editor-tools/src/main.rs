use google_drive3::{
    api::File, hyper_rustls, hyper_util, yup_oauth2, DriveHub
};

#[tokio::main]
async fn main() {
    let secret = yup_oauth2::read_application_secret(
        "D:\\projects\\homm5-runtime\\editor-tools\\client_secret_59722361664-ptmtbg21sef4fvpffk7djnt5ogd4rf2s.apps.googleusercontent.com.json"
    ).await.unwrap();

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .force_account_selection(true)
    .persist_tokens_to_disk("D:\\projects\\homm5-runtime\\editor-tools\\target\\tokens\\token.json")
    .build()
    .await
    .unwrap();

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    let hub = DriveHub::new(client, auth);
    let res = hub
        .files()
        .create(File {
            parents: Some(vec!["1ecJTiwv5CzJ5T8vyVqA-CqivwbzV8y-I".to_string()]),
            name: Some("Test xlsx".to_string()),
            ..Default::default()
        })
        .upload(
            std::fs::File::open("D:\\test2.xlsx").unwrap(),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();
    // let res = hub.files()
    //     .list()
    //     .add_scope(Scope::MetadataReadonly)
    //     .param("fields", "files(id, name, mimeType, parents, modifiedTime)")
    //     .q("'1ecJTiwv5CzJ5T8vyVqA-CqivwbzV8y-I' in parents")
    //     .doit()
    //     .await
    //     .unwrap();
    println!("{:#?}", res.1);
}
