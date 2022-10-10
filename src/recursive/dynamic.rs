/// cashes保存各种面额的纸币，amount找零的钱，寻找最少张数


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


pub fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut[u32], cashes_used: &mut[u32]) -> u32 {
    for denm in 1..= amount {
        let mut min_cashes_num = denm;
        let mut used_cashes = 1;
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            let cashes_num = 1 + min_cashes[index];
            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                used_cashes = *c;
            }
        }
        min_cashes[denm as usize] = min_cashes_num;
        cashes_used[denm as usize] = used_cashes;
    }
    min_cashes[amount as usize]
}

pub fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("${}", &curr);
        amount -= curr;
    }
}

pub fn fibnacci_dp(n: u32) -> u32 {
    let mut dp = [1, 1];

    for i in 2..=n {
        let id1 = (i % 2) as usize;
        let id2 = ((i - 1) % 2) as usize;
        let id3 = ((i - 2) % 2) as usize;
        dp[id1] = dp[id2] + dp[id3];
    }

    dp[((n - 1) % 2) as usize]
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

    #[test]
    pub fn dy_rec_test(){
        let amount = 81u32; let cashes = [1, 5, 10, 20, 50];
        let mut min_cashes : [ u32 ; 82] = [ 0 ; 82 ] ; 
        let mut cashes_used : [ u32 ; 82] = [ 0 ; 82 ] ; 
        let cs_num = dp_rec_mc(&cashes , amount , &mut min_cashes , &mut cashes_used ); 
        println!("Refund for ￥{} need {} cashes :" , amount , cs_num);
        print_cashes(&cashes_used , amount);
    }

    #[test]
    pub fn fibnacci_dp_test() {
        let n = 5;
        let res = fibnacci_dp(n);
        println!("{}", res);
    }
}