mod deck;

fn main() {
    let mut cards = deck::generate();

    let shuffled_cards = deck::shuffle(&mut cards);

    let mut hand1: Vec<deck::Card> = shuffled_cards.drain(0..26).collect();
    let mut hand2: Vec<deck::Card> = shuffled_cards.drain(0..26).collect();

    let mut counter = 0;

    while hand1.len() > 10 && hand2.len() > 10 && counter < 100_000 {
        let h1_card = hand1.remove(0);
        let h2_card = hand2.remove(0);

        if h1_card.card_number > h2_card.card_number {
            hand1.push(h1_card);
            hand1.push(h2_card);
        } else if h1_card.card_number < h2_card.card_number {
            hand2.push(h1_card);
            hand2.push(h2_card);
        } else {
            hand1.push(h1_card);
            hand2.push(h2_card);
        }

        counter += 1;
        report(&hand1, &hand2, counter);
    }

    if hand1.len() > hand2.len() {
        println!("player 1 wins");
    } else {
        println!("player 2 wins");
    }
}

fn report(h1: &Vec<deck::Card>, h2: &Vec<deck::Card>, counter: i32) {
    println!("Turn {}", counter);

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
