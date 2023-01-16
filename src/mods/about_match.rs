enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn get_poker_num(c: PokerCard) -> u8 {
    match c {
        PokerCard::Clubs(num) =>num,
        PokerCard::Spades(num) =>num,
        PokerCard::Diamonds(num) =>num,
        PokerCard::Hearts(num) =>num,
    }
}

pub fn about_match() {
    let c = PokerCard::Clubs(10);
    let r = get_poker_num(c);
    println!("{:?}",r)
}