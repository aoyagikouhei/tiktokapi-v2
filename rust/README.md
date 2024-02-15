# tiktokapi-v2

TikTok API v2 library.

[Documentation](https://docs.rs/tiktokapi-v2)

- Request builder
- Convenience setted parameter methods
- OAuth2
- type support
- OAuth web example

## Supported APIs
- get_v2_user_info
- post_v2_video_list
- post_v2_video_query

## Features
### default
- reqwest/default-tls

### rustls-tls
- reqwest/rustls-tls

## Changes
[CHANGELOG.md](https://github.com/aoyagikouhei/tiktokapi-v2/blob/main/CHANGELOG.md)

## Examples

### API
```rust
use tiktokapi_v2::{
    apis::get_v2_user_info::Api,
    responses::user::UserField,
};
let access_token = "xxx";
let api = Api::new(UserField::all());
let res = api.execute(access_token).await.unwrap();
println!("{:?}", res);
```