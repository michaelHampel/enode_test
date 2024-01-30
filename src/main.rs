
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let client = reqwest::Client::builder().build()?;

    let res = client
        .get("http://httpbin.org/get")
        .send()
        .await?;
    
    println!("Http Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Response body: \n{}", body);

    Ok(())

}
