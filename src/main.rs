mod deck;

fn main() {
    let mut cards = deck::generate();

    let shuffled_cards = deck::shuffle(&mut cards);

    let mut hand1: Vec<deck::Card> = shuffled_cards.drain(0..26).collect();
    let mut hand2: Vec<deck::Card> = shuffled_cards.drain(0..26).collect();

    let mut counter = 0;

    let mut sacrifices: Vec<deck::Card> = Vec::new();

    while (hand1.len() > 10 && hand2.len() > 10 && counter < 100_000) || !sacrifices.is_empty() {
        let h1_card = hand1.remove(0);
        let h2_card = hand2.remove(0);

        let h1_number = h1_card.card_number;
        let h2_number = h2_card.card_number;

        sacrifices.push(h1_card);
        sacrifices.push(h2_card);

        if h1_number > h2_number {
            hand1.append(&mut sacrifices);
        } else if h1_number < h2_number {
            hand2.append(&mut sacrifices);
        } else {
            let h1_sacs = &[hand1.remove(0), hand1.remove(0), hand1.remove(0)];
            let h2_sacs = &[hand2.remove(0), hand2.remove(0), hand2.remove(0)];
            sacrifices.extend_from_slice(h1_sacs);
            sacrifices.extend_from_slice(h2_sacs);
        }

        counter += 1;
        report(&hand1, &hand2, &sacrifices, counter);
    }

    if hand1.len() > hand2.len() {
        println!("player 1 wins");
    } else {
        println!("player 2 wins");
    }
}

fn report(h1: &Vec<deck::Card>, h2: &Vec<deck::Card>, sacrifices: &Vec<deck::Card>, counter: i32) {
    println!("---------Turn {}----------", counter);

    if !sacrifices.is_empty() {
        println!("Fighting for {} cards!", sacrifices.len());
    }

    println!("Hand1 {}", h1.len());
    for card in h1 {
        print!("{}", card);
    }

    println!("");

    println!("Hand2 {}", h2.len());
    for card in h2 {
        print!("{}", card);
    }

    println!("");
}
