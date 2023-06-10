use futures_util::StreamExt;
use openai::api::models::req::{self, PostConversationRequest};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let email = std::env::var("EMAIL")?;
    let password = std::env::var("PASSWORD")?;
    let store = openai::token::FileStore::default();
    let mut auth = openai::oauth::OAuthBuilder::builder()
        .email(email)
        .password(password)
        .cache(true)
        .cookie_store(true)
        .token_store(store)
        .client_timeout(std::time::Duration::from_secs(20))
        .build();
    let token = auth.do_get_access_token().await?;
    let api = openai::api::opengpt::OpenGPTBuilder::builder()
        .access_token(token.access_token().to_owned())
        .cookie_store(false)
        .build();

    // check account status
    let resp = api.get_account_check().await?;
    println!("{:#?}", resp);
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // get models
    let resp = api.get_models().await?;

    let req = req::PostNextConversationBodyBuilder::default()
        .model(resp.models[0].slug.to_string())
        .prompt("Java Example".to_string())
        .build()?;

    let mut resp: openai::api::PostConversationStreamResponse = api
        .post_conversation_stream(PostConversationRequest::Next(req))
        .await?;

    let mut previous_response = String::new();
    let mut out: tokio::io::Stdout = tokio::io::stdout();

    while let Some(ele) = resp.next().await {
        let message = &ele.message()[0];
        if message.starts_with(&previous_response) {
            let new_chars: String = message.chars().skip(previous_response.len()).collect();
            out.write_all(new_chars.as_bytes()).await?;
        } else {
            out.write_all(message.as_bytes()).await?;
        }
        out.flush().await?;
        previous_response = message.to_string();
    }

    // get conversation
    // let req = req::GetConversationRequestBuilder::default()
    //     .conversation_id("78feb7c4-a864-4606-8665-cdb7a1cf4f6d".to_owned())
    //     .build()?;
    // let resp = api.get_conversation(req).await?;
    // println!("{:#?}", resp);
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // get conversation list
    // let req = req::GetConversationRequestBuilder::default()
    //     .offset(0)
    //     .limit(20)
    //     .build()?;
    // let resp = api.get_conversations(req).await?;
    // println!("{:#?}", resp);
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // // clart conversation
    // let req = req::PatchConversationRequestBuilder::default()
    //     .conversation_id("3de1bd20-ecea-4bf7-96f5-b8eb681b180d".to_owned())
    //     .is_visible(false)
    //     .build()?;
    // let resp = api.patch_conversation(req).await?;
    // println!("{:#?}", resp);
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // // clart conversation list
    // let req = req::PatchConversationRequestBuilder::default()
    //     .is_visible(false)
    //     .build()?;
    // let resp = api.patch_conversations(req).await?;
    // println!("{:#?}", resp);
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // rename conversation title
    // let req = req::PatchConversationRequestBuilder::default()
    //     .conversation_id("78feb7c4-a864-4606-8665-cdb7a1cf4f6d".to_owned())
    //     .title("fuck".to_owned())
    //     .build()?;
    // let resp = api.patch_conversation(req).await?;
    // println!("{:#?}", resp);
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // // message feedback
    // let req = req::MessageFeedbackRequestBuilder::default()
    //     .message_id("463a23c4-0855-4c5b-976c-7697519335ad".to_owned())
    //     .conversation_id("78feb7c4-a864-4606-8665-cdb7a1cf4f6d".to_owned())
    //     .rating(req::Rating::ThumbsUp)
    //     .build()?;
    // let resp = api.message_feedback(req).await?;
    // println!("{:#?}", resp);

    Ok(())
}
