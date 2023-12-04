use crate::check::check;
use crate::fortnite::{Shop, aggregate};
use crate::popup::popup;

// FIXME: Fix the guessed capacity for the vecs; not sure how else i could get it

pub async fn raven() {
    let shop = Shop::new();
    let vec_daily = shop.get_daily().await.unwrap();
    let vec_upcoming = shop.get_upcoming().await.unwrap();

    // Daily
    let mut daily = Vec::with_capacity(300);
    for item in vec_daily {
        daily.append(&mut aggregate(item));
    }
    let daily_skin = check(daily).await;

    // Upcoming (future skins)
    let mut upcoming = Vec::with_capacity(200);
    for item in vec_upcoming {
        upcoming.append(&mut aggregate(item));
    }
    let upcoming_skin = check(upcoming).await;

    // Taking action
    let (word, tag, can_buy) = match (daily_skin.is_some(), upcoming_skin.is_some()) {
        (false, false) => { return }
        (false, true) => ("Upcoming", upcoming_skin.unwrap(), false),
        _ => ("Daily", daily_skin.unwrap(), true)
    };

    // Dialog message & all
    skin_found(
        format!("{word} skin!\n{tag:?}"),
        format!("New skin: {tag:?}"),
        can_buy
    ).await;
    println!("New skin found! {tag:?}");
}

pub async fn skin_found(title: String, message: String, can_buy: bool) {
    // Popup message
    for _ in 0..5 {
        popup(title.clone(), message.clone()).await;
    }

    // Fortnite (TODO: Not sure how to make this work)
    if can_buy {
        open::that("https://flooferland.vercel.app/uri/com.epicgames.launcher://apps/fn:4fe75bbc5a674f4f9b356b5c90567da5:Fortnite?action=launch").unwrap();
        open::that("https://flooferland.vercel.app/uri/com.epicgames.launcher://apps/fortnite?action=launch").unwrap();
        open::that("com.epicgames.launcher://apps/fn:4fe75bbc5a674f4f9b356b5c90567da5:Fortnite?action=launch").unwrap();
    }
}
