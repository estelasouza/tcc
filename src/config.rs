use mongodb::{Client, options::{ClientOptions, ResolverConfig}};

#[tokio::main]
pub async fn settings() ->  Result<Vec<String>,mongodb::error::Error>  {
    let client_uri = "mongodb+srv://admin:Test10@cluster0.ud6jftd.mongodb.net/?retryWrites=true&w=majority";

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

   Client::with_options(options)?.list_database_names(None,None).await

    
    
}
