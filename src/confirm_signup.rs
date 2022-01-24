//! A simple example of the AWS cognitoidentityprovider
//! [Client confirm_sign_up method](https://docs.rs/aws-sdk-cognitoidentityprovider/0.5.2/aws_sdk_cognitoidentityprovider/client/struct.Client.html#method.confirm_sign_up)

use aws_sdk_cognitoidentityprovider::error::ConfirmSignUpError;
use aws_sdk_cognitoidentityprovider::output::ConfirmSignUpOutput;
use aws_sdk_cognitoidentityprovider::{Client, SdkError};
use std::io;

/// Submits the verification code sent to the signup email by AWS Cognito. If accomplishes this by:
/// - First, getting the confirmation code from user terminal input.
/// - Second, submitting the confirm_sign_up method with the appropriate details.
///
/// Expects the variables:
/// - client which is the Client instantiated in this example's bin.rs
/// - client_id which represents the Client ID from AWS Cognito's App client list.
/// - username which is the username being confirmed.
///
/// Returns the result of either a successful confirmation with ConfirmSignUpOutput or an error with
/// ConfirmSignUpError.
pub async fn verify(
    client: &Client,
    client_id: &str,
    username: &str,
) -> Result<ConfirmSignUpOutput, SdkError<ConfirmSignUpError>> {
    // Get verification code from terminal input.
    println!("Please input verification code (check email): ");
    let mut verification_input = String::new();
    io::stdin()
        .read_line(&mut verification_input)
        .expect("Failed to read verification code.");
    verification_input = verification_input.trim_end().parse().unwrap();

    // Send the confirm_sign_up method with the client_id, username, and confirmation_code.
    let verification = client
        .confirm_sign_up()
        .client_id(client_id)
        .username(username)
        .confirmation_code(&verification_input)
        .send()
        .await;

    verification
}
