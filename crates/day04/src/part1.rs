use crate::model::Card;

pub fn solve(input: &Vec<Card>) {
    const POW_BASE: usize = 2;
    let result = input
        .iter()
        .map(|card| {
            let amount_of_winning_nums = card
                .winning_numbers
                .iter()
                .map(|num| if card.numbers.contains(num) { 1 } else { 0 })
                .sum::<u32>();

            if amount_of_winning_nums > 0 {
                POW_BASE.pow(amount_of_winning_nums - 1)
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Part 1 solution: {}", result);
}
