#![allow(clippy::uninlined_format_args)]
use std::fmt::{Display, Formatter};

use crate::value::Value;

use crate::encode::{
    encoding::{ArrayEncoding, BaseEncoding, Encoding, TextEncoding},
    types::{Brackets, Separator},
};

#[derive(Debug, Default)]
pub enum Classification<'a> {
    Array(Array<'a>),
    Integer(Integer<'a>),
    Text(Text<'a>),
    #[default]
    Empty,
}

impl Display for Classification<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Classification::Array(arr) => arr.fmt(f),
            Classification::Integer(int) => int.fmt(f),
            Classification::Text(text) => write!(f, "{}", text),
            Classification::Empty => write!(f, "Empty"),
        }
    }
}

impl From<&Classification<'_>> for Value {
    fn from(classification: &Classification) -> Self {
        fn to_map(classification: &Classification) -> Value {
            Value::Map(vec![
                (
                    Value::from("encoding"),
                    Value::from(&classification.encoding()),
                ),
                (
                    Value::from("score"),
                    Value::from(classification.score() as i64),
                ),
                (
                    Value::from("value"),
                    Value::from(classification.value_string()),
                ),
            ])
        }
        to_map(classification)
    }
}

impl Eq for Classification<'_> {}

impl PartialEq for Classification<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score() && self.encoding() == other.encoding()
    }
}

impl PartialOrd for Classification<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Classification<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.score().cmp(&other.score()) {
            std::cmp::Ordering::Equal => self.encoding().cmp(&other.encoding()),
            ord => ord,
        }
    }
}

impl<'a> From<Array<'a>> for Classification<'a> {
    fn from(arr: Array<'a>) -> Self {
        Classification::Array(arr)
    }
}

impl<'a> From<Integer<'a>> for Classification<'a> {
    fn from(int: Integer<'a>) -> Self {
        Classification::Integer(int)
    }
}

impl Classification<'_> {
    pub fn encoding(&self) -> Encoding {
        match self {
            Classification::Array(arr) => Encoding::Array(ArrayEncoding::new(
                arr.collapse().iter().map(|c| c.encoding()).collect(),
                Some(arr.brackets.clone()),
                Some(arr.separator),
            )),
            Classification::Integer(i) => Encoding::Base(BaseEncoding::new(i.base)),
            Classification::Text(t) => Encoding::Text(t.encoding),
            Classification::Empty => Encoding::Empty,
        }
    }

    pub fn is_lines(&self) -> bool {
        match self {
            Classification::Array(arr) => arr.is_lines(),
            _ => false,
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Classification::Array(arr) => arr.score,
            Classification::Integer(int) => int.score,
            Classification::Text(text) => text.score,
            Classification::Empty => usize::MAX,
        }
    }

    pub fn value_string(&self) -> String {
        match self {
            Classification::Array(arr) => arr
                .collapse()
                .iter()
                .map(|c| c.value_string())
                .collect::<Vec<_>>()
                .join(&arr.separator.to_string()),
            Classification::Integer(int) => int.value_string(),
            Classification::Text(text) => text.value_string(),
            Classification::Empty => String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Classification::Empty => true,
            Classification::Array(arr) => arr.values().is_empty(),
            Classification::Text(text) => text.value.is_empty(),
            Classification::Integer(int) => int.value.is_empty(),
        }
    }
}

#[derive(Debug)]
pub struct Array<'a> {
    pub values: Vec<Vec<Classification<'a>>>,
    pub brackets: Brackets,
    pub separator: Separator,
    pub score: usize,
}

impl Display for Array<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.brackets.string_pair().join(
                &self
                    .values
                    .iter()
                    .map(|v| self.brackets.string_pair().join(
                        &v.iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(&self.separator.to_string())
                    ))
                    .collect::<Vec<_>>()
                    .join(&self.separator.to_string())
            )
        )
    }
}

impl<'a> Array<'a> {
    pub fn new(
        values: Vec<Vec<Classification<'a>>>,
        brackets: &Brackets,
        separator: Separator,
        err: usize,
    ) -> Array<'a> {
        Self {
            values,
            brackets: brackets.clone(),
            separator,
            score: err,
        }
    }

    pub fn is_lines(&self) -> bool {
        self.separator.is_newline() && self.brackets.is_none()
    }

    pub fn collapse(&self) -> Vec<&Classification> {
        self.values
            .iter()
            .map(|classifications| {
                classifications
                    .iter()
                    .min()
                    .unwrap_or(&Classification::Empty)
            })
            .collect()
    }

    // Turn array classification into array of arrayclassifications with one classification
    // Only if the classification occurs in every column
    pub fn transpose(&self) -> Vec<Classification> {
        // TODO: add this for better array classification
        todo!()
    }

    pub fn values(&self) -> &Vec<Vec<Classification>> {
        &self.values
    }

    pub fn value_string(&self) -> String {
        self.brackets
            .string_pair()
            .join(
                &self
                    .values
                    .iter()
                    .map(|v| {
                        self.brackets.string_pair().join(
                            &v.iter()
                                .map(|c| c.value_string())
                                .collect::<Vec<_>>()
                                .join(&self.separator.to_string()),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(&self.separator.to_string()),
            )
            .to_string()
    }
}

#[derive(Debug)]
pub struct Integer<'a> {
    pub base: i32,
    pub value: &'a str,
    pub score: usize,
}

impl Display for Integer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.base {
            2 => write!(f, "{}", Encoding::BINARY),
            10 => write!(f, "{}", Encoding::INTEGER),
            16 => write!(f, "{}", Encoding::HEX),
            _ => write!(f, "{{base-{}, {}, {}}}", self.base, self.value, self.score),
        }
    }
}

impl<'a> Integer<'a> {
    pub fn new(base: i32, value: &'a str, err: usize) -> Self {
        Self {
            base,
            value,
            score: err,
        }
    }

    pub fn value_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    pub encoding: TextEncoding,
    pub value: &'a str,
    pub score: usize,
}

impl Display for Text<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.encoding, self.value, self.score)
    }
}

impl<'a> Text<'a> {
    pub fn new(encoding: TextEncoding, value: &'a str, err: usize) -> Self {
        Self {
            encoding,
            value,
            score: err,
        }
    }

    pub fn value_string(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification_ord() {
        let left = Classification::Integer(Integer::new(10, "10", 0));
        let right = Classification::Integer(Integer::new(2, "10", 0));
        let result = left.cmp(&right);
        assert_eq!(result, std::cmp::Ordering::Less);

        let left = Classification::Integer(Integer::new(2, "10", 0));
        let right = Classification::Integer(Integer::new(16, "10", 0));
        let result = left.cmp(&right);
        assert_eq!(result, std::cmp::Ordering::Less);
    }

    // TODO:
    //#[test]
    //fn test_flatten() {
    //    let arr_inner_1 = Array::new(
    //        vec![
    //            vec![
    //                Classification::Integer(Integer::new(10, "10", 0)),
    //                Classification::Integer(Integer::new(16, "10", 0)),
    //            ],
    //            vec![
    //                Classification::Integer(Integer::new(10, "10", 0)),
    //                Classification::Integer(Integer::new(16, "10", 0)),
    //            ],
    //        ],
    //        &Brackets::default(),
    //        Separator::default(),
    //        0,
    //    );
    //    let arr_inner_2 = Array::new(
    //        vec![
    //            vec![
    //                Classification::Integer(Integer::new(10, "10", 0)),
    //                Classification::Integer(Integer::new(16, "10", 0)),
    //            ],
    //            vec![
    //                Classification::Integer(Integer::new(10, "10", 0)),
    //                Classification::Integer(Integer::new(16, "10", 0)),
    //            ],
    //        ],
    //        &Brackets::default(),
    //        Separator::default(),
    //        0,
    //    );
    //    let arr = Array::new(
    //        vec![
    //            vec![Classification::Array(arr_inner_1)],
    //            vec![Classification::Array(arr_inner_2)],
    //        ],
    //        &Brackets::default(),
    //        Separator::default(),
    //        0,
    //    );
    //    let result = arr.flatten();
    //    assert_eq!(result.len(), 2);
    //}
}
