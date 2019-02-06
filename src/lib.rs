extern crate cfg_if;
extern crate wasm_bindgen;
extern crate pusoy_dos2;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use pusoy_dos2::game::{Game, Hand};
use pusoy_dos2::cards::{Suit, PlayedCard};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn create_game(players: Box<[JsValue]>) -> Game {
    utils::set_panic_hook();
    let suit_order = [
        Suit::Clubs,
        Suit::Hearts,
        Suit::Diamonds,
        Suit::Spades
    ];

    let mut ids = vec!();
    for player in players.iter() {
        ids.push(JsValue::into_serde(player).unwrap());
    }

    let decks = 1;
    let jokers = 0;
    let reversals = false;

    Game::new(decks, jokers, &ids, suit_order, reversals)
}

#[wasm_bindgen]
pub fn get_player(game: &Game, id: &str) -> JsValue {
    let player = game.get_player(id);
    match player {
        Some(p) => JsValue::from_serde(&p.get_hand()).unwrap(),
        None    => JsValue::NULL
    }
}


#[wasm_bindgen]
pub fn get_hand_type(js_hand: &JsValue) -> JsValue {

    let cards: Vec<PlayedCard> = js_hand
        .into_serde().unwrap();

    let hand = Hand::build(cards);

    match hand {
        Some(h) => JsValue::from_serde(&h).unwrap(),
        None    => JsValue::NULL
    }
    
}
