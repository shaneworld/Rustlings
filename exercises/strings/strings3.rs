// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string!
    let mut start = 0;
    let mut end = 0;
    for (index, c) in input.char_indices() {
        if c != ' ' {
            start = index;
            break;
        }
    }

    for (index, c) in input.char_indices().rev() {
        if c != ' ' {
            end = index;
            break;
        }
    }

   input[start .. end+1].to_string()
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string! There's multiple ways to do this!
    let mut copy: String = input.to_string();
    copy.push_str(" world!");
    copy
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    let mut words: Vec<&str> = input.split_whitespace().collect();
    let length = words.len();
    for i in 0..length {
        if words[i] == "cars" {
            words[i] = "balloons";
        }
    }
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
