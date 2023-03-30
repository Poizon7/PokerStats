#![allow(non_snake_case)]

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::sync::{Arc, Mutex};
use std::time::Instant;

use tokio;

#[derive(Clone, PartialEq)]
enum Suit {
    Clubs,
    Dimonds,
    Spades,
    Hearts,
}

impl Suit {
    fn to(suit: u8) -> Result<Self, &'static str> {
        match suit {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Dimonds),
            2 => Ok(Suit::Spades),
            3 => Ok(Suit::Hearts),
            _ => Err("Invalid number"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
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

impl Value {
    fn to(value: u8) -> Result<Self, &'static str> {
        match value {
            0 => Ok(Value::Ace),
            1 => Ok(Value::Two),
            2 => Ok(Value::Three),
            3 => Ok(Value::Four),
            4 => Ok(Value::Five),
            5 => Ok(Value::Six),
            6 => Ok(Value::Seven),
            7 => Ok(Value::Eight),
            8 => Ok(Value::Nine),
            9 => Ok(Value::Ten),
            10 => Ok(Value::Jack),
            11 => Ok(Value::Queen),
            12 => Ok(Value::King),
            _ => Err("Invalid number"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Hand {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    None,
}

#[derive(Clone)]
struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    fn new(value: u8, suit: u8) -> Self {
        let suit = Suit::to(suit).unwrap();
        let value = Value::to(value).unwrap();
        Card {
            suit: suit,
            value: value,
        }
    }
}

#[derive(Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        Deck {
            cards: (0..52)
                .into_iter()
                .map(|x| Card::new(x % 13, x % 4))
                .collect(),
        }
    }
}

fn Sort(hand: &mut [Card; 5]) {
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 1..5 {
            if (hand[i].value.clone() as i32) < (hand[i - 1].value.clone() as i32) {
                let temp = hand[i].value.clone();
                hand[i].value = hand[i - 1].value.clone();
                hand[i - 1].value = temp;

                sorted = false;
            }
        }
    }
}

async fn test(mut deck: Deck, stats: Arc<Mutex<[u128; 9]>>, rounds: u64) {
    let mut rng = thread_rng();

    let mut tempStats = [0; 9];
    for _r in 0..rounds {
        deck.cards.shuffle(&mut rng);

        let mut hand = [
            deck.cards[0].clone(),
            deck.cards[1].clone(),
            deck.cards[2].clone(),
            deck.cards[3].clone(),
            deck.cards[4].clone(),
        ];

        Sort(&mut hand);

        let mut highest = Hand::None;

        for i in 0..4 {
            if hand[i].value == hand[i + 1].value {
                if hand[0].value == hand[1].value && hand[2].value == hand[3].value {
                    highest = Hand::TwoPair;
                } else if hand[0].value == hand[1].value && hand[3].value == hand[4].value {
                    highest = Hand::TwoPair;
                } else if hand[1].value == hand[2].value && hand[3].value == hand[4].value {
                    highest = Hand::TwoPair;
                } else {
                    highest = Hand::Pair;
                }
            }
        }

        if highest == Hand::Pair {
            for i in 0..3 {
                if hand[i].value == hand[i + 1].value {
                    if hand[i].value == hand[i + 2].value {
                        if i == 0 {
                            if hand[3].value == hand[4].value {
                                highest = Hand::FullHouse;
                            } else {
                                highest = Hand::ThreeOfAKind;
                            }
                        } else if i == 2 {
                            if hand[0].value == hand[1].value {
                                highest = Hand::FullHouse;
                            } else {
                                highest = Hand::ThreeOfAKind;
                            }
                        } else {
                            highest = Hand::ThreeOfAKind;
                        }
                    }
                }
            }

            if highest == Hand::ThreeOfAKind {
                if hand[0].value == hand[1].value
                    && hand[0].value == hand[2].value
                    && hand[0].value == hand[3].value
                {
                    highest = Hand::FourOfAKind;
                } else if hand[1].value == hand[2].value
                    && hand[1].value == hand[3].value
                    && hand[1].value == hand[4].value
                {
                    highest = Hand::FourOfAKind;
                }
            }
        } else {
            if hand[0].suit == hand[1].suit
                && hand[0].suit == hand[2].suit
                && hand[0].suit == hand[3].suit
                && hand[0].suit == hand[4].suit
            {
                highest = Hand::Flush;
            }

            if !(highest == Hand::Pair
                || highest == Hand::TwoPair
                || highest == Hand::ThreeOfAKind
                || highest == Hand::FullHouse
                || highest == Hand::FourOfAKind)
            {
                if (hand[0].value.clone() as i32) + 1 == (hand[1].value.clone() as i32)
                    && (hand[1].value.clone() as i32) + 1 == (hand[2].value.clone() as i32)
                    && (hand[2].value.clone() as i32) + 1 == (hand[3].value.clone() as i32)
                    && (hand[3].value.clone() as i32) + 1 == (hand[4].value.clone() as i32)
                {
                    if highest == Hand::Flush {
                        highest = Hand::StraightFlush;
                    } else {
                        highest = Hand::Straight;
                    }
                }
            }
        }

        if highest == Hand::StraightFlush {
            tempStats[0] += 1;
        } else if highest == Hand::FourOfAKind {
            tempStats[1] += 1;
        } else if highest == Hand::FullHouse {
            tempStats[2] += 1;
        } else if highest == Hand::Flush {
            tempStats[3] += 1;
        } else if highest == Hand::Straight {
            tempStats[4] += 1;
        } else if highest == Hand::ThreeOfAKind {
            tempStats[5] += 1;
        } else if highest == Hand::TwoPair {
            tempStats[6] += 1;
        } else if highest == Hand::Pair {
            tempStats[7] += 1;
        } else {
            tempStats[8] += 1;
        }
    }

    let mut stats = stats.lock().unwrap();
    for i in 0..9 {
        stats[i] += tempStats[i];
    }
}

#[tokio::main]
async fn main() {
    let deck = Deck::new();

    let stats: [u128; 9] = [0; 9];
    let stats: Arc<Mutex<[u128; 9]>> = Arc::new(Mutex::new(stats));

    let mut handles = Vec::new();
    let max: u64 = 1000000000; // 1 000 000 000
    let statusBar = 10000000; // 10 000 000

    let time = Instant::now();
    for _r in 0..(max / statusBar) {
        let deck = deck.clone();

        let stats = stats.clone();

        handles.push(tokio::spawn(async move {
            test(deck, stats, statusBar).await;
        }));
    }

    let mut counter = 0;
    for handle in handles {
        handle.await.unwrap();
        counter += 1;
        println!("{}0M, {:?}", counter, time.elapsed());
    }

    let time = time.elapsed();

    println!("{:?}", time);
    //println!("{:?}", time / 1 / max);

    let stats = stats.lock().unwrap();

    println!("Straight Flush: {:?}", stats[0]);
    println!("Four Of A Kind: {:?}", stats[1]);
    println!("Full House: {:?}", stats[2]);
    println!("Flush: {:?}", stats[3]);
    println!("Straight: {:?}", stats[4]);
    println!("Three Of A Kind: {:?}", stats[5]);
    println!("Two Pair: {:?}", stats[6]);
    println!("Pair: {:?}", stats[7]);
    println!("None: {:?}", stats[8]);

    println!();

    println!("Straight Flush: {:?}", (stats[0] as f64) / (max as f64));
    println!("Four Of A Kind: {:?}", (stats[1] as f64) / (max as f64));
    println!("Full House: {:?}", (stats[2] as f64) / (max as f64));
    println!("Flush: {:?}", (stats[3] as f64) / (max as f64));
    println!("Straight: {:?}", (stats[4] as f64) / (max as f64));
    println!("Three Of A Kind: {:?}", (stats[5] as f64) / (max as f64));
    println!("Two Pair: {:?}", (stats[6] as f64) / (max as f64));
    println!("Pair: {:?}", (stats[7] as f64) / (max as f64));
    println!("None: {:?}", (stats[8] as f64) / (max as f64));
}
