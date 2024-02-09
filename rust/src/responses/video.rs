use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Video {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_coun: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i32>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Video {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Video {:?}", self.extra);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum VideoField {
    Id,
    CreateTime,
    CoverImageUrl,
    ShareUrl,
    VideoDescription,
    Duration,
    Height,
    Width,
    Title,
    EmbedHtml,
    EmbedLink,
    LikeCount,
    CommentCoun,
    ShareCount,
    ViewCount,
}
impl std::fmt::Display for VideoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreateTime => write!(f, "create_time"),
            Self::CoverImageUrl => write!(f, "cover_image_url"),
            Self::ShareUrl => write!(f, "share_url"),
            Self::VideoDescription => write!(f, "video_description"),
            Self::Duration => write!(f, "duration"),
            Self::Height => write!(f, "height"),
            Self::Width => write!(f, "width"),
            Self::Title => write!(f, "title"),
            Self::EmbedHtml => write!(f, "embed_html"),
            Self::EmbedLink => write!(f, "embed_link"),
            Self::LikeCount => write!(f, "like_count"),
            Self::CommentCoun => write!(f, "comment_coun"),
            Self::ShareCount => write!(f, "share_count"),
            Self::ViewCount => write!(f, "view_count"),
        }
    }
}
