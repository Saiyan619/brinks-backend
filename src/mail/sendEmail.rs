use std::{env, fmt::Error};
use lettre::{Message, SmtpTransport, Transport, message::{SinglePart, header}, transport::smtp::authentication::Credentials};
// use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::errors::{ErrorMessage, HttpError};

// pub async fn send_email_verification(email_to: Vec<&str>, email_from:String, token:String, subject:String) -> Result<(), HttpError>{
//     let resend_key = env::var("RESEND_API_KEY").unwrap();
//     let resend = Resend::new(&resend_key);
//     let verification_url = format!("http://localhost:5173/verify-email?token={}", token);
//     let html_content = format!(
//         r#"
//         <div style="font-family: sans-serif; padding: 20px; max-width: 500px; margin: 0 auto; border: 1px solid #eee; border-radius: 8px;">
//             <h2 style="color: #333;">Verify your account</h2>
//             <p style="color: #555; line-height: 1.5;">Thank you for registering! Please click the button below to verify your email address and activate your account:</p>
//             <p style="margin-top: 25px;">
//                 <a href="{url}" style="background: #000; color: #fff; padding: 12px 24px; text-decoration: none; border-radius: 5px; font-weight: bold; display: inline-block;">
//                     Verify Email Address
//                 </a>
//             </p>
//             <p style="margin-top: 25px; font-size: 12px; color: #999;">If the button doesn't work, copy and paste this link into your browser:<br>{url}</p>
//         </div>
//         "#,
//         url = verification_url
//     );

//     let email = CreateEmailBaseOptions::new(email_from, email_to, subject)
//     .with_html(&html_content);

//     let _email = resend.emails.send(email).await.map_err(|_| HttpError::server_error(ErrorMessage::EmailError.return_err()))?;

//     println!("{:?}", _email);

//     Ok(())
// }

pub async fn send_email_verification(
    to_email: String,
    token: String
) -> Result<(), Box<dyn std::error::Error>>{
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_server = env::var("SMTP_SERVER")?;
    let smtp_port: u16 = env::var("SMTP_PORT")?.parse()?;
    let smtp_from_address = env::var("SMTP_FROM_ADDRESS")?; 

    let subject = "Email Verification";
    let base_url = "http://localhost:5173/verify-email";
    let verification_url = create_verification_link(base_url, token);

    let html_template = format!(
        r#"
        <div style="font-family: sans-serif; padding: 20px; max-width: 500px; margin: 0 auto; border: 1px solid #eee; border-radius: 8px;">
            <h2 style="color: #333;">Verify your account</h2>
            <p style="color: #555; line-height: 1.5;">Thank you for registering! Please click the button below to verify your email address and activate your account:</p>
            <p style="margin-top: 25px;">
                <a href="{url}" style="background: #000; color: #fff; padding: 12px 24px; text-decoration: none; border-radius: 5px; font-weight: bold; display: inline-block;">
                    Verify Email Address
                </a>
            </p>
            <p style="margin-top: 25px; font-size: 12px; color: #999;">If the button doesn't work, copy and paste this link into your browser:<br>{url}</p>
        </div>
        "#,
        url = verification_url
    );

    let email = Message::builder()
        .from(smtp_from_address.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(html_template)
        )?;

    let creds = Credentials::new(smtp_username.clone(), smtp_password.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(creds)
        .port(smtp_port)
        .build();
    
    let result = mailer.send(&email);

    match result {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Failed to send email: {:?}", e),
    }

    Ok(())
}



fn create_verification_link(base_url: &str, token: String) -> String {
    format!("{}?token={}", base_url, token)
}
