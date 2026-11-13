use discord_storage::*;
use discord_core::models::*;
use discord_core::snowflake::Snowflake;

#[test]
fn test_windows_dpapi_encryption_roundtrip() {
    let original = "discord_super_secret_user_token_mfa_authenticated_123456789";
    let protected = TokenStore::protect_string(original).unwrap();
    assert_ne!(protected, original.as_bytes());

    let unprotected = TokenStore::unprotect_string(&protected).unwrap();
    assert_eq!(unprotected, original);
}

#[tokio::test]
async fn test_cache_store_operations() {
    let cache = CacheStore::new(None::<&str>);
    let channel_id = Snowflake::new(55555);
    let msg_id = Snowflake::new(88888);

    let msg = Message {
        id: msg_id,
        channel_id,
        author: User {
            id: Snowflake::new(1),
            username: "Alice".to_string(),
            discriminator: "0".to_string(),
            global_name: Some("Alice G".to_string()),
            avatar: None,
            bot: None,
            system: None,
            mfa_enabled: None,
            banner: None,
            accent_color: None,
            locale: None,
            verified: None,
            email: None,
            flags: None,
            premium_type: None,
            public_flags: None,
        },
        content: "Initial message".to_string(),
        timestamp: "2026-11-13T23:00:00Z".to_string(),
        edited_timestamp: None,
        tts: false,
        mention_everyone: false,
        mentions: Vec::new(),
        mention_roles: Vec::new(),
        attachments: Vec::new(),
        embeds: Vec::new(),
        reactions: None,
        pinned: false,
        webhook_id: None,
        member: None,
        thread: None,
    };

    cache.append_message(msg.clone()).await;
    let messages = cache.get_messages(channel_id).await;
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Initial message");

    let mut edited = msg.clone();
    edited.content = "Edited message content".to_string();
    cache.edit_message(edited).await;

    let messages = cache.get_messages(channel_id).await;
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Edited message content");

    cache.delete_message(channel_id, msg_id).await;
    let messages = cache.get_messages(channel_id).await;
    assert_eq!(messages.len(), 0);
}
