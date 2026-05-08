//! Client-to-Client protocol to establish an out-of-band connection using multiple round trips.
//! 
//! It is bound to an [`AppID`]. Only applications using that APPID (and thus this protocol) can interoperate with
//! other compliant implementations.
//!
//! At its core, "peer messages" are exchanged over an established wormhole connection with the other side.
//! - an "offer" message describing how to connect to the initiator, with an
//!   optional "refresh key" to use to obtain a fresh wormhole without human intervention
//!   if the current one has already been closed
//! - an "answer" message indicating receipt and acceptance of the offer
//! - optional "trickle" messages in both directions until the connection is established

use crate::{AppConfig, AppID};
use std::{borrow::Cow};
use serde::{Deserialize, Serialize};

const APPID_RAW: &str = "stunnels.io/wormhole/trickle";

/// The App ID associated with this protocol.
pub const APPID: AppID = AppID(Cow::Borrowed(APPID_RAW));

/// An [`crate::AppConfig`] with default parameters for the Offer/Answer/trickle protocol.
///
/// You **must not** change `id` and `rendezvous_url` to be interoperable.
/// The `app_version` can be adjusted if you want to disable some features.
pub const APP_CONFIG: AppConfig<AppVersion> = AppConfig::<AppVersion> {
    id: AppID(Cow::Borrowed(APPID_RAW)),
    rendezvous_url: Cow::Borrowed(crate::rendezvous::DEFAULT_RENDEZVOUS_SERVER),
    app_version: AppVersion::new(),
};

/**
 * The application specific version information for this protocol.
 */
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppVersion {}

// TODO check invariants during deserialization
impl AppVersion {
    const fn new() -> Self {
        Self {}
    }
}

impl Default for AppVersion {
    fn default() -> Self {
        Self::new()
    }
}
