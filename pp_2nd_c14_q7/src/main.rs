peg::parser!{
    grammar time_parser() for str{
        pub rule time() = hour() ampm() / hour() colon() minute() ampm() / hour() colon() minute()
        rule colon() = ":"
        rule ampm() = "am" / "pm"
        rule hour() = digit() digit() / digit()
        rule minute() = digit() digit()
        rule digit() = ['0'..='9']
    }
}

fn main() {
    assert!(time_parser::time("4am").is_ok());
    assert!(time_parser::time("23:42").is_ok());
    assert!(time_parser::time("3:16").is_ok());
    assert!(time_parser::time("7:38pm").is_ok());
    assert!(time_parser::time("3:16am").is_ok());

}

