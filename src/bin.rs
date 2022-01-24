//! Simple binary tying together various modules that implement the AWS Rust SDK for
//! cognitoidentityprovider.
//!
//! # Organization
//!
//! The directory structure for these examples is a flat hierarchy. There are presently three
//! examples of interest:
//!
//! 1. signup
//! 2. confirm_signup
//! 3. auth
//!
//! This binary pulls the AWS Cognito's region and/or client-id from environment variables or
//! terminal argument and then sets up an [aws_sdk_cognitoidentityprovider Client](https://docs.rs/aws-sdk-cognitoidentityprovider/0.5.2/aws_sdk_cognitoidentityprovider/client/struct.Client.html).
//! It then runs through the three examples mentioned.

use aws_config::meta::region::RegionProviderChain;
use aws_config::RetryConfig;
use aws_sdk_cognitoidentityprovider::{Client, Error, Region};
use structopt::StructOpt;

use cognitoidentityprovider_examples::*;

/// Opt for StructOpt including both region and client_id.
#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The AWS Client ID.
    #[structopt(short, long)]
    client_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get env variables
    let Opt { region, client_id } = Opt::from_args();

    // Set AWS region and config.
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let config = aws_sdk_cognitoidentityprovider::config::Builder::from(&shared_config)
        .retry_config(RetryConfig::disabled())
        .build();
    println!("AWS Region set to: {:?}", shared_config.region().unwrap());

    // Instantiate a Client with config.
    let client = Client::from_conf(config);
    println!("Client ready! Preparing signup...");

    // Set client_id from env args, panics if if no client_id.
    let client_id = match client_id {
        Some(c) => {
            println!("Client id is set to: {:?}", &c);
            c
        }
        None => {
            panic!("client-id missing from env arguments.");
        }
    };

    // Initiate a signup with signup module. Set returned Hashmap as user credentials.
    let user_credentials = signup::initiate(&client, &client_id).await.unwrap();

    // Verify email with confirm_signup module.
    let verification_status = confirm_signup::verify(
        &client,
        &client_id,
        user_credentials.get("USERNAME").unwrap(),
    )
    .await;

    // Match result of verification.
    match verification_status {
        Ok(_p) => {
            println!("Verification success!")
        }
        Err(e) => {
            println!("Verification error: {:?}", e);
        }
    }

    // Authenticate user_credentials with auth module.
    let auth = auth::initiate(&client, &client_id, &user_credentials).await;

    // Match auth results. Prints to terminal the access token if successful. Otherwise, prints the
    // error and returns the error.
    match auth {
        Ok(a) => {
            println!("Authentication successful!");
            let access_token = a.authentication_result.unwrap().access_token;
            println!("Access token is: {:?}", &access_token);
            Ok(())
        }
        Err(e) => {
            println!("Authentication error: {:?}", e);
            Err(Error::from(e))
        }
    }
}
