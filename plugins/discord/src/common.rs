use serde::Serialize;
#[macro_export]
macro_rules! new {
    () => {
        #[must_use]
        #[doc = "Creates a new empty struct."]
        pub fn new() -> Self {
            Self::default()
        }
    };
}

#[macro_export]
macro_rules! override_field {
    ($name:ident, $type:ty) => {
        #[doc = concat!("Adds `", stringify!($name), "` field.")]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = Some($name);
            self
        }
    };
}

#[macro_export]
macro_rules! initialize_field {
    ($name:ident, $type:ty) => {
        #[doc = concat!("Adds `", stringify!($name), "` field.")]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = $name;
            self
        }
    };
}

/// Representation of discord message
#[derive(Default, Serialize, Clone)]
pub struct DiscordMessage {
    /// the message contents (up to 2000 characters)
    pub content: String,
    /// override the default username of the webhook
    pub username: Option<String>,
    /// override the default avatar of the webhook
    pub avatar_url: Option<String>,
    /// true if this is a TTS message
    pub tts: Option<bool>,
    /// embedded [rich](DiscordEmbedType::Rich) content (upto 10 embeds)
    pub embeds: Option<Vec<DiscordEmbed>>,
    /// allowed mentions for the message
    pub allowed_mentions: Option<DiscordAllowedMentions>,
    /// the components to include with the message
    pub components: Option<Vec<DiscordMessageComponent>>,
    // attachment,files[n], payload_json, flags, thread_name, applied_tags, poll isn't supported yet
}

impl DiscordMessage {
    new!();
    initialize_field!(content, String);
    override_field!(username, String);
    override_field!(avatar_url, String);
    override_field!(tts, bool);
    override_field!(embeds, Vec<DiscordEmbed>);
    override_field!(allowed_mentions, DiscordAllowedMentions);
    override_field!(components, Vec<DiscordMessageComponent>);
}

#[derive(Default, Serialize, Clone)]
pub struct DiscordEmbed {
    /// title of embed
    pub title: Option<String>,
    // field "type" isn't allowed
    /// [type of embed](DiscordEmbedType) (always ["rich"](DiscordEmbedType::Rich) for webhook embeds)
    pub r#type: DiscordEmbedType,
    /// description of embed
    pub description: Option<String>,
    /// url of embed
    pub url: Option<String>,
    // timestamp is optional, so ignoring it
    // pub timestamp: Option<>
    /// color code of the embed, use [DiscordEmbed::color] to use hex code
    pub color: Option<i64>,
    /// footer information
    pub footer: Option<DiscordFooterEmbed>,
    /// image information
    pub image: Option<DiscordImageEmbed>,
    /// thumbnail information [DiscordImageEmbed] is also used for it
    pub thumbnail: Option<DiscordImageEmbed>,
    /// video information [DiscordImageEmbed] is also used for it
    pub video: Option<DiscordImageEmbed>,
    /// provider information
    pub provider: Option<DiscordProviderEmbed>,
    /// fields information, max of 25
    pub fields: Option<Vec<DiscordFieldEmbed>>,
    /// author information
    pub author: Option<DiscordAuthorEmbed>
}

impl DiscordEmbed {
    new!();
    override_field!(title, String);
    override_field!(description, String);
    override_field!(url, String);
    override_field!(footer, DiscordFooterEmbed);
    override_field!(image, DiscordImageEmbed);
    override_field!(thumbnail, DiscordImageEmbed);
    override_field!(video, DiscordImageEmbed);
    override_field!(provider, DiscordProviderEmbed);
    override_field!(fields, Vec<DiscordFieldEmbed>);
    override_field!(author, DiscordAuthorEmbed);
    initialize_field!(r#type, DiscordEmbedType);

    /// color should be hex value without `#`
    pub fn color(mut self, color: &str) -> Self {
        let parsed_color: i64 = i64::from_str_radix(color, 16).expect("Unable to parse color code");

        self.color = Some(parsed_color);
        self
    }
}

#[derive(Default, Serialize, Clone)]
pub enum DiscordEmbedType {
    /// generic embed rendered from embed attributes
    #[default]
    Rich,
    /// image embed
    Image,
    /// video embed
    Video,
    /// animated gif image embed rendered as a video embed
    GIFV,
    /// article embed
    Article,
    /// link embed
    Link
}

#[derive(Default, Serialize, Clone)]
pub struct DiscordFooterEmbed {
    /// footer text
    pub text: String,
    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// a proxied url of footer icon
    pub proxy_icon_url: Option<String>
}

impl DiscordFooterEmbed {
    new!();
    initialize_field!(text, String);
    override_field!(icon_url, String);
    override_field!(proxy_icon_url, String);
}

#[derive(Default, Serialize, Clone)]
pub struct DiscordFieldEmbed {
    /// name of the field
    pub name: String,
    /// value of the field
    pub value: String,
    /// whether or not this field should display inline
    pub inline: Option<bool>
}

impl DiscordFieldEmbed {
    new!();
    initialize_field!(name, String);
    initialize_field!(value, String);
    override_field!(inline, bool);
}

#[derive(Default, Serialize, Clone)]
pub struct DiscordImageEmbed {
    /// source url of image (only supports http(s) and attachments)
    pub url: String,
    /// a proxied url of the image
    pub proxy_url: Option<String>,
    /// height of image
    pub height: Option<i64>,
    /// width of image
    pub width: Option<i64>
}

impl DiscordImageEmbed {
    new!();
    initialize_field!(url, String);
    override_field!(proxy_url, String);
    override_field!(height, i64);
    override_field!(width, i64);
}

#[derive(Default, Serialize, Clone)]
pub struct DiscordProviderEmbed {
    /// name of provider
    pub name: Option<String>,
    /// url of provider
    pub url: Option<String>
}

impl DiscordProviderEmbed {
    new!();
    override_field!(name, String);
    override_field!(url, String);
}

#[derive(Default, Serialize, Clone)]
pub struct DiscordAuthorEmbed {
    /// name of author
    pub name: String,
    /// url of author (only supports http(s))
    pub url: Option<String>,
    /// url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// a proxied url of author icon
    pub proxy_icon_url: Option<String>
}

impl DiscordAuthorEmbed {
    new!();
    initialize_field!(name, String);
    override_field!(url, String);
    override_field!(icon_url, String);
    override_field!(proxy_icon_url, String);
}

#[derive(Default, Serialize, Clone)]
pub struct DiscordAllowedMentions {
    /// An array of [allowed mention types](DiscordAllowedMentionTypes) to parse from the content.
    pub parse: Option<Vec<DiscordAllowedMentionTypes>>,
    /// Array of role_ids to mention (Max size of 100)
    pub roles: Option<Vec<String>>,
    /// Array of role_ids to mention (Max size of 100)
    pub users: Option<Vec<String>>,
    /// For replies, whether to mention the author of the message being replied to (default false)
    pub replied_user: bool
}

impl DiscordAllowedMentions {
    new!();
    override_field!(parse, Vec<DiscordAllowedMentionTypes>);
    override_field!(roles, Vec<String>);
    override_field!(users, Vec<String>);
    initialize_field!(replied_user, bool);
}

#[derive(Serialize, Clone)]
pub enum DiscordAllowedMentionTypes {
    /// Controls role mentions
    Roles,
    /// Controls user mentions
    Users,
    /// Controls `@everyone` and `@here` mentions
    Everyone
}

#[derive(Default, Serialize, Clone)]
/// See <https://discord.com/developers/docs/interactions/message-components#what-is-a-component> to
/// learn more
pub struct DiscordMessageComponent {
    pub r#type: u8,
    pub label: Option<String>,
    pub style: Option<String>,
    pub custom_id: Option<String>,
    pub components: Option<Vec<DiscordMessageComponent>>
}

impl DiscordMessageComponent {
    new!();
    override_field!(label, String);
    override_field!(style, String);
    override_field!(custom_id, String);
    override_field!(components, Vec<DiscordMessageComponent>);

    pub fn r#type(mut self, r#type: DiscordMessageComponentTypes) -> Self {
        self.r#type = r#type as u8;
        self
    }
}

#[derive(Default, Serialize)]
pub enum DiscordMessageComponentTypes {
    /// Container for other components
    ActionRow = 1,
    /// Button object
    #[default]
    Button = 2,
    /// Select menu for picking from defined text options
    StringSelect = 3,
    /// Text input object
    TextInput = 4,
    /// Select menu for users
    UserSelect = 5,
    /// Select menu for roles
    RoleSelect = 6,
    /// Select menu for mentionables (users and roles)
    MentionableSelect = 7,
    /// Select menu for channels
    ChannelSelect = 8
}