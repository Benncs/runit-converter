// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ElementUnit, Unit, UnitError};
const UNSAFE_CHAR: [char; 8] = [';', '&', '|', '>', '<', '$', '!', '`'];
fn is_safe(input: &str) -> bool {
    input.chars().all(|c| !UNSAFE_CHAR.contains(&c))
}
pub trait UnitParser {
    fn set_delimiter(&mut self, delimiter: &str) -> bool;
    fn set_exp_symbol(&mut self, symbol: &str) -> bool;
    fn get_delimiter(&self) -> &String;
    fn get_exp_symbol(&self) -> &String;
    fn parse_element_unit(&self, text: &str) -> Result<ElementUnit, UnitError>;
    fn parse_unit(&self, text: &str) -> Result<Unit, UnitError>;
}

pub struct InlineUnitParser {
    delimiter: String,
    exp_symbol: String,
}

impl Default for InlineUnitParser {
    fn default() -> Self {
        Self {
            delimiter: String::from("*"),
            exp_symbol: String::from("^"),
        }
    }
}
const FULL_PARSE_N_SPLIT: usize = 2;

impl UnitParser for InlineUnitParser {
    fn set_delimiter(&mut self, delimiter: &str) -> bool {
        if !is_safe(delimiter) {
            return false;
        }
        self.delimiter = delimiter.to_string();
        true
    }

    fn set_exp_symbol(&mut self, symbol: &str) -> bool {
        if !is_safe(symbol) {
            return false;
        }
        self.exp_symbol = symbol.to_string();
        true
    }

    fn get_delimiter(&self) -> &String {
        &self.delimiter
    }

    fn get_exp_symbol(&self) -> &String {
        &self.exp_symbol
    }

    fn parse_element_unit(&self, text: &str) -> Result<ElementUnit, UnitError> {
        if !is_safe(text) {
            return Err(UnitError::BadUnit("Impossible to parse".to_owned()));
        }
        let splited: Vec<&str> = text.split(&self.exp_symbol).collect();
        if splited.len() == FULL_PARSE_N_SPLIT {
            Ok(ElementUnit::new(
                splited[0],
                splited[1]
                    .parse::<f64>()
                    .map_err(|_| UnitError::ParseError("Bad exponential".to_owned()))?,
            ))
        } else {
            Err(UnitError::ParseError(text.to_owned()))
        }
    }

    fn parse_unit(&self, text: &str) -> Result<Unit, UnitError> {
        let vec_u: Result<Vec<ElementUnit>, UnitError> = text
            .split(&self.delimiter)
            .map(|pu_str| self.parse_element_unit(pu_str))
            .collect();

        Ok(Unit::from_vec(vec_u?))
    }
}

#[cfg(test)]
mod test {
    use super::{InlineUnitParser, UnitParser};
    use proptest::prelude::*;
    #[test]
    fn t_simple_parse_element_unit() {
        let input = "kg^1";
        let parser = InlineUnitParser::default();
        let unit = parser.parse_element_unit(input).unwrap();
        assert!(unit.name == *"kg");
        assert!(unit.exp() == 1.);
    }

    #[test]
    fn t_simple_parse_element_unit_2() {
        let input = "Pa^-5";
        let parser = InlineUnitParser::default();
        let unit = parser.parse_element_unit(input).unwrap();
        assert!(unit.name == *"Pa");
        assert!(unit.exp() == -5.);
    }

    #[test]
    fn t_simple_parse_unit() {
        let input = "kg^1";
        let parser = InlineUnitParser::default();
        let unit = parser.parse_unit(input).unwrap();
        assert!(unit.partials.len() == 1);

        assert!(unit.partials[0].name == *"kg");
        assert!(unit.partials[0].exp() == 1.);
    }
    #[test]
    fn t_parse_unit() {
        let input = "kg^1*Pa^-6";
        let parser = InlineUnitParser::default();
        let unit = parser.parse_unit(input).unwrap();
        assert!(unit.partials.len() == 2);

        assert!(unit.partials[0].name == *"kg");
        assert!(unit.partials[0].exp() == 1.);

        assert!(unit.partials[1].name == *"Pa");
        assert!(unit.partials[1].exp() == -6.);
    }

    #[test]
    fn t_setter_getter_parse() {
        let mut parser = InlineUnitParser::default();
        assert!(parser.get_delimiter() == "*");
        assert!(parser.get_exp_symbol() == "^");
        parser.set_delimiter("**");
        parser.set_exp_symbol("^^");
        assert!(parser.get_delimiter() == "**");
        assert!(parser.get_exp_symbol() == "^^");
    }

    proptest! {
        #[test]
        fn fuzzy_parse_element_unit(name in "[a-zA-Z]{1,5}", exp in -10i32..10) {
            let parser = InlineUnitParser::default();
            let input = format!("{}^{}", name, exp);
            let result = parser.parse_element_unit(&input);
            prop_assert!(result.is_ok());
            let unit = result.unwrap();
            prop_assert_eq!(&unit.name, &name);
            prop_assert_eq!(unit.exp(), exp as f64);
        }
    }

    proptest! {
        #[test]
        fn fuzzy_parse_unit(
            names in prop::collection::vec("[a-zA-Z]{1,5}", 1..5),
            exps in prop::collection::vec(-5i32..5, 1..5),
        ) {
            let len = names.len().min(exps.len());
            let parts: Vec<String> = (0..len)
                .map(|i| format!("{}^{}", names[i], exps[i]))
                .collect();
            let input = parts.join("*");
            let parser = InlineUnitParser::default();
            let result = parser.parse_unit(&input);
            prop_assert!(result.is_ok());

            let unit = result.unwrap();
            prop_assert_eq!(unit.partials.len(), len);

            for i in 0..len {
                prop_assert_eq!(&unit.partials[i].name, &names[i]);
                prop_assert_eq!(unit.partials[i].exp(), exps[i] as f64);
            }
        }
    }
}
