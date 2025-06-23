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


pub fn print_hand(hand: &Vec<Card>){
    print!("You peer down at your hand. You have the following cards: ");
    for x in hand{
        println!();
        x.print_card();
    }
    println!();
}

pub fn dealer_draw(deck: &mut Deck, first_draw: bool) -> Card{
    let current_card = deck.draw();
    if first_draw == true{print!("The dealer draws himself a card face down.");}
    else{
        print!("The dealer draws himself a ");
        current_card.print_card();
    }

    println!("\n");
    current_card
}

pub fn player_draw(deck: &mut Deck) -> Card{
    print!("The dealer draws you a ");
    let current_card = deck.draw();
    current_card.print_card();
    println!("\n");
    current_card
}


pub fn hand_value(cards: &Vec<Card>) -> u32{
    let mut value = 0;
    for x in cards{
        value += x.card_value();
    }
    value
}

pub fn game_loop(deck: &mut Deck){
    loop{
        println!("Please press enter to start a new game:");
        let mut player_hand: Vec<Card> = Vec::new();
        let mut dealer_hand: Vec<Card> = Vec::new();
        let trimmed_input = get_input();
        if trimmed_input == ""{
            println!("\"Let's get this started\" says the dealer.\n\n");
            player_hand.push(player_draw(deck));
            dealer_hand.push(dealer_draw(deck, true));
            dealer_hand.push(dealer_draw(deck, false));
            println!("\n");
            loop{
                println!("\nChoose from the following options:\n1)Look at your hand\n2)Look at dealer's showing card\n3)Hit\n4)Stand");
                let player_choice = get_input();
                if player_choice == "1"{    //Display the current player's hand
                    print_hand(&player_hand);
                }
                if player_choice == "2"{    //Display the dealer's face up card
                    print!("You look over at the dealer. ");
                    if dealer_hand[dealer_hand.len()-1].card_value() == 10{
                        print!("He looks back at you bored as if he has seen this exact moment millions of times. ");
                    }
                    else if dealer_hand[dealer_hand.len()-1].card_value() == 11{
                        print!("He looks back at you slyly. Not a hint of worry permiates their aura. ");
                    }
                    else{
                        print!("The dealer looks disinterested in the game. ");
                    }
                    print!("They are showing a ");
                    dealer_hand[0].print_card();
                    println!(".");

                }
                else if player_choice == "3"{  //Draw another card for the player and then loop again
                    player_hand.push(player_draw(deck)); //should probably check for bust here somehow
                    let player_value = hand_value(&player_hand);
                    if player_value > 21{
                        println!("You have busted with a hand value of {}", player_value);    //Have dealer play through anyways here similar to option 4
                        let mut dealer_value = hand_value(&dealer_hand);
                        loop{
                            if dealer_value < 17{
                                let current_card = dealer_draw(deck, false);
                                dealer_value += current_card.card_value();
                            }
                            else if dealer_value > 21{
                                println!("The dealer busted with a hand value of {}.", dealer_value);
                            }
                            else{
                                println!("The dealer stands with a hand value of {}.", dealer_value);
                                break;
                            }
                        }
                        println!("The dealer gives you a faked sympathetic look. You have lost with a {} against the dealer's {}", player_value, dealer_value);
                        break;
                    }

                }
                else if player_choice == "4"{  //Display the dealer's hand and then draw cards if and until they reach 17+
                    let player_value = hand_value(&player_hand);
                    println!("You stand on a hand value of {}.", player_value);
                    let mut dealer_value = hand_value(&dealer_hand);
                    loop{
                        if dealer_value < 17{
                            let current_card = dealer_draw(deck, false);
                            dealer_value += current_card.card_value();
                        }
                        else if dealer_value > 21{
                            println!("The dealer busted with a hand value of {}.", dealer_value);
                        }
                        else{
                            println!("The dealer stands with a hand value of {}.", dealer_value);
                            break;
                        }
                    }
                    if dealer_value > player_value{
                        println!("The dealer gives you a faked sympathetic look. You have lost with a {} against the dealer's {}", player_value, dealer_value);
                    }
                    else if dealer_value == player_value{
                        println!("The dealer, bored as ever, stares at the cards on the table. There is a push with you and the dealer having {}", player_value);
                    }
                    else{
                        println!("The dealer gives you an unenthusiastic double thumbs up. You beat the dealer's {} with a {}", dealer_value, player_value);
                    }
                    break;
                }
            }
            
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
