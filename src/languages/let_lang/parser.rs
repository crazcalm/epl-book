use pest::Parser;

#[derive(Parser)]
#[grammar = "languages/let_lang/let.pest"]
struct LetParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let expected: Vec<(Rule, String)> = vec![(Rule::number, "98760".to_string())];

        let pairs = LetParser::parse(Rule::number, "98760").unwrap_or_else(|e| panic!("{}", e));

        println!("list of pairs: {:?}", &pairs);

        // Because ident_list is silent, the iterator will idents
        for (index, pair) in pairs.enumerate() {
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:     {:?}", pair.as_rule());
            println!("Span:     {:?}", pair.as_span());
            println!("Text:     {}", pair.as_str());

            assert_eq!(expected[index].0, pair.as_rule());
            assert_eq!(expected[index].1, pair.as_str());

            // A pair can be converted to an iterator of the tokens which make it up:
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    _ => println!("Inner: {}", inner_pair.as_str()),
                };
            }
        }
    }

    #[test]
    fn test_test() {
        assert_eq!(1, 1);
    }
}
