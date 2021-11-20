use crate::config::SignalConfig;
use crate::error::Result;
use crate::store::Storage;
use crate::store::StorageLocation;
extern crate base64;

use libsignal_service::ServiceAddress;
use libsignal_service::cipher::ServiceCipher;
use libsignal_service::configuration::ServiceConfiguration;
use libsignal_service::configuration::SignalServers;
use libsignal_service::prelude::Envelope;

use prost::Message;
// use protocol::envelope::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SealedSenderMessage {
    // pub uuid: String,
    #[serde(with = "serde_str")]
    pub message: String,
}

#[derive(Serialize)]
pub struct DecryptSealedMessageResponse {
    // its base64 encoded libsignal_service::prelude::Content
    pub message: String,
    pub sender: ServiceAddress,
    pub sender_device: u32,
    pub timestamp: u64,
    pub needs_receipt: bool,
}
fn service_cfg() -> ServiceConfiguration {
    // XXX: read the configuration files!
    SignalServers::Production.into()
}
pub async fn decrypt_sealed_message(
    data: SealedSenderMessage,
) -> Result<DecryptSealedMessageResponse> {
    println!("decrypt_sealed_message");

    let service_cfg = service_cfg();
    let config = SignalConfig::default();
    let storage = open_storage(&config).await?;
    let signaling_key = storage.signaling_key().await?;
    println!("signaling_key loaded");
    let mut cipher = ServiceCipher::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        storage.clone(),
        rand::thread_rng(),
        service_cfg.credentials_validator().expect("trust root"),
    );
    println!("cipher created");
    // let msg = data.message.into_bytes();
    let msg = base64::decode(data.message).unwrap();
    let envelope = Envelope::decrypt(&msg, &signaling_key, false)?;
    let content = cipher.open_envelope(envelope).await?.unwrap();
    println!("sealed message content decrypted");
    let content_vec = content.body.into_proto().encode_to_vec();
    let message = base64::encode(&content_vec);
    Ok(DecryptSealedMessageResponse {
        message: message,
        sender_device: content.metadata.sender_device,
        timestamp: content.metadata.timestamp,
        needs_receipt: content.metadata.needs_receipt,
        sender: content.metadata.sender,
    })
}

async fn open_storage(config: &crate::config::SignalConfig) -> anyhow::Result<Storage> {
    let home = dirs::home_dir()
        .unwrap()
        .join(".local")
        .join("share")
        .join("textsecure.nanuc");
    let location = StorageLocation::Path(home);
    println!("opening storage at location {:?}", location.to_str());

    let storage = Storage::open(&location).await?;
    println!("opening storage finished {:?}", location.to_str());
    Ok(storage)
}
