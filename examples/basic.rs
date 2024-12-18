fn main() {
    let re = regex_automata_macro::regex_sparse!("(?iu)a(b|c)*d");

    println!("{} + {}", re.forward().memory_usage(), re.reverse().memory_usage());

    assert!(re.is_match("abbbcd"));
}
