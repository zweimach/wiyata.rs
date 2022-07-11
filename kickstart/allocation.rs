pub fn allocation(houses: &Vec<u32>, budget: u32) -> u32 {
    let mut budget = budget;
    let mut total = 0;

    for price in houses {
        if budget < *price {
            break;
        }
        budget -= price;
        total += 1;
    }
    total
}
