use graphql_client::{GraphQLQuery, Response};
use reqwest::{
    self,
    header::{self, HeaderMap},
    Client,
};
use std::error::Error;
use tokio;

const URI: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";
const TOKEN: &str = "********************************";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.graphql",
    query_path = "./src/products_query.graphql",
    response_derives = "Debug"
)]
pub struct ProductsQuery;

async fn perform_my_query(variables: products_query::Variables) -> Result<(), Box<dyn Error>> {
    let request_body = ProductsQuery::build_query(variables);
    let mut header = HeaderMap::new();
    header.insert(
        "X-Shopify-Storefront-Access-Token",
        header::HeaderValue::from_str(TOKEN).unwrap(),
    );

    let client = Client::builder().default_headers(header).build()?;

    let res = client.post(URI).json(&request_body).send().await?;
    let response_body: Response<products_query::ResponseData> = res.json().await?;
    match response_body.data {
        Some(r) => {
            for i in r.products.edges {
                println!("{:#?}", i.node.id);
                println!("---------------");
            }
        }
        _ => (),
    }

    Ok(())
}

type URL = String;
const URL: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.graphql",
    query_path = "./src/cart_create.graphql",
    response_derives = "Debug"
)]
pub struct CartCreate;

async fn cart_create_graphql(variables: cart_create::Variables) -> Result<(), Box<dyn Error>> {
    let request_body = cart_create::Variables;
    let mut header = HeaderMap::new();
    header.insert(
        "X-Shopify-Storefront-Access-Token",
        header::HeaderValue::from_str(TOKEN).unwrap(),
    );

    let client = Client::builder().default_headers(header).build()?;

    let res = client.post(URI).json(&request_body).send().await?;
    let response_body: Response<products_query::ResponseData> = res.json().await?;
    match response_body.data {
        Some(r) => {
            for i in r.products.edges {
                println!("{:#?}", i.node.id);
                println!("---------------");
            }
        }
        _ => (),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input: products_query::Variables = products_query::Variables { first: 100 };
    perform_my_query(input).await?;
    return Ok(());
}
