use crate::DataStruct::stack::Stack;

const BASESTR:[&str; 16] = ["0", "1", "2", "3", "4", "5",
                            "6", "7", "8", "9", "A", "B",
                            "C", "D", "E", "F"];

pub fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        num2str_rec(num/base, base) + 
            BASESTR[(num % base) as usize]
    }
}

pub fn num2str_stk(mut num: i32, base: i32) -> String {
    let digits: [&str; 16] = ["0", "1", "2", "3", "4", "5",
                            "6", "7", "8", "9", "A", "B",
                            "C", "D", "E", "F"];
    let mut rem_stack = Stack::new();

    while num > 0 {
        if num < base {
            rem_stack.push(num);
        } else {
            rem_stack.push(num % base);
        }
        num / base;
    }

    let mut numstr = "".to_string();
    while !rem_stack.is_empty() {
        numstr += digits[rem_stack.pop().unwrap() as usize];
    }

    numstr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn num2str_rec_test(){
        let num = 100;
        let sb = num2str_rec(num, 2);
        let so = num2str_rec(num, 8);
        let sh = num2str_rec(num, 16);
        println!("{} is b{}, o{}, x{}", num, sb, so, sh);
    }

    #[test]
    fn num2str_stk_test(){
        let num = 100;
        let sb = num2str_stk(num, 2);
        let so = num2str_stk(num, 8);
        let sh = num2str_stk(num, 16);
        println!("{} is b{}, o{}, x{}", num, sb, so, sh);
    }

    #[test]
    fn num2str_test_all(){
        num2str_rec_test();
    }
}