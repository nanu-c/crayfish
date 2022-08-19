use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AvatarMessage {
    // pub uuid: String,
    #[serde(with = "serde_str")]
    pub avatar: String,
}


#[derive(Serialize)]
pub struct DecryptAvatarMessageResponse {
    // its base64 encoded libsignal_service::prelude::Content
    pub avatar: String,
}

pub async fn decrypt_avatar_message(
    data: AvatarMessage,
) -> Result<DecryptAvatarMessageResponse> {
    println!("decrypt_avatar_message");
    
    let avatar  = "yeai".to_string();
    Ok(DecryptAvatarMessageResponse {
        avatar,
    })
}