use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url_100: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_large_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_deep_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_count: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl User {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("User {:?}", self.extra);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum UserField {
    OpenId,
    UnionId,
    AvatarUrl,
    AvatarUrl100,
    AvatarLargeUrl,
    DisplayName,
    BioDescription,
    ProfileDeepLink,
    IsVerified,
    FollowerCount,
    FollowingCount,
    LikesCount,
    VideoCount,
}
impl std::fmt::Display for UserField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OpenId => write!(f, "open_id"),
            Self::UnionId => write!(f, "union_id"),
            Self::AvatarUrl => write!(f, "avatar_url"),
            Self::AvatarUrl100 => write!(f, "avatar_url_100"),
            Self::AvatarLargeUrl => write!(f, "avatar_large_url"),
            Self::DisplayName => write!(f, "display_name"),
            Self::BioDescription => write!(f, "bio_description"),
            Self::ProfileDeepLink => write!(f, "profile_deep_link"),
            Self::IsVerified => write!(f, "is_verified"),
            Self::FollowerCount => write!(f, "follower_count"),
            Self::FollowingCount => write!(f, "following_count"),
            Self::LikesCount => write!(f, "likes_count"),
            Self::VideoCount => write!(f, "video_count"),
        }
    }
}
