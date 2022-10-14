impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut dp = [0u128; 26];
        for x in s.chars() {
            // may overflow during computing, mod in every step to prevent it
            // (x + y) % n = (x % n + y % n) % n when x >= 0, y >= 0
            dp[x as usize -'a' as usize] = (dp.iter().sum::<u128>() + 1) % (1_000_000_000 + 7);
        }
        (dp.iter().sum::<u128>() % (1_000_000_000 + 7)) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(7, Solution::distinct_subseq_ii(String::from("abc")))
    }
    #[test]
    fn example2() {
        assert_eq!(6, Solution::distinct_subseq_ii(String::from("aba")))
    }
    #[test]
    fn example3() {
        assert_eq!(3, Solution::distinct_subseq_ii(String::from("aaa")))
    }
    #[test]
    fn example_overflow() {
        assert_eq!(216384685, Solution::distinct_subseq_ii(String::from("ophajrtskwvemzzpybodijlrnixjzwuhllbteqcnpatxwbfbhyhnhntypjsmiskryyircromuqpflmkjycxxeokajpfcgmhebxqqoftauketrwoocxqflasfmcoivpslmsvafvsjcwabansddjbhhrrelcfrgwgssbuhfjplkpxfdzvjwtmlbphxeasvacwtvslbnpflmlasxqfeegkbgguuawscvuzrfxiaepkvzrrrsnezluebtwcpnivgnetmlrdyytsooumbsnewejceeaajbzpqphghsafysqbzxvda")))
    }
}

fn main() {
    let sum: i32 = [1,2,3].iter().sum();
    println!("{}", sum);
    println!("Hello, world!");
}
struct Solution;