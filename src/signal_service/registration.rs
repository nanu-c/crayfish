use super::utils::service_with_number;
use crate::error::Result;

use libsignal_service::provisioning::{
    generate_registration_id, VerifyAccountResponse, ProvisioningManager,
    VerificationCodeResponse,
};
use libsignal_service::push_service::{
        AccountAttributes, DeviceCapabilities
};
use phonenumber::PhoneNumber;
use serde::Deserialize;
use serde_big_array::BigArray;

#[derive(Deserialize, Debug)]
pub struct Register {
    #[serde(with = "serde_str")]
    pub number: PhoneNumber,
    pub password: String,
    pub captcha: String,
    pub use_voice: bool,
}

#[derive(Deserialize, Debug)]
pub struct ConfirmRegistration {
    #[serde(with = "serde_str")]
    pub number: PhoneNumber,
    pub password: String,
    pub name: String,
    pub confirm_code: u32,
    #[serde(with = "BigArray")]
    pub signaling_key: [u8; 52],
}

pub async fn register_user(data: Register) -> Result<VerificationCodeResponse> {
    let mut push_service = service_with_number(data.number.clone(), data.password.clone());
    let mut provisioning_manager =
        ProvisioningManager::new(&mut push_service, data.number, data.password);

    Ok(if data.use_voice {
        provisioning_manager
            .request_voice_verification_code(Some(&data.captcha), None)
            .await?
    } else {
        provisioning_manager
            .request_sms_verification_code(Some(&data.captcha), None)
            .await?
    })
}

pub async fn verify_user(data: ConfirmRegistration) -> Result<VerifyAccountResponse> {
    let registration_id = generate_registration_id(&mut rand::thread_rng());
    let mut push_service = service_with_number(data.number.clone(), data.password.clone());
    let mut provisioning_manager =
        ProvisioningManager::new(&mut push_service, data.number, data.password);
        let account_attrs = AccountAttributes {
            signaling_key:  Some(data.signaling_key.to_vec()),
            registration_id,
            voice: false,
            video: false,
            fetches_messages: true,
            pin: None,
            name: data.name,
            registration_lock: None,
            unidentified_access_key: None,
            unrestricted_unidentified_access: false,
            discoverable_by_phone_number: true,
            capabilities: DeviceCapabilities {
                announcement_group: true,
                gv2: true,
                storage: false,
                gv1_migration: true,
                sender_key: true,
                change_number: false,
                gift_badges: false,
                stories: false,
            },
        };

    Ok(provisioning_manager
        .confirm_verification_code(
            data.confirm_code,
            account_attrs,

        )
        .await?)
}
