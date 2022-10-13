//Your task is to sort a given string.
//Each word in the string will contain a single number.
//This number is the position the word should have in the result.

//Note: Numbers can be from 1 to 9. So 1 will be the first word (not 0).

//If the input string is empty, return an empty string.
//The words in the input String will only contain valid consecutive numbers.
fn order_old(sentence: &str) -> String {
    let mut x: Vec<(String, &str)> = sentence
        .split(' ')
        .map(|a| {
            let n = a.chars().filter(|y| y.is_numeric()).collect::<String>();
            (n, a)
        })
        .collect();
    x.sort();
    x.into_iter().map(|x| x.1).collect::<Vec<&str>>().join(" ")
}

fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
