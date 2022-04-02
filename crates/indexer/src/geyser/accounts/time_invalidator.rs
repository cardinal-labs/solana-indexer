use cardinal_time_invalidator::state::TimeInvalidator as TimeInvalidatorAccount;
use indexer_core::{
    db::{insert_into, models::TimeInvalidator, tables::time_invalidators},
    prelude::*,
};

use super::Client;
use crate::prelude::*;

pub(crate) async fn process(
    client: &Client,
    key: Pubkey,
    time_invalidator: TimeInvalidatorAccount,
) -> Result<()> {
    let row = TimeInvalidator {
        address: Owned(bs58::encode(key).into_string()),
        bump: time_invalidator.bump.try_into()?,
        token_manager_address: Owned(bs58::encode(time_invalidator.token_manager).into_string()),
        expiration: time_invalidator
            .expiration
            .map(|e| NaiveDateTime::from_timestamp(e, 0)),
        duration_seconds: time_invalidator.duration_seconds.try_into()?,
        extension_payment_amount: time_invalidator
            .extension_payment_amount
            .map(TryFrom::try_from)
            .transpose()?,
        extension_payment_mint: time_invalidator
            .extension_payment_mint
            .map(|m| Owned(bs58::encode(m).into_string())),
        extension_duration_seconds: time_invalidator
            .extension_duration_seconds
            .map(TryFrom::try_from)
            .transpose()?,
        max_expiration: time_invalidator
            .max_expiration
            .map(|e| NaiveDateTime::from_timestamp(e, 0)),
        disable_partial_extension: time_invalidator
            .disable_partial_extension
            .map(TryFrom::try_from)
            .transpose()?,
    };
    client
        .db()
        .run(move |db| {
            insert_into(time_invalidators::table)
                .values(&row)
                .on_conflict(time_invalidators::address)
                .do_update()
                .set(&row)
                .execute(db)
        })
        .await
        .context("Failed to insert TimeInvalidator")?;

    Ok(())
}
