use discord_core::*;

#[test]
fn test_snowflake_timestamp_extraction() {
    // Discord Epoch: 1420070400000 ms (2015-01-01T00:00:00Z)
    // Test snowflake ID: 175928847299117063
    let snowflake = Snowflake::new(175928847299117063);
    assert_eq!(snowflake.as_u64(), 175928847299117063);
    assert_eq!(snowflake.timestamp_millis(), 1462015105796);
    assert_eq!(snowflake.worker_id(), 1);
    assert_eq!(snowflake.process_id(), 0);
    assert_eq!(snowflake.increment(), 7);
}

#[test]
fn test_permission_hierarchy_and_overwrites() {
    let owner_id = Snowflake::new(100);
    let user_id = Snowflake::new(200);
    let guild_id = Snowflake::new(1);
    let admin_role_id = Snowflake::new(10);
    let member_role_id = Snowflake::new(20);

    let roles = vec![
        Role {
            id: guild_id,
            name: "@everyone".to_string(),
            color: 0,
            hoist: false,
            icon: None,
            unicode_emoji: None,
            position: 0,
            permissions: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
            managed: false,
            mentionable: false,
        },
        Role {
            id: admin_role_id,
            name: "Admin".to_string(),
            color: 0xFF0000,
            hoist: true,
            icon: None,
            unicode_emoji: None,
            position: 10,
            permissions: Permissions::ADMINISTRATOR,
            managed: false,
            mentionable: true,
        },
        Role {
            id: member_role_id,
            name: "Muted".to_string(),
            color: 0x00FF00,
            hoist: false,
            icon: None,
            unicode_emoji: None,
            position: 1,
            permissions: Permissions::empty(),
            managed: false,
            mentionable: false,
        },
    ];

    // Case 1: Owner gets all permissions
    let owner_perms = Permissions::compute_channel_permissions(
        owner_id,
        &[],
        owner_id,
        guild_id,
        &roles,
        &[],
    );
    assert!(owner_perms.contains(Permissions::ADMINISTRATOR));
    assert!(owner_perms.contains(Permissions::MANAGE_GUILD));

    // Case 2: Administrator role gets all permissions even if channel denies VIEW_CHANNEL
    let channel_overwrites = vec![
        PermissionOverwrite {
            id: guild_id,
            overwrite_type: 0,
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
        },
    ];
    let admin_perms = Permissions::compute_channel_permissions(
        user_id,
        &[admin_role_id],
        owner_id,
        guild_id,
        &roles,
        &channel_overwrites,
    );
    assert!(admin_perms.contains(Permissions::VIEW_CHANNEL));

    // Case 3: Regular member denied VIEW_CHANNEL by channel overwrite
    let regular_perms = Permissions::compute_channel_permissions(
        user_id,
        &[],
        owner_id,
        guild_id,
        &roles,
        &channel_overwrites,
    );
    assert!(!regular_perms.contains(Permissions::VIEW_CHANNEL));
}

#[test]
fn test_discord_markdown_parser() {
    let input = "Hello **world**, ||secret spoiler|| and <#123456789> with `code`!";
    let nodes = DiscordMarkdownParser::parse(input);
    assert_eq!(nodes.len(), 9);

    assert_eq!(nodes[0], MarkdownNode::Text("Hello ".to_string()));
    assert_eq!(nodes[1], MarkdownNode::Bold(vec![MarkdownNode::Text("world".to_string())]));
    assert_eq!(nodes[2], MarkdownNode::Text(", ".to_string()));
    assert_eq!(nodes[3], MarkdownNode::Spoiler(vec![MarkdownNode::Text("secret spoiler".to_string())]));
    assert_eq!(nodes[4], MarkdownNode::Text(" and ".to_string()));
    assert_eq!(nodes[5], MarkdownNode::ChannelMention(Snowflake::new(123456789)));
    assert_eq!(nodes[6], MarkdownNode::Text(" with ".to_string()));
    assert_eq!(nodes[7], MarkdownNode::InlineCode("code".to_string()));
    assert_eq!(nodes[8], MarkdownNode::Text("!".to_string()));
}
