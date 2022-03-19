use crate::error::Result;
use libsignal_service::attachment_cipher::{decrypt_in_place, AttachmentCipherError};

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;


#[derive(Deserialize, Debug)]
pub struct AvatarMessage {
    // pub uuid: String,
    #[serde(with = "serde_str")]
    pub avatar: String,
    #[serde(with = "BigArray")]
    pub key: [u8; 64],
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
    let mut ciphertext = base64::decode(data.avatar).unwrap();

    let result  = decrypt_in_place(data.key,  &mut ciphertext);
    match result {
        Ok(res)=>{
            Ok(DecryptAvatarMessageResponse {
                avatar: base64::encode(&ciphertext),
            })
        },
        Err(e)=> {
            eprint!("error decrypting avatar {}",e );
            Ok(DecryptAvatarMessageResponse {
                avatar: "".to_string(),
            })
        },    
    }

}
