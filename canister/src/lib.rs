mod state;

use ic_cdk::{init, post_upgrade, pre_upgrade};
use ic_certified_assets::types::AssetCanisterArgs;

use crate::state::{
    is_candid_stable_state, load_candid_stable_state, load_stable_state, save_stable_state,
};

/// Orchestrators (e.g. a Casals baton) may install or upgrade with no argument
/// bytes at all, which Candid cannot parse; that means "no argument".
fn decode_optional_args(arg_bytes: Vec<u8>) -> Option<AssetCanisterArgs> {
    if arg_bytes.is_empty() {
        return None;
    }
    candid::decode_one(&arg_bytes).expect("failed to decode the canister argument")
}

#[init(decode_with = "decode_optional_args")]
fn init(args: Option<AssetCanisterArgs>) {
    ic_certified_assets::init(args);
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable_state = ic_certified_assets::pre_upgrade();
    save_stable_state(&stable_state).expect("failed to serialize stable state");
}

#[post_upgrade(decode_with = "decode_optional_args")]
fn post_upgrade(args: Option<AssetCanisterArgs>) {
    let stable_state = if is_candid_stable_state() {
        // backward compatibility
        load_candid_stable_state()
            .expect("failed to restore candid stable state")
            .into()
    } else {
        load_stable_state().expect("failed to deserialize stable state")
    };
    ic_certified_assets::post_upgrade(stable_state, args);
}

ic_certified_assets::export_canister_methods!();

#[cfg(test)]
mod tests {
    use super::*;
    use ic_certified_assets::types::{AssetCanisterArgs, UpgradeArgs};

    #[test]
    fn no_argument_bytes_mean_no_argument() {
        assert!(decode_optional_args(vec![]).is_none());
        assert!(decode_optional_args(candid::encode_args(()).unwrap()).is_none());
        let none: Option<AssetCanisterArgs> = None;
        assert!(decode_optional_args(candid::encode_one(none).unwrap()).is_none());
        let upgrade = Some(AssetCanisterArgs::Upgrade(UpgradeArgs {
            set_permissions: None,
        }));
        assert!(matches!(
            decode_optional_args(candid::encode_one(upgrade).unwrap()),
            Some(AssetCanisterArgs::Upgrade(_))
        ));
    }
}
