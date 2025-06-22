// Interactive Blackjack game created to learn Rust
// Developed solely by Brandon David Delliquadri

use std::string;

use rand::seq::SliceRandom;
use rand::rng;
use std::io;

#[derive(Debug, Copy, Clone)]
pub struct Card {
    suit: char,
    rank: char,
}

pub struct Deck{
    pub deck: Vec<Card>,
}

impl Deck{
    pub fn new() -> Self{   //Generate 52 card deck with 4 suits, 13 ranks
        Deck { deck: Vec::new() }

    }
    fn populate_deck(&mut self){
        let suits = vec!['S', 'H', 'D', 'C'];
        let ranks = vec!['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
        for x in suits.iter() {
            for y in ranks.iter() {
                let new_card = Card::new(*y,*x);
                self.deck.push(new_card);
            }
        }
    }

    fn shuffle(&mut self){   //Randomly reorder the vector
        let mut rng = rng();
        self.deck.shuffle(&mut rng);
    }

    fn draw (&mut self) -> Card{
        if self.deck.is_empty(){
            self.populate_deck();
            self.shuffle();
            self.draw()
        }else{
            self.deck.pop().unwrap()
        }
    }

    // pub fn print_deck(&self){
    //     println!("\nCurrent Deck: {:?}", self.deck);
    // }
}

impl Card{
    fn new(rank: char, suit: char) -> Self{
        Card {rank,suit}
    }

    pub fn print_card(&self){
        let suit: String = char_to_string(self.suit);
        let rank: String = char_to_string(self.rank);
        print!("{} of {}", rank, suit);
    }

    pub fn card_value (& self) -> u32{
        if self.rank == 'Q' || self.rank == 'K' || self.rank == 'J' || self.rank == 'T'{10} 
        else if self.rank == 'A'{11}
        else{
            self.rank.to_digit(10).unwrap()
        }
    }
}

pub fn char_to_string(char: char) -> String{
    if char == 'S'{
        String::from("Spades")
    }else if char == 'H'{
        String::from("Hearts")
    } else if char == 'D'{
        String::from("Diamonds")
    } else if char == 'C'{
        String::from("Clubs")
    }else if char == 'T'{
        String::from("10")
    }else if char == 'J'{
        String::from("Jack")
    }else if char == 'Q'{
        String::from("Queen")
    }else if char == 'K'{
        String::from("King")
    }else if char == 'A'{
        String::from("Ace")
    }else{
        String::from(char)
    }
}

pub fn get_input() -> String{
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let trimmed_input = user_input.trim().to_string();
    trimmed_input
}

pub fn dealer_draw(){
    print!("The dealer draws himself a ");
}

pub fn player_draw(){
    print!("The dealer draws you a ");
}

pub fn game_loop(deck: &mut Deck){
    loop{
        println!("Please press 1 and enter to start a new game:");
        let mut player_hand: Vec<Card> = Vec::new();
        let mut dealer_hand: Vec<Card> = Vec::new();
        let trimmed_input = get_input();
        if trimmed_input == "1"{
            println!("\"Let's get this started\" says the dealer.");
            player_draw();
            let current_card = deck.draw();
            player_hand.push(current_card);
            current_card.print_card();
            println!("\n");
        }else{
            break;
        }

    }
    
}

fn main() {
    let mut game_deck = Deck::new();
    game_deck.populate_deck();
    game_deck.shuffle();

    println!("* * * * * * * * * * * * * * * * * * * * * * * * * * * *");
    println!("* * * * * * * * * * * * * * * * * * * * * * * * * * * *");
    println!("* * * Welcome to Windows Blackjack Simulator 2025 * * *");
    println!("* * * * * * * * * * * * * * * * * * * * * * * * * * * *");
    println!("* * * * * * * * * * * * * * * * * * * * * * * * * * * *");

    game_loop(&mut game_deck);
    // let mut count = 0u32;
    // loop{
    //     count +=1;
    //     let temp_card = game_deck.draw();
    //     temp_card.print_card();
    //     print!("        This card's value is {}", temp_card.card_value());
    //     println!();
    //     if count ==100{
    //         break;
    //     }
    // }
    
}
