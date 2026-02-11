// Example: Simple Card Game Implementation
// File: src/games/card_game.rs (optional extension)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: u32,
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardGameState {
    pub deck: Vec<Card>,
    pub player_hands: std::collections::HashMap<String, Vec<Card>>,
    pub table_cards: Vec<Card>,
    pub current_player: Option<String>,
    pub game_phase: GamePhase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamePhase {
    Waiting,  // Menunggu pemain
    Starting, // Persiapan game
    Playing,  // Game berlangsung
    Finished, // Game selesai
}

impl CardGameState {
    pub fn new() -> Self {
        Self {
            deck: Self::create_deck(),
            player_hands: std::collections::HashMap::new(),
            table_cards: Vec::new(),
            current_player: None,
            game_phase: GamePhase::Waiting,
        }
    }

    fn create_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        let mut id = 0;

        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                deck.push(Card {
                    id,
                    suit: suit.clone(),
                    rank: rank.clone(),
                });
                id += 1;
            }
        }

        deck
    }

    pub fn shuffle_deck(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        self.deck.shuffle(&mut thread_rng());
    }

    pub fn deal_cards(&mut self, player_id: &str, count: usize) -> Vec<Card> {
        let cards: Vec<Card> = self.deck.drain(..count.min(self.deck.len())).collect();
        self.player_hands
            .entry(player_id.to_string())
            .or_insert_with(Vec::new)
            .extend(cards.clone());
        cards
    }

    pub fn play_card(&mut self, player_id: &str, card_id: u32) -> Result<Card, String> {
        let hand = self
            .player_hands
            .get_mut(player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        let card_pos = hand
            .iter()
            .position(|c| c.id == card_id)
            .ok_or_else(|| "Card not in hand".to_string())?;

        let card = hand.remove(card_pos);
        self.table_cards.push(card.clone());

        Ok(card)
    }

    pub fn next_turn(&mut self, players: &[String]) {
        if players.is_empty() {
            self.current_player = None;
            return;
        }

        if let Some(current) = &self.current_player {
            if let Some(pos) = players.iter().position(|p| p == current) {
                let next_pos = (pos + 1) % players.len();
                self.current_player = Some(players[next_pos].clone());
            } else {
                self.current_player = Some(players[0].clone());
            }
        } else {
            self.current_player = Some(players[0].clone());
        }
    }
}

// Example game actions yang bisa dipake:
//
// Start Game:
// {
//   "type": "game_action",
//   "action": "start_game",
//   "data": {}
// }
//
// Play Card:
// {
//   "type": "game_action",
//   "action": "play_card",
//   "data": {
//     "card_id": 25
//   }
// }
//
// Draw Card:
// {
//   "type": "game_action",
//   "action": "draw_card",
//   "data": {
//     "count": 1
//   }
// }
