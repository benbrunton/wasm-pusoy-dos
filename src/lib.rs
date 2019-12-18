extern crate cfg_if;
extern crate wasm_bindgen;
extern crate pusoy_dos2;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use pusoy_dos2::game::{
    Game,
    Hand,
    Ruleset,
    FlushPrecedence,
    Player
};
use pusoy_dos2::cards::{PlayedCard, Card, Suit, Rank};
use pusoy_dos2::ai::get_move;

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
pub fn create_game(
    players: Box<[JsValue]>,
    decks: f64,
    jokers: f64,
    ruleset: &str
) -> Game {
    utils::set_panic_hook();

    let mut ids = vec!();
    for player in players.iter() {
        ids.push(JsValue::into_serde(player).unwrap());
    }

    let (suit_order, ruleset) = if ruleset == "pickering" {
        get_pickering_rules()
    } else {
        get_classic_rules()
    };
    Game::new(decks as u8, jokers as u8, &ids, suit_order, ruleset)
}

fn get_pickering_rules() -> ([Suit; 4], Ruleset) {
    ([
        Suit::Clubs,
        Suit::Hearts,
        Suit::Diamonds,
        Suit::Spades
    ],
    Ruleset {
        reversals_enabled: true,
        flush_precedence: FlushPrecedence::Rank
    })
}

fn get_classic_rules() -> ([Suit; 4], Ruleset) {
    ([
        Suit::Clubs,
        Suit::Spades,
        Suit::Hearts,
        Suit::Diamonds,
    ],
    Ruleset {
        reversals_enabled: true,
        flush_precedence: FlushPrecedence::Rank
    })

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

    convert_hand(hand)
}

#[wasm_bindgen]
pub fn submit_move(
    game: &mut Game,
    id: &str, 
    js_hand: &JsValue) -> JsValue {

    let cards: Vec<PlayedCard> = js_hand
        .into_serde().unwrap();

    let result = game.play_move(id, cards);

    match result {
        Ok(_) => JsValue::TRUE,
        Err(a) => JsValue::from_serde(&a).unwrap()
    }
}

#[wasm_bindgen]
pub fn get_last_move(game: &Game) -> JsValue {
    convert_hand(game.get_last_move())
}

#[wasm_bindgen]
pub fn get_next_player(game: &Game) -> JsValue {
     match game.get_next_player() {
        Some(h) => JsValue::from_serde(&h).unwrap(),
        None    => JsValue::NULL
    }
}

#[wasm_bindgen]
pub fn get_cpu_move(game: &Game) -> JsValue {
    let id = game.get_next_player().expect("no next player");
    suggest_move(game, &id)
}

#[wasm_bindgen]
pub fn suggest_move(game: &Game, id: &str) -> JsValue {
    let hand = game.suggest_move(id).unwrap();
    JsValue::from_serde(&hand).unwrap()
}

#[wasm_bindgen]
pub fn suggest_move_multiplayer(
    last_move: &JsValue,
    player_hand: &JsValue,
    suit_order: &JsValue,
    rank_order: &JsValue,
    ) -> JsValue {

    let last_move: Hand = last_move.into_serde().unwrap();
    let player_hand: Vec<Card> = player_hand.into_serde().unwrap();
    let player = Player::new("abc".to_string(), player_hand);
    let suits: [Suit; 4] = suit_order.into_serde().unwrap();
    let ranks: [Rank; 13] = rank_order.into_serde().unwrap();

    let suggested_move = get_move(Some(last_move), Some(player), suits, ranks).unwrap();
    JsValue::from_serde(&suggested_move).unwrap()
}

#[wasm_bindgen]
pub fn get_winners(game: &Game) -> JsValue {
    let winners = game.get_winners();
    JsValue::from_serde(&winners).unwrap()
}

#[wasm_bindgen]
pub fn check_move(game: &Game, js_hand: &JsValue) -> JsValue {
    let cards: Vec<PlayedCard> = js_hand
        .into_serde().unwrap();
    let result = game.check_move(cards);
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn check_move_multiplayer(
    last_move: &JsValue,
    player_hand: &JsValue,
    ruleset: &str,
    ranks: &JsValue,
    suits: &JsValue,
) -> JsValue {
    let hand: Vec<PlayedCard> = player_hand
        .into_serde().unwrap();
    let last_move_option: Option<Hand> = last_move.into_serde().unwrap();
    let flush_precedence = if ruleset == "pickering" {
        FlushPrecedence::Rank
    } else {
        FlushPrecedence::Suit
    };
    let suit_order: [Suit; 4] = suits.into_serde().unwrap();
    let rank_order: [Rank; 13] = ranks.into_serde().unwrap();

    let result = Game::check_move_m(
        hand,
        last_move_option,
        suit_order,
        rank_order,
        flush_precedence 
    );

    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_suit_order(game: &Game) -> JsValue {
    JsValue::from_serde(&game.get_suit_order()).unwrap() 
}

fn convert_hand(hand: Option<Hand>) -> JsValue {
    match hand {
        Some(h) => JsValue::from_serde(&h).unwrap(),
        None    => JsValue::NULL
    }
}
