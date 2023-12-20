use std::{fs, collections::{HashSet, VecDeque}};

struct Queue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: VecDeque::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, el: T) {
        self.queue.push_back(el);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read file input.txt");

    let cards:Vec<HashSet<u32>> = parse_input(&contents);

    let mut queue: Queue<usize> = Queue::new();
    for (i, _) in cards.iter().enumerate() {
        queue.enqueue(i);
    }

    let mut number_of_cards = 0;
    while queue.length() > 0 {
        let el = queue.dequeue().expect("Should have been able to dequeue");

        if el >= cards.len() {
            continue;
        }

        number_of_cards += 1;

        let card = cards.get(el).expect("Should have been able to get card");
        let number_matches = card.len();

        for i in 0..number_matches {
            queue.enqueue(el + i + 1);
        }
    }

    println!("Number of cards: {}", number_of_cards);
}

fn parse_input(contents: &str) -> Vec<HashSet<u32>>
{
    let mut cards: Vec::<HashSet<u32>>  = Vec::new();
    
    for line in contents.lines() {
        let mut iter = line.split(":");
        iter.next(); // Skip card number
        let card = iter.next()
            .expect("Should have card");
        let mut card_iter = card.split("|");
        
        let left_side = parse_card_numbers(card_iter.next().expect("Should have left side"));
        let right_side = parse_card_numbers(card_iter.next().expect("Should have right side"));

        cards.push(left_side.intersection(&right_side).copied().collect());
    }

    cards
}

fn parse_card_numbers(card: &str) -> HashSet<u32>
{
    let ret: HashSet<u32>  = HashSet::from_iter(card.split(' ')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u32>().expect("Should have been able to parse as number")));
    ret
}