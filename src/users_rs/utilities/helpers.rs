use ic_cdk::api::call::call;
use ic_cdk::api::call::RejectionCode;

pub async fn generate_random_id() -> Result<u64, String> {
    let result: Result<(Vec<u8>,), (RejectionCode, String)> =
        call(candid::Principal::management_canister(), "raw_rand", ()).await;

    let random_id = match result {
        Ok((id,)) => id,
        Err((code, msg)) => {
            ic_cdk::println!("Error generating random ID: {:?}, message: {}", code, msg);
            return Err(format!("Failed to generate random ID: {}", msg));
        }
    };

    if random_id.len() < 8 {
        ic_cdk::println!("Not enough bytes to generate a contest ID");
        return Err("Not enough bytes to generate contest ID".to_string());
    }

    let random_id = u64::from_le_bytes(
        random_id[0..8]
            .try_into()
            .expect("slice with incorrect length"),
    );

    Ok(random_id)
}
