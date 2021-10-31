use std::ops::Not;

use crate::apple_responses::Root;

mod apple_responses;

const APPLE_FETCH_REQUEST_URL: &str = "https://www.apple.com/shop/pickup-message-recommendations?cppart=UNLOCKED/US&location=94010&product=MLKP3LL/A";

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(APPLE_FETCH_REQUEST_URL)
        .await?
        .json::<Root>()
        .await?;


    let mut found = false;

    for store in resp.body.pickup_message.stores {
        println!("Checking {} (distance: {})", store.store_name, store.store_distance_with_unit);
        if store.parts_availability.is_empty().not() {
            for (_, value) in store.parts_availability {
                if let Some(product_title) = value
                    .get("storePickupProductTitle")
                    .and_then(|x| x.as_str())
                {
                    println!("{} has {}", store.store_name, product_title);
                    found = true;
                }
            }
        }
    }

    if !found {
        panic!("no store has the iPhone available.");
    }

    Ok(())
}
