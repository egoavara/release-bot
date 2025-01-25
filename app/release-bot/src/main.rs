use cynic::http::ReqwestExt;

#[tokio::main]
async fn main() {
    let reqwest = reqwest::Client::new();
    // let config = oapi_jira::apis::configuration::Configuration {
    //     client: reqwest,
    //     base_path: "https://jira.daumkakao.com".to_owned(),
    //     user_agent: Some(
    //         "OpenAPI-Generator/1001.0.0-SNAPSHOT-7976c7d8afd785633bfb479e9cd673542daba37d/rust"
    //             .to_owned(),
    //     ),
    //     basic_auth: None,
    //     oauth_access_token: None,
    //     bearer_access_token: None,
    //     api_key: None,
    // };
    // let a =
    //     oapi_jira::apis::issues_api::get_issue(&config, "w", None, None, None, None, None, None)
    //         .await
    //         .unwrap();
    // println!("{:?}", a);

    // reqwest
    //     .post("https://graphql.github.com/graphql/proxy")
    //     .json(&gql_github::github::Repository {
    //         owner: "egoavara".to_owned(),
    //         name: "Hello-World".to_owned(),
    //     })
    //     .send()
    //     .await
    //     .unwrap();
    // gql_github::github::
}
