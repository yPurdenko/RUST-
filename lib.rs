///////////////////////////////////////////////////mod card///////////////////////////////////////////////////////////////
pub mod card{
    use std::fmt;

    pub struct Card {
        pub rank: String,
        pub suit: String,
        pub is_face_up: bool,
        pub value: u8,
    }

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            if self.is_face_up {
                write!(f, "{}{}", self.rank, self.suit)
            } else {
                write!(f, "X")
            }
        }
    }

    impl Card {
        pub fn new(rank: String, suit: String, value: u8) -> Card {
            let is_face_up = true;
            Card {
                rank,
                suit,
                is_face_up,
                value,
            }
        }
        pub fn flip(&mut self) {
            self.is_face_up = !self.is_face_up;
        }
        // return card value if card is face up else return 0
        pub fn value(&self) -> u8 {
            if self.is_face_up {
                return self.value;
            } else {
                return 0;
            }
        }
    }

    #[cfg(test)]
    mod card_tests {
        use crate::card;
        #[test]
        fn init_test() {
            assert_eq!(format!("{}",card::Card::new("A".to_string(),"h".to_string(),1)), "Ah".to_string());
            assert_eq!(format!("{}",card::Card::new("A".to_string(),"s".to_string(),1)), "As".to_string());
            assert_eq!(format!("{}",card::Card::new("10".to_string(),"h".to_string(),1)), "10h".to_string());
        }
        #[test]
        fn test_cards_value() {
            let  card = card::Card::new("A".to_string(),"h".to_string(),1);
            assert_eq!(card.value(), 1);
            let  card = card::Card::new("A".to_string(),"h".to_string(),10);
            assert_eq!(card.value(), 10);
            let  card = card::Card::new("A".to_string(),"h".to_string(),2);
            assert_eq!(card.value(), 2);
        }
        #[test]
        fn test_cards_flip() {
            let mut card = card::Card::new("A".to_string(),"h".to_string(),1);
            assert_eq!(card.value(), 1);
            card.flip();
            assert_eq!(card.value(), 0);
            let mut card = card::Card::new("A".to_string(),"h".to_string(),10);
            card.flip();
            assert_eq!(card.value(), 0);
        }

        #[test]
        fn test_cards_double_flip() {
            let mut card = card::Card::new("A".to_string(),"h".to_string(),1);
            assert_eq!(card.value(), 1);
            card.flip();
            card.flip();
            assert_eq!(card.value(), 1);
            let mut card = card::Card::new("A".to_string(),"h".to_string(),10);
            card.flip();
            card.flip();
            assert_eq!(card.value(), 10);
        } 
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////mod deck_of_cards///////////////////////////////////////////////////////
pub mod deck_of_cards{

use crate::card;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

    pub struct DeckOfCards {
        pub cards: Vec<card::Card>,
    }

    impl DeckOfCards {
        pub fn new() -> DeckOfCards {
            let ranks = vec![
                "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
            ];
            let suits = vec!["c", "d", "h", "s"];
            let mut cards: Vec<card::Card> = vec![];
            let mut value = 0;
            for rank in &ranks {
                value += 1;
                if value > 10 {
                    value = 10;
                }
                for suit in &suits {
                    cards.push(card::Card::new(rank.to_string(), suit.to_string(), value));
                }
            }
            DeckOfCards { cards }
        }
        pub fn refresh(&mut self) {
            self.cards = vec![];
            let ranks = vec![
                "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
            ];
            let suits = vec!["c", "d", "h", "s"];
            let mut value = 0;
            for rank in &ranks {
                value += 1;
                if value > 10 {
                    value = 10;
                }
                for suit in &suits {
                    self.cards
                        .push(card::Card::new(rank.to_string(), suit.to_string(), value));
                }
            }
        }
        pub fn shuffel(&mut self) {
            let mut rng = thread_rng();
            self.cards.shuffle(&mut rng);
        }
    }


    impl fmt::Display for DeckOfCards {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            let mut res: Vec<String> = vec![];
            for i in &self.cards {
                res.push(format!("{}", i));
            }
            write!(f, "{:?}", res)
        }
    }
    #[cfg(test)]
    mod deck_of_cards_tests {
        use crate::deck_of_cards;
        #[test]
        fn init_test() {
            let deck = deck_of_cards::DeckOfCards::new();
            assert_eq!(deck.cards.len(), 52);
        }

        #[test]
        fn shuffel_test() {
            let mut deck = deck_of_cards::DeckOfCards::new();
            deck.shuffel();
            assert_eq!(deck.cards.len(), 52);
        }

        #[test]
        fn refresh_test() {
            let mut deck = deck_of_cards::DeckOfCards::new();
            let _t = deck.cards.pop();
            deck.refresh();
            assert_eq!(deck.cards.len(), 52);
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////mod player//////////////////////////////////////////////////////////////////
pub mod player{    
    use crate::card;
    use std::fmt;
    use std::io::{stdin, stdout, Write};

    pub struct Player {
        pub cards: Vec<card::Card>,
        pub name: String,
        pub total: u8,
    }
    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut cards: Vec<String> = vec![];
            for i in &self.cards {
                cards.push(format!("{}", i));
            }
            write!(f, "{}: {:?}", self.name, cards)
        }
    }

    impl Player {
        pub fn new(name: String) -> Player {
            let cards = vec![];
            let total = 0;
            Player { cards, name, total }
        }

        pub fn score(&mut self) {
            let mut res = 0;
            let mut count_of_as: u8 = 0;
            for i in &self.cards {
                if i.is_face_up && i.rank == "A".to_string() {
                    count_of_as += 1;
                }
                res += i.value();
            }
            while count_of_as > 0 && res <= 11 {
                res += 10;
                count_of_as -= 1;
            }
            self.total = res;
        }

        pub fn clear(&mut self) {
            self.cards = vec![];
        }

        pub fn add(&mut self, card: card::Card) {
            self.cards.push(card);
            self.score();
        }

        pub fn is_busted(&self) -> bool {
            self.total > 21
        }
        pub fn is_hitting(&self) -> bool {
            println!("{} , do you want take a card? [Y/N]: ", self.name);
            let mut s = String::new();
            let _ = stdout().flush();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            s == "y".to_string()
        }

        fn lose(&self) {
            println!("{}, lose. score : {}", self.name, self.total);
        }

        pub fn win(&self) {
            println!("{}, win. score : {}", self.name, self.total);
        }

        pub fn push(&self) {
            println!("{}, remisuje. score : {}", self.name, self.total);
        }

        pub fn bust(&mut self) {
            println!("{} , i'm over", self.name);
            self.lose();
        }
    }

    #[cfg(test)]
    mod player_tests{
        use crate::player;
        use crate::card;
        #[test]
        fn init_test() {
            let  player = player::Player::new("Player1".to_string());
            assert_eq!(format!("{}",player), "Player1: []".to_string());
        }
        #[test]
        fn cards_test() {
            let mut player = player::Player::new("Player1".to_string());
            player.add(card::Card::new("A".to_string(), "h".to_string(), 1));
            assert_eq!(format!("{}",player), "Player1: [\"Ah\"]".to_string());
            player.add(card::Card::new("A".to_string(), "s".to_string(), 1));
            assert_eq!(format!("{}",player), "Player1: [\"Ah\", \"As\"]".to_string());
            player.clear();
            assert_eq!(format!("{}",player), "Player1: []".to_string());
        }

        #[test]
        fn score_test() {
            let mut player = player::Player::new("Player1".to_string());
            player.add(card::Card::new("A".to_string(), "h".to_string(), 1));
            player.add(card::Card::new("A".to_string(), "s".to_string(), 1));
            player.score();
            assert_eq!(player.total, 12);
        }
        
    }

}
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////mod dealer/////////////////////////////////////////////////////////////////
pub mod dealer{
    use crate::card;
    use crate::deck_of_cards;
    use crate::player;
    use std::fmt;

    pub struct Dealer {
        pub cards: Vec<card::Card>,
        pub total: u8,
        pub deck_of_cards: deck_of_cards::DeckOfCards,
    }

    impl fmt::Display for Dealer {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut cards: Vec<String> = vec![];
            for i in &self.cards {
                cards.push(format!("{}", i));
            }
            write!(f, "Dealer: {:?}", cards)
        }
    }

    impl Dealer {
        pub fn new() -> Dealer {
            let cards: Vec<card::Card> = vec![];
            let total = 0;
            let deck_of_cards = deck_of_cards::DeckOfCards::new();
            Dealer {
                cards,
                total,
                deck_of_cards,
            }
        }

        pub fn flip_first_card(&mut self) {
            if !self.cards.is_empty() {
                self.cards[0].flip();
                self.score();
            } else {
                println!("dealer have not any cards");
            }
        }
        pub fn is_hitting(&mut self) -> bool {
            self.score();
            self.total < 17
        }
        pub fn shuffel(&mut self) {
            self.deck_of_cards.shuffel();
        }
        pub fn give(&mut self, player: &mut player::Player) {
            if !self.deck_of_cards.cards.is_empty() {
                let c = self.deck_of_cards.cards.pop().unwrap();
                player.add(c);
            } else {
                println!("deck of cards is empty");
            }
        }
        pub fn add(&mut self) {
            if !self.deck_of_cards.cards.is_empty() {
                let c = self.deck_of_cards.cards.pop().unwrap();
                self.cards.push(c);
                self.score();
            } else {
                println!("deck of cards is empty");
            }
        }
        pub fn score(&mut self) {
            let mut res = 0;
            let mut count_of_as: u8 = 0;
            for i in &self.cards {
                if i.is_face_up && i.rank == "A".to_string() {
                    count_of_as += 1;
                }
                res += i.value();
            }
            while count_of_as > 0 && res < 11 {
                res += 10;
                count_of_as -= 1;
            }
            self.total = res;
        }

        pub fn lose(&self) {
            println!("Dealer, lose.  score: {} ", self.total);
        }

        pub fn win(&self) {
            println!("Dealer, win. score : {}", self.total);
        }

        pub fn push(&self) {
            println!("Dealer, remisuje. score: {}", self.total);
        }

        pub fn bust(&mut self) {
            println!("Dealer , i'm over");
            self.lose();
        }
        pub fn is_busted(&self) -> bool {
            self.total > 21
        }
    }    
    #[cfg(test)]
    mod dealer_tests{
        use crate::dealer;
        #[test]
        fn init_test() {
            let  dealer = dealer::Dealer::new();
            assert_eq!(format!("{}",dealer), "Dealer: []".to_string());
        }
        #[test]
        fn cards_test() {
            let mut dealer = dealer::Dealer::new();
            dealer.add();
            assert_eq!(format!("{}",dealer), "Dealer: [\"Ks\"]".to_string());
            dealer.add();
            assert_eq!(format!("{}",dealer), "Dealer: [\"Ks\", \"Kh\"]".to_string());
            dealer.flip_first_card();
            assert_eq!(format!("{}",dealer), "Dealer: [\"X\", \"Kh\"]".to_string());               
        }

        #[test]
        fn score_test() {
            let mut dealer = dealer::Dealer::new();
            dealer.add();
            assert_eq!(dealer.total, 10);
            dealer.add();
            assert_eq!(dealer.total, 20);
            dealer.flip_first_card();
            assert_eq!(dealer.total, 10);
        }    
    }
}
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////mod blackjack_game//////////////////////////////////////////////////////////
pub mod blackjack_game{
use crate::player; 
use crate::dealer;

pub struct BlackJackGame {
    pub players: Vec<player::Player>,
    pub dealer: dealer::Dealer,
}

impl BlackJackGame {
    pub fn new(names: Vec<String>) -> BlackJackGame {
        let mut players: Vec<player::Player> = vec![];
        for name in &names {
            players.push(player::Player::new(name.to_string()));
        }
        let mut dealer = dealer::Dealer::new();
        dealer.shuffel();
        BlackJackGame { players, dealer }
    }

    fn still_playing(&mut self) {
        let len = self.players.len();
        for player in 0..len {
            let i = len - self.players.len();
            let i = player - i;
            if self.players[i].is_busted() {
                self.players.remove(i);
            }
        }
    }
    fn additional_cards(&mut self) {
        for player in &mut self.players {
            while !player.is_busted() && player.is_hitting() {
                self.dealer.give(player);
                println!("{}", player);
                if player.is_busted() {
                    player.bust();
                }
            }
        }
    }

    fn start(&mut self){
        println!("************blackjack************");
        println!("name:        cards:     ")
    }

    pub fn play(&mut self) {
        self.start();
        for player in &mut self.players {
            self.dealer.give(player);
            self.dealer.give(player);
        }

        self.dealer.add();
        self.dealer.add();
        self.dealer.flip_first_card();
        self.dealer.score();

        for player in &self.players {
            println!("{}", player);
        }
        println!("{}", self.dealer);

        self.additional_cards();
        self.still_playing();
        self.dealer.flip_first_card();

        if self.players.is_empty() {
            self.dealer.win()
        } else {
            while self.dealer.is_hitting() {
                self.dealer.add();
            }
            if self.dealer.is_busted() {
                for player in &self.players {
                    player.win();
                }
            } else {
                for player in &mut self.players {
                    if player.total > self.dealer.total {
                        player.win();
                    } else if player.total < self.dealer.total {
                        player.bust();
                    } else {
                        player.push();
                    }
                }
            }
            println!("{} score: {}", self.dealer, self.dealer.total);
        }
        
        for player in &mut self.players{
            player.clear();
        }
        self.dealer.deck_of_cards.refresh();
    }
}
}