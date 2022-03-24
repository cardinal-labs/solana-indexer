use borsh::BorshDeserialize;
use indexer_core::{
    db::{insert_into, models::TwitterHandle, tables::twitter_handle_name_services},
    prelude::*,
};

use super::Client;
use crate::prelude::*;

#[derive(BorshDeserialize, PartialEq, Debug, Clone)]
struct TwitterHandleAndRegistry {
    registry_key: [u8; 32],
    handle: String,
}

pub(crate) async fn process(
    client: &Client,
    key: Pubkey,
    wallet: Pubkey,
    data: Vec<u8>,
) -> Result<()> {
    let th = TwitterHandleAndRegistry::try_from_slice(data.as_slice())
        .context("failed to deserialize registry key and handle!")?;

    let row = TwitterHandle {
        address: Owned(bs58::encode(key).into_string()),
        wallet_address: Owned(bs58::encode(wallet).into_string()),
        twitter_handle: Owned(th.handle),
    };

    client
        .db()
        .run(move |db| {
            insert_into(twitter_handle_name_services::table)
                .values(&row)
                .on_conflict(twitter_handle_name_services::address)
                .do_update()
                .set(&row)
                .execute(db)
        })
        .await
        .context("Failed to insert twitter handle")?;

    Ok(())
}
