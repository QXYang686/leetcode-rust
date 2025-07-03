fn main() {
    println!("Hello, world!");
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_possible_string_count() {
        assert_eq!(5, Solution::possible_string_count("aabbccdd".to_string(), 7));
        assert_eq!(1, Solution::possible_string_count("aabbccdd".to_string(), 8));
        assert_eq!(8, Solution::possible_string_count("aaabbb".to_string(), 3));
    }
}

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let k = k as usize;
        const MOD: i64 = 1_000_000_007;
        let n = word.len();

        // 将 word 处理为重复字符次数分段
        let mut cnt = 1;
        let mut freq = vec![];
        let word_chars: Vec<char> = word.chars().collect();
        for i in 1..n {
            if word_chars[i] == word_chars[i - 1] {
                cnt += 1;
            } else {
                freq.push(cnt);
                cnt = 1;
            }
        }
        freq.push(cnt);

        // 所有可能的原始输入方案数
        let mut ans = 1;
        for &o in &freq {
            ans = ans * o as i64 % MOD;
        }
        // 每种字符仅出现一次就达成长度限制了，所有方案都可用
        if freq.len() >= k {
            return ans as i32;
        }

        // f(i,j) 到freq[i]截止长度为j的原始方案数
        // f(i,j) = sum_{j'=1}^{freq[i]}{f(i-1,j-j')}
        // j'=1到freq[i]连续求和，通过前缀和可将复杂度降至O(1)
        // g(i,j) = sum_{j'=0}^{j}{f(i,j')}
        // f(i,j) = g(i-1,j-1) - g(i-1,j-freq[i]-1)
        let mut f = vec![0; k];
        let mut g = vec![1; k];
        f[0] = 1;

        for &num in &freq {
            let mut f_new = vec![0; k];
            for j in 1..k {
                f_new[j] = g[j - 1];
                if j as i32 - num - 1 >= 0 {
                    f_new[j] = (f_new[j] - g[j - num as usize - 1] + MOD) % MOD;
                }
            }
            let mut g_new = vec![0; k];
            g_new[0] = f_new[0];
            for j in 1..k {
                g_new[j] = (g_new[j - 1] + f_new[j]) % MOD;
            }
            g = g_new;
        }

        ((ans - g[k - 1] as i64 + MOD) % MOD) as i32
    }
}
