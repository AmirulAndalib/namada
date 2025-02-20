//! A tx for a jailed validator to unjail themselves and re-enter the
//! validator sets.

use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, tx_data: BatchedTx) -> TxResult {
    let data = ctx.get_tx_data(&tx_data)?;
    let validator = Address::try_from_slice(&data[..])
        .wrap_err("Failed to decode the address of the validator to unjail")?;
    ctx.unjail_validator(&validator)
        .wrap_err("Failed to unjail validator")
}
