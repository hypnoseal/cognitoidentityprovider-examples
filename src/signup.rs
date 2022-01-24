//! A simple AWS cognitoidentityprovider [Client sign_up method](https://docs.rs/aws-sdk-cognitoidentityprovider/0.5.2/aws_sdk_cognitoidentityprovider/client/struct.Client.html#method.sign_up)
//! for terminal with prompts.

use aws_sdk_cognitoidentityprovider::model::AttributeType;
use aws_sdk_cognitoidentityprovider::{Client, Error};
use std::collections::HashMap;
use std::io;

/// Gets email, username, and password then initiates a sign_up call with these variables.
///
/// On success, returns the user_credentials for use in other examples.
///
/// On failure, panics with an Error.
pub async fn initiate(client: &Client, client_id: &str) -> Result<HashMap<String, String>, Error> {
    // Gets email from user terminal input.
    println!("Please input email: ");
    let mut email_input = String::new();
    io::stdin()
        .read_line(&mut email_input)
        .expect("Failed to read email input.");
    email_input = email_input.trim_end().parse().unwrap();

    // Sets email_input as an AttributeType for use in the sign_up call.
    let email_attribute = AttributeType::builder()
        .set_name(Some("email".to_string()))
        .set_value(Some(email_input))
        .build();

    // Gets username from terminal input.
    println!("Please input username: ");
    let mut username_input = String::new();
    io::stdin()
        .read_line(&mut username_input)
        .expect("Failed to read username input.");
    username_input = username_input.trim_end().parse().unwrap();

    // Gets password from terminal input and protects this input from view with rpassword.
    let mut password_input = rpassword::prompt_password_stdout(
        "Please input password (AWS Cognito default password requirements): ",
    )
    .unwrap();
    password_input = password_input.trim_end().parse().unwrap();

    // Initiates the signup process with the variables set above.
    let _signup = client
        .sign_up()
        .client_id(client_id)
        .username(&username_input)
        .password(&password_input)
        .set_user_attributes(Some(vec![email_attribute]))
        .send()
        .await
        .unwrap();

    println!("Signup successful!");

    // Assign user_credentials for use in other examples. Likely not something you would want to do
    // in an actual project.
    let user_credentials = HashMap::from([
        ("USERNAME".to_string(), username_input.clone()),
        ("PASSWORD".to_string(), password_input.clone()),
    ]);

    Ok(user_credentials)
}
