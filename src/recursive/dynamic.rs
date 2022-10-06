pub fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    let mut min_cashes = amount;

    if cashes.contains(&amount) {
        return 1;
    } else {
        for c in cashes.iter().filter(|&c| *c <= amount).collect::<Vec<&u32>>() {
            let num_cashes = 1 + rec_mc1(&cashes, amount - c);
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }

    min_cashes
}

pub fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut[u32]) -> u32 {
    let mut min_cashes_num = amount;

    if cashes.contains(&amount) {
        min_cashes[amount as usize] = 1;
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        return min_cashes[amount as usize];
    } else {
        for c in cashes.iter()
                             .filter(|c| *(*c) <= amount)
                             .collect::<Vec<&u32>>() {
            let cashes_num = 1 + rec_mc2(cashes, amount - c, min_cashes);
            
            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                min_cashes[amount as usize] = min_cashes_num;
            }
        }
    }

    min_cashes_num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn rec_mc_test() {
        let cashes = [1, 5, 10, 20, 50];
        let amount = 31u32;
        let cashes_num = rec_mc1(&cashes, amount);
        println!("need refund {} cashes", cashes_num);
    }
}