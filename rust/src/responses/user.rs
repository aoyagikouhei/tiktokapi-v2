use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    pub username: Option<String>,
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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
    Username,
    FollowerCount,
    FollowingCount,
    LikesCount,
    VideoCount,
}

impl UserField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(UserField::OpenId);
        set.insert(UserField::UnionId);
        set.insert(UserField::AvatarUrl);
        set.insert(UserField::AvatarUrl100);
        set.insert(UserField::AvatarLargeUrl);
        set.insert(UserField::DisplayName);
        set.insert(UserField::BioDescription);
        set.insert(UserField::ProfileDeepLink);
        set.insert(UserField::IsVerified);
        set.insert(UserField::Username);
        set.insert(UserField::FollowerCount);
        set.insert(UserField::FollowingCount);
        set.insert(UserField::LikesCount);
        set.insert(UserField::VideoCount);
        set
    }
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
            Self::Username => write!(f, "username"),
            Self::FollowerCount => write!(f, "follower_count"),
            Self::FollowingCount => write!(f, "following_count"),
            Self::LikesCount => write!(f, "likes_count"),
            Self::VideoCount => write!(f, "video_count"),
        }
    }
}
