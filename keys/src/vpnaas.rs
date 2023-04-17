pub mod proto {
    tonic::include_proto!("vpnaas");
}

pub use proto::keys_client::KeysClient;
pub use proto::keys_server::KeysServer;

use proto::{User, UserPubkey};
use tonic::Status;

impl TryFrom<User> for String {
    type Error = Status;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        if user.username.is_empty() {
            return Err(Status::invalid_argument("Username is empty"));
        }

        Ok(user.username)
    }
}

impl TryFrom<UserPubkey> for (String, [u8; 32]) {
    type Error = Status;

    fn try_from(obj: UserPubkey) -> Result<Self, Self::Error> {
        let UserPubkey { user, pubkey } = obj;

        let username = user
            .ok_or(Status::invalid_argument("User not specified"))?
            .try_into()?;

        let pubkey = pubkey
            .ok_or(Status::invalid_argument("Public key not specified"))
            .and_then(|p| {
                p.bytes
                    .try_into()
                    .map_err(|_| Status::invalid_argument("Invalid public key length"))
            })?;

        Ok((username, pubkey))
    }
}
