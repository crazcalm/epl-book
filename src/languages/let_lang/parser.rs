use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "languages/let_lang/let.pest"]
struct LetParser;

fn eval(expression: pest::iterators::Pair<Rule>) -> String {
    println!("Eval has been called");

    let result = match expression.as_rule() {
        Rule::a_program => eval(expression.into_inner().next().unwrap()),
        Rule::expression => eval(expression.into_inner().next().unwrap()),
        Rule::zero_exp => match eval(expression.into_inner().next().unwrap()).as_str() {
            "0" => "true".to_string(),
            _ => "false".to_string(),
        },
        Rule::diff_exp => {
            let mut iterator = expression.into_inner();

            let exp_1 = eval(iterator.next().unwrap()).parse::<i32>().unwrap();
            let exp_2 = eval(iterator.next().unwrap()).parse::<i32>().unwrap();

            format!("result: ({})", exp_1 - exp_2)
        }
        Rule::number => {
            println!("found number: {:?}", &expression);
            expression.as_str().to_string()
        }
        _ => panic!("unknown expression: {:?}", &expression),
    };

    format!("{}", result)
}

fn valueOf(expression: pest::iterators::Pairs<Rule>) -> String {
    for pair in expression {
        return eval(pair);
    }

    "No expressions".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_number() {
        let pairs = LetParser::parse(Rule::a_program, "42").unwrap();
        let result = valueOf(pairs);

        assert_eq!(result, "42".to_string());
    }

    #[test]
    fn test_eval_zero_exp() {
        let mut pairs = LetParser::parse(Rule::a_program, "zero?(0)").unwrap();
        let mut result = valueOf(pairs);

        assert_eq!(result, "true".to_string());

        pairs = LetParser::parse(Rule::a_program, "zero?(1)").unwrap();
        result = valueOf(pairs);

        assert_eq!(result, "false".to_string());
    }

    #[test]
    fn test_eval_diff_exp() {
        let pairs = LetParser::parse(Rule::a_program, "-(10, 32)").unwrap();
        let result = valueOf(pairs);

        assert_eq!(result, "result: (-22)".to_string());
    }

    #[test]
    fn test_example() {
        let pairs = LetParser::parse(Rule::a_program, "let save = 42 in -(save, 30)")
            .unwrap_or_else(|e| panic!("{}", e));

        println!("list of pairs: {:#?}", &pairs);

        // Because ident_list is silent, the iterator will idents
        for (index, pair) in pairs.enumerate() {
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:     {:?}", pair.as_rule());
            println!("Span:     {:?}", pair.as_span());
            println!("Text:     {}", pair.as_str());

            // A pair can be converted to an iterator of the tokens which make it up:
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    _ => println!("Inner: {}", inner_pair.as_str()),
                };
            }
        }
    }
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
