//! A simple example of the AWS cognitoidentityprovider
//! [Client initiate_auth method](https://docs.rs/aws-sdk-cognitoidentityprovider/0.5.2/aws_sdk_cognitoidentityprovider/client/struct.Client.html#method.initiate_auth)

use aws_sdk_cognitoidentityprovider::error::InitiateAuthError;
use aws_sdk_cognitoidentityprovider::model::AuthFlowType;
use aws_sdk_cognitoidentityprovider::output::InitiateAuthOutput;
use aws_sdk_cognitoidentityprovider::{Client, SdkError};
use std::collections::HashMap;

/// Initiates an [Client initiate_auth method](https://docs.rs/aws-sdk-cognitoidentityprovider/0.5.2/aws_sdk_cognitoidentityprovider/client/struct.Client.html#method.initiate_auth).
///
/// Expects the variables:
/// - client which is the Client instantiated in this example's bin.rs
/// - client_id which represents the Client ID from AWS Cognito's App client list.
/// - user_credentials which are the user credentials passed from the signup module.
///
/// Returns the result of either the successful initiate auth or an error.
pub async fn initiate(
    client: &Client,
    client_id: &str,
    user_credentials: &HashMap<String, String>,
) -> Result<InitiateAuthOutput, SdkError<InitiateAuthError>> {
    let auth = client
        .initiate_auth()
        .auth_flow(AuthFlowType::UserPasswordAuth)
        .client_id(client_id)
        .set_auth_parameters(Some(user_credentials.clone()))
        .send()
        .await;

    auth
}
