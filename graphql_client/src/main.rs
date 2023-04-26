use cart_create::{CartBuyerIdentityInput, CartInput};
use graphql_client::{GraphQLQuery, Response};
use reqwest::{
    self,
    header::{self, HeaderMap},
    Client,
};
use std::error::Error;
use tokio;

const URI: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";
// TODO:
const TOKEN: &str = "a0a1c9d687b1a6b1bbea52dbb2025277";

type URL = String;
const URL: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.graphql",
    query_path = "./src/products_query.graphql",
    response_derives = "Debug"
)]
pub struct ProductsQuery;

async fn query_products(variables: products_query::Variables) -> Result<(), Box<dyn Error>> {
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.graphql",
    query_path = "./src/cart_create.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CartCreate;

async fn cart_create_graphql(variables: cart_create::Variables) -> Result<(), Box<dyn Error>> {
    let request_body = CartCreate::build_query(variables);
    let mut header = HeaderMap::new();
    header.insert(
        "X-Shopify-Storefront-Access-Token",
        header::HeaderValue::from_str(TOKEN).unwrap(),
    );

    let client = Client::builder().default_headers(header).build()?;

    let res = client.post(URL).json(&request_body).send().await?;
    let response_body: Response<cart_create::ResponseData> = res.json().await?;

    match response_body.data {
        Some(r) => {
            println!(
                "cart id: {:#?}",
                r.clone().cart_create.unwrap().cart.unwrap().id
            );
            println!(
                "cart checkout_url: {:#?}",
                r.cart_create.unwrap().cart.unwrap().checkout_url
            );
        }
        _ => (),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let input: products_query::Variables = products_query::Variables { first: 100 };
    // query_products(input).await?;

    let iden = CartBuyerIdentityInput {
        country_code: None,
        customer_access_token: Some("c53b4857b05fb46fe88c396f4374d77d".to_string()),
        delivery_address_preferences: None,
        email: None,
        phone: None,
    };

    let cart_input = CartInput {
        attributes: None,
        buyer_identity: Some(iden),
        discount_codes: None,
        lines: None,
        note: None,
    };

    let input = cart_create::Variables {
        input: Some(cart_input),
    };

    cart_create_graphql(input).await?;
    return Ok(());
}
