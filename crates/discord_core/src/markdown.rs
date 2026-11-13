use serde::{Deserialize, Serialize};
use crate::snowflake::Snowflake;

/// Markdown Abstract Syntax Tree Node for Discord Messages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkdownNode {
    Text(String),
    Bold(Vec<MarkdownNode>),
    Italic(Vec<MarkdownNode>),
    Underline(Vec<MarkdownNode>),
    Strikethrough(Vec<MarkdownNode>),
    Spoiler(Vec<MarkdownNode>),
    InlineCode(String),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    BlockQuote(Vec<MarkdownNode>),
    Heading {
        level: u8,
        children: Vec<MarkdownNode>,
    },
    UserMention {
        id: Snowflake,
        nickname_mention: bool,
    },
    RoleMention(Snowflake),
    ChannelMention(Snowflake),
    CustomEmoji {
        name: String,
        id: Snowflake,
        animated: bool,
    },
    Link {
        title: String,
        url: String,
    },
    Url(String),
}

/// Tokenizer and recursive parser for Discord Markdown
pub struct DiscordMarkdownParser;

impl DiscordMarkdownParser {
    pub fn parse(input: &str) -> Vec<MarkdownNode> {
        let mut nodes = Vec::new();
        let chars: Vec<char> = input.chars().collect();
        let mut idx = 0;

        while idx < chars.len() {
            // Code block: ```[lang]\n...\n```
            if chars[idx..].starts_with(&['`', '`', '`']) {
                idx += 3;
                let mut lang = String::new();
                while idx < chars.len() && chars[idx] != '\n' && chars[idx] != '`' {
                    lang.push(chars[idx]);
                    idx += 1;
                }
                if idx < chars.len() && chars[idx] == '\n' {
                    idx += 1;
                }
                let mut code = String::new();
                while idx < chars.len() && !chars[idx..].starts_with(&['`', '`', '`']) {
                    code.push(chars[idx]);
                    idx += 1;
                }
                if idx < chars.len() && chars[idx..].starts_with(&['`', '`', '`']) {
                    idx += 3;
                }
                nodes.push(MarkdownNode::CodeBlock {
                    language: if lang.trim().is_empty() { None } else { Some(lang.trim().to_string()) },
                    code,
                });
                continue;
            }

            // Inline code: `...`
            if chars[idx] == '`' {
                idx += 1;
                let mut code = String::new();
                while idx < chars.len() && chars[idx] != '`' {
                    code.push(chars[idx]);
                    idx += 1;
                }
                if idx < chars.len() && chars[idx] == '`' {
                    idx += 1;
                }
                nodes.push(MarkdownNode::InlineCode(code));
                continue;
            }

            // Spoiler: ||...||
            if chars[idx..].starts_with(&['|', '|']) {
                idx += 2;
                let mut inner = String::new();
                while idx < chars.len() && !chars[idx..].starts_with(&['|', '|']) {
                    inner.push(chars[idx]);
                    idx += 1;
                }
                if idx < chars.len() && chars[idx..].starts_with(&['|', '|']) {
                    idx += 2;
                }
                nodes.push(MarkdownNode::Spoiler(Self::parse(&inner)));
                continue;
            }

            // Strikethrough: ~~...~~
            if chars[idx..].starts_with(&['~', '~']) {
                idx += 2;
                let mut inner = String::new();
                while idx < chars.len() && !chars[idx..].starts_with(&['~', '~']) {
                    inner.push(chars[idx]);
                    idx += 1;
                }
                if idx < chars.len() && chars[idx..].starts_with(&['~', '~']) {
                    idx += 2;
                }
                nodes.push(MarkdownNode::Strikethrough(Self::parse(&inner)));
                continue;
            }

            // Bold: **...**
            if chars[idx..].starts_with(&['*', '*']) {
                idx += 2;
                let mut inner = String::new();
                while idx < chars.len() && !chars[idx..].starts_with(&['*', '*']) {
                    inner.push(chars[idx]);
                    idx += 1;
                }
                if idx < chars.len() && chars[idx..].starts_with(&['*', '*']) {
                    idx += 2;
                }
                nodes.push(MarkdownNode::Bold(Self::parse(&inner)));
                continue;
            }

            // Mentions: <@...>, <#...>, <:name:id>, <a:name:id>
            if chars[idx] == '<' {
                let start = idx;
                let mut tag = String::new();
                while idx < chars.len() && chars[idx] != '>' {
                    tag.push(chars[idx]);
                    idx += 1;
                }
                if idx < chars.len() && chars[idx] == '>' {
                    tag.push('>');
                    idx += 1;
                    if let Some(mention) = Self::parse_mention(&tag) {
                        nodes.push(mention);
                        continue;
                    }
                }
                idx = start; // Fallback to normal text
            }

            // Normal text
            let mut text = String::new();
            while idx < chars.len()
                && chars[idx] != '`'
                && chars[idx] != '<'
                && !chars[idx..].starts_with(&['*', '*'])
                && !chars[idx..].starts_with(&['|', '|'])
                && !chars[idx..].starts_with(&['~', '~'])
            {
                text.push(chars[idx]);
                idx += 1;
            }
            if !text.is_empty() {
                nodes.push(MarkdownNode::Text(text));
            }
        }

        nodes
    }

    fn parse_mention(tag: &str) -> Option<MarkdownNode> {
        if tag.starts_with("<@&") && tag.ends_with('>') {
            let id_str = &tag[3..tag.len() - 1];
            id_str.parse::<u64>().ok().map(|id| MarkdownNode::RoleMention(Snowflake::new(id)))
        } else if tag.starts_with("<@!") && tag.ends_with('>') {
            let id_str = &tag[3..tag.len() - 1];
            id_str.parse::<u64>().ok().map(|id| MarkdownNode::UserMention {
                id: Snowflake::new(id),
                nickname_mention: true,
            })
        } else if tag.starts_with("<@") && tag.ends_with('>') {
            let id_str = &tag[2..tag.len() - 1];
            id_str.parse::<u64>().ok().map(|id| MarkdownNode::UserMention {
                id: Snowflake::new(id),
                nickname_mention: false,
            })
        } else if tag.starts_with("<#") && tag.ends_with('>') {
            let id_str = &tag[2..tag.len() - 1];
            id_str.parse::<u64>().ok().map(|id| MarkdownNode::ChannelMention(Snowflake::new(id)))
        } else if (tag.starts_with("<:") || tag.starts_with("<a:")) && tag.ends_with('>') {
            let animated = tag.starts_with("<a:");
            let prefix_len = if animated { 3 } else { 2 };
            let parts: Vec<&str> = tag[prefix_len..tag.len() - 1].split(':').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                if let Ok(id) = parts[1].parse::<u64>() {
                    return Some(MarkdownNode::CustomEmoji {
                        name,
                        id: Snowflake::new(id),
                        animated,
                    });
                }
            }
            None
        } else {
            None
        }
    }
}
