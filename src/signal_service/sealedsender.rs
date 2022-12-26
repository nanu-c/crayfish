use crate::config::SignalConfig;
use crate::error::Result;
use crate::store::Storage;
use crate::store::StorageLocation;
use libsignal_service::prelude::ProtobufMessage;
extern crate base64;

use libsignal_service::cipher::ServiceCipher;
use libsignal_service::configuration::ServiceConfiguration;
use libsignal_service::configuration::SignalServers;
use libsignal_service::prelude::Envelope;
use libsignal_service::ServiceAddress;
use libsignal_service::prelude::Uuid;

// use protocol::envelope::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SealedSenderMessage {
    pub uuid: Option<Uuid>,
    pub local_device_id: u32,
    pub local_uuid: Option<Uuid>,
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
    let uuid = data.local_uuid;
    let device_id = data.local_device_id;
    println!("signaling_key loaded");
    let mut cipher = ServiceCipher::new(
        storage.clone(),
        storage.clone(),
        storage.clone(),
        storage,
        rand::thread_rng(),
        service_cfg.unidentified_sender_trust_root,
        uuid.expect("local uuid to initialize service cipher"),
        device_id,
    );
    println!("cipher created");
    // let msg = data.message.into_bytes();
    let msg = base64::decode(data.message).unwrap();
    let envelope = Envelope::decrypt(&msg, &signaling_key, false)?;
    let content = cipher.open_envelope(envelope).await?;
    match content {
        Some(unwraped_content) =>{
            println!("sealed message content decrypted");
            let content_vec =unwraped_content.body.into_proto().encode_to_vec();
            let message = base64::encode(&content_vec);
            Ok(DecryptSealedMessageResponse {
                message,
                sender_device: unwraped_content.metadata.sender_device,
                timestamp: unwraped_content.metadata.timestamp, // todo server timestamp from envelope
                needs_receipt: unwraped_content.metadata.needs_receipt,
                sender: unwraped_content.metadata.sender,
            })  
        },
        None => {
            println!("sealed message content not decrypted");
            Ok(DecryptSealedMessageResponse {
                message: "".to_string(),
                sender_device: 0,
                timestamp: 0,
                needs_receipt: false,
                sender: ServiceAddress{
                    uuid: None,
                    phonenumber: None,
                    relay: None,

                },
            })  
        }
    }

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
