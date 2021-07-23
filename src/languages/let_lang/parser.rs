#[allow(unused_imports)]
use pest::iterators::Pair;
#[allow(unused_imports)]
use pest::Parser;

#[allow(unused)]
use crate::chapter_2_2::{BasicStack, Env, Stack};

#[derive(Parser)]
#[grammar = "languages/let_lang/let.pest"]
struct LetParser;

#[allow(unused)]
fn eval(
    expression: pest::iterators::Pair<Rule>,
    environ: &Stack<(String, i32)>,
) -> (String, Stack<(String, i32)>) {
    println!("Eval has been called");

    let result = match expression.as_rule() {
        Rule::a_program => eval(expression.into_inner().next().unwrap(), environ),
        Rule::expression => eval(expression.into_inner().next().unwrap(), environ),
        Rule::zero_exp => match eval(expression.into_inner().next().unwrap(), environ)
            .0
            .as_str()
        {
            "0" => ("true".to_string(), environ.clone()),
            _ => ("false".to_string(), environ.clone()),
        },
        Rule::diff_exp => {
            let mut iterator = expression.into_inner();

            let exp_1 = eval(iterator.next().unwrap(), environ)
                .0
                .parse::<i32>()
                .unwrap();
            let exp_2 = eval(iterator.next().unwrap(), environ)
                .0
                .parse::<i32>()
                .unwrap();

            (format!("{}", exp_1 - exp_2), environ.clone())
        }
        Rule::if_exp => {
            let mut iterator = expression.into_inner();

            let if_exp = eval(iterator.next().unwrap(), environ);

            match if_exp.0.as_str() {
                "true" => eval(iterator.next().unwrap(), environ),
                _ => {
                    let _ = iterator.next();
                    eval(iterator.next().unwrap(), environ)
                }
            }
        }
        Rule::let_exp => {
            let mut iterator = expression.into_inner();

            let ident_exp = iterator.next().unwrap();
            let value_exp = iterator.next().unwrap();
            let in_exp = iterator.next().unwrap();

            // Evaluate the value for the identifier
            let value = eval(value_exp, environ);

            // Adding identifier's value to the environment
            let new_environment = environ.extend_env(
                ident_exp.as_str().to_string(),
                value.0.parse::<i32>().unwrap(),
            );

            eval(in_exp, &new_environment)
        }
        Rule::number => {
            println!("found number: {:?}", &expression);
            (expression.as_str().to_string(), environ.clone())
        }
        Rule::ident => {
            let value = environ.apply_env(|x| x, expression.as_str().to_string());
            match value {
                Some(x) => (x.to_string(), environ.clone()),
                None => panic!(
                    "value {} does not exist in this environment",
                    expression.as_str()
                ),
            }
        }
        _ => panic!("unknown expression: {:?}", &expression),
    };

    let testing = (result.0, result.1.clone());
    println!("Eval returned value: {:?}", testing);
    testing
}

#[allow(unused)]
fn value_of(
    expression: pest::iterators::Pairs<Rule>,
    environ: &Stack<(String, i32)>,
) -> (String, Stack<(String, i32)>) {
    for pair in expression {
        return eval(pair, environ);
    }

    panic!("No expression passed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_number() {
        let pairs = LetParser::parse(Rule::a_program, "42").unwrap();
        let environ: Stack<(String, i32)> = Stack::empty_stack();
        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "42".to_string());
    }

    #[test]
    fn test_eval_zero_exp() {
        let mut pairs = LetParser::parse(Rule::a_program, "zero?(0)").unwrap();
        let environ: Stack<(String, i32)> = Stack::empty_stack();
        let mut result = value_of(pairs, &environ);

        assert_eq!(result.0, "true".to_string());

        pairs = LetParser::parse(Rule::a_program, "zero?(1)").unwrap();
        result = value_of(pairs, &environ);

        assert_eq!(result.0, "false".to_string());
    }

    #[test]
    fn test_ident_eval_exist() {
        let pairs = LetParser::parse(Rule::a_program, "variable").unwrap();
        let mut environ: Stack<(String, i32)> = Stack::empty_stack();
        environ = environ.extend_env("variable".to_string(), 42);

        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "42".to_string());
    }

    #[test]
    #[should_panic(expected = "value does_not_exist does not exist in this environment")]
    fn test_ident_eval_false() {
        let pairs = LetParser::parse(Rule::a_program, "does_not_exist").unwrap();
        let mut environ: Stack<(String, i32)> = Stack::empty_stack();
        environ = environ.extend_env("variable".to_string(), 42);

        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "42".to_string());
    }

    #[test]
    fn test_if_eval_exp_true() {
        let pairs =
            LetParser::parse(Rule::a_program, "if zero?(-(10,10)) then 42 else 11").unwrap();
        let environ: Stack<(String, i32)> = Stack::empty_stack();
        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "42".to_string());
    }
    #[test]
    fn test_if_eval_exp_false() {
        let pairs =
            LetParser::parse(Rule::a_program, "if zero?(-(10, 20)) then 42 else 11").unwrap();
        let environ: Stack<(String, i32)> = Stack::empty_stack();
        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "11".to_string())
    }

    #[test]
    fn test_eval_diff_exp() {
        let pairs = LetParser::parse(Rule::a_program, "-(10, 32)").unwrap();
        let environ: Stack<(String, i32)> = Stack::empty_stack();
        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "-22".to_string());
    }
    #[test]
    fn test_nested_diff_exp() {
        let pairs = LetParser::parse(Rule::a_program, "-(10, -(20, 10))").unwrap();
        let environ: Stack<(String, i32)> = Stack::empty_stack();
        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "0".to_string());
    }

    #[test]
    fn test_eval_let_exp() {
        let pairs = LetParser::parse(Rule::a_program, "let save = 42 in -(save, 41)").unwrap();
        let environ: Stack<(String, i32)> = Stack::empty_stack();
        let result = value_of(pairs, &environ);

        assert_eq!(result.0, "1".to_string());
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
}
