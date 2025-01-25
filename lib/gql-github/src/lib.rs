use graphql_client::GraphQLQuery;
use reqwest::Client;
use url::Url;

type URI = Url;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/github.graphql",
    query_path = "queries/RepoByUser.graphql",
    response_derives = "Debug, Serialize"
)]
pub struct RepoByUser;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/github.graphql",
    query_path = "queries/PrByUser.graphql",
    response_derives = "Debug, Serialize"
)]
pub struct PrByUser;

pub async fn call<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::Client,
    url: U,
    variables: Q::Variables,
) -> Result<graphql_client::Response<Q::ResponseData>, reqwest::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = client.post(url).json(&body).send().await?;

    reqwest_response.json().await
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;

    fn init_github_client() -> Client {
        dotenvy::dotenv().unwrap();

        let github_api_token =
            std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

        reqwest::Client::builder()
            .user_agent("graphql-rust/0.12.0")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))
                        .unwrap(),
                ))
                .collect(),
            )
            .build()
            .expect("Failed to create reqwest client")
    }

    #[tokio::test]
    async fn test_query() {
        let client = init_github_client();
        let body = RepoByUser::build_query(repo_by_user::Variables {
            username: "egoavara".to_owned(),
            first: 20,
            after: None,
        });
        println!("{:#?}", serde_json::to_value(&body));
        let resp = client
            .post("https://api.github.com/graphql")
            .json(&body)
            .send()
            .await
            .unwrap();
        let result = resp.json::<graphql_client::Response<repo_by_user::ResponseData>>().await;

        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn pr_by_user() {
        let client = init_github_client();
        let body = PrByUser::build_query(pr_by_user::Variables {
            username: "egoavara".to_owned(),
            first: 20,
            after: None,
        });
        let resp = client
            .post("https://api.github.com/graphql")
            .json(&body)
            .send()
            .await
            .unwrap();
        let result = resp.json::<pr_by_user::ResponseData>().await;

        println!("{:#?}", result);
    }
}
