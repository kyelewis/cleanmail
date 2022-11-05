use std::{ env };

use futures::TryStreamExt; // try_collect()

#[tokio::main]
async fn main() {
  eprintln!("✉️ CleanMail CLI 0.1.0 - built with ❤️ by kye@kyedoesdev.com");

  let domain = match env::var("CLEAN_MAIL_HOST") {
      Ok(val) => val,
      Err(error) => panic!("CLEAN_MAIL_HOST: {error}")
  };

  let user = match env::var("CLEAN_MAIL_USER") {
      Ok(val) => val,
      Err(error) => panic!("CLEAN_MAIL_USER: {error}")
  };

  let password = match env::var("CLEAN_MAIL_PASSWORD") {
      Ok(val) => val,
      Err(error) => panic!("CLEAN_MAIL_PASSWORD: {error}")
  };

  let tls = async_native_tls::TlsConnector::new();

  let client = async_imap::connect((String::from(&domain), 993),  String::from(&domain) , tls).await.unwrap();

  eprintln!("Logging In");

  let mut imap_session = client
      .login(user, password)
      .await
      .unwrap();

  imap_session.examine("INBOX").await.unwrap();

  let messages = match imap_session.fetch("1", "RFC822").await {
      Ok(messages) => messages,
      Err(error) => panic!("Error fetching messages: {error}")
  };
 

  let messages: Vec<_> = match messages.try_collect().await {
      Ok(messages) => messages,
      Err(error) => panic!("Couldn't read messages: {error}")
  };

  eprintln!("You have {} messages", messages.len());

  let _ = imap_session.logout().await;

}
