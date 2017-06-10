mod deck;

fn main() {
    let mut cards = deck::generate();

    let shuffled_cards = deck::shuffle(&mut cards);

    for card in shuffled_cards {
        print!("{}", card);
    }
}
