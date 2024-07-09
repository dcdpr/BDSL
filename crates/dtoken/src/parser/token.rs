use std::{collections::HashMap, str::FromStr};

use tinyjson::JsonValue;

use crate::{
    error::Error,
    types::{
        alias::Alias, border::Border, color::Color, cubic_bezier::CubicBezier,
        dimension::Dimension, duration::Duration, font_family::FontFamily, font_weight::FontWeight,
        gradient::Gradient, number::Number, shadow::Shadow, stroke_style::StrokeStyle,
        transition::Transition, typography::Typography,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub value: Value,
    pub description: Option<String>,
}

impl Token {
    pub fn from_map(
        map: &HashMap<String, JsonValue>,
        default_type: Option<String>,
    ) -> Result<Self, Error> {
        let value = map
            .get("$value")
            .ok_or(Error::prop("$value", Error::MustExist))?;

        let description = map
            .get("$description")
            .and_then(|v| v.get::<String>())
            .cloned();

        // If $value is an alias, it doesn't require a `$type` field.
        if let Some(alias_str) = value.get::<String>() {
            if let Ok(alias) = Alias::from_str(alias_str) {
                return Ok(Self {
                    value: Value::Alias(alias),
                    description,
                });
            }
        }

        let token_type = map
            .get("$type")
            .and_then(|v| v.get::<String>().cloned())
            .or(default_type)
            .ok_or(Error::prop("$type", Error::MustExist))?;

        let value: Value = match token_type.as_str() {
            "color" => Color::try_from(value).map(Into::into),
            "dimension" => Dimension::try_from(value).map(Into::into),
            "fontFamily" => FontFamily::try_from(value).map(Into::into),
            "fontWeight" => FontWeight::try_from(value).map(Into::into),
            "duration" => Duration::try_from(value).map(Into::into),
            "cubicBezier" => CubicBezier::try_from(value).map(Into::into),
            "number" => Number::try_from(value).map(Into::into),
            "strokeStyle" => StrokeStyle::try_from(value).map(Into::into),
            "border" => Border::try_from(value).map(Into::into),
            "transition" => Transition::try_from(value).map(Into::into),
            "shadow" => Shadow::try_from(value).map(Into::into),
            "gradient" => Gradient::try_from(value).map(Into::into),
            "typography" => Typography::try_from(value).map(Into::into),
            _ => Err(Error::UnexpectedType),
        }
        .map_err(|err| Error::kind(token_type, err))?;

        Ok(Self { value, description })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Color(Color),
    Dimension(Dimension),
    FontFamily(FontFamily),
    FontWeight(FontWeight),
    Duration(Duration),
    CubicBezier(CubicBezier),
    Number(Number),
    StrokeStyle(StrokeStyle),
    Border(Border),
    Transition(Transition),
    Shadow(Shadow),
    Gradient(Gradient),
    Typography(Typography),
    Alias(Alias),
}

impl From<Color> for Value {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl From<Dimension> for Value {
    fn from(value: Dimension) -> Self {
        Self::Dimension(value)
    }
}

impl From<FontFamily> for Value {
    fn from(value: FontFamily) -> Self {
        Self::FontFamily(value)
    }
}

impl From<FontWeight> for Value {
    fn from(value: FontWeight) -> Self {
        Self::FontWeight(value)
    }
}

impl From<Duration> for Value {
    fn from(value: Duration) -> Self {
        Self::Duration(value)
    }
}

impl From<CubicBezier> for Value {
    fn from(value: CubicBezier) -> Self {
        Self::CubicBezier(value)
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<StrokeStyle> for Value {
    fn from(value: StrokeStyle) -> Self {
        Self::StrokeStyle(value)
    }
}

impl From<Border> for Value {
    fn from(value: Border) -> Self {
        Self::Border(value)
    }
}

impl From<Transition> for Value {
    fn from(value: Transition) -> Self {
        Self::Transition(value)
    }
}

impl From<Gradient> for Value {
    fn from(value: Gradient) -> Self {
        Self::Gradient(value)
    }
}

impl From<Shadow> for Value {
    fn from(value: Shadow) -> Self {
        Self::Shadow(value)
    }
}

impl From<Typography> for Value {
    fn from(value: Typography) -> Self {
        Self::Typography(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::gradient::GradientStop;

    use super::*;
    use tinyjson::JsonValue::{Number, Object, String};

    #[test]
    fn test_token_from_map() {
        let test_cases = vec![
            (
                HashMap::from([
                    ("$type".to_string(), String("color".to_owned())),
                    ("$value".to_string(), String("#FF5733".to_owned())),
                    ("$description".to_string(), String("Red color".to_owned())),
                ]),
                None,
                Ok(Token {
                    value: Value::Color(Color {
                        r: 255,
                        g: 87,
                        b: 51,
                        a: 255,
                    }),
                    description: Some("Red color".to_owned()),
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("dimension".to_owned())),
                    ("$value".to_string(), String("16px".to_owned())),
                ]),
                None,
                Ok(Token {
                    value: Value::Dimension(Dimension::Pixels(16.0)),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("fontFamily".to_owned())),
                    ("$value".to_string(), String("Arial, sans-serif".to_owned())),
                ]),
                None,
                Ok(Token {
                    value: Value::FontFamily(FontFamily {
                        primary: "Arial, sans-serif".to_owned(),
                        fallbacks: vec![],
                    }),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("fontWeight".to_owned())),
                    ("$value".to_string(), String("bold".to_owned())),
                ]),
                None,
                Ok(Token {
                    value: Value::FontWeight(FontWeight::from_str("bold").unwrap()),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("duration".to_owned())),
                    ("$value".to_string(), String("500ms".to_owned())),
                ]),
                None,
                Ok(Token {
                    value: Value::Duration(Duration {
                        milliseconds: 500.0,
                    }),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("cubicBezier".to_owned())),
                    (
                        "$value".to_string(),
                        JsonValue::Array(vec![Number(0.0), Number(0.5), Number(1.0), Number(1.0)]),
                    ),
                ]),
                None,
                Ok(Token {
                    value: Value::CubicBezier(CubicBezier {
                        p1x: 0.0,
                        p1y: 0.5,
                        p2x: 1.0,
                        p2y: 1.0,
                    }),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("number".to_owned())),
                    ("$value".to_string(), Number(42.0)),
                ]),
                None,
                Ok(Token {
                    value: Value::Number(super::Number(42.0)),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("strokeStyle".to_owned())),
                    ("$value".to_string(), String("dotted".to_owned())),
                ]),
                None,
                Ok(Token {
                    value: Value::StrokeStyle(StrokeStyle::from_str("dotted").unwrap()),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("border".to_owned())),
                    (
                        "$value".to_string(),
                        Object(HashMap::from([
                            ("color".to_string(), String("#000000".to_owned())),
                            ("width".to_string(), String("2px".to_owned())),
                            ("style".to_string(), String("dashed".to_owned())),
                        ])),
                    ),
                ]),
                None,
                Ok(Token {
                    value: Value::Border(Border {
                        color: Color::from_hex("#000000").unwrap(),
                        width: Dimension::from_str("2px").unwrap(),
                        style: StrokeStyle::from_str("dashed").unwrap(),
                    }),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("transition".to_owned())),
                    (
                        "$value".to_string(),
                        Object(HashMap::from([
                            ("duration".to_string(), String("500ms".to_owned())),
                            ("delay".to_string(), String("100ms".to_owned())),
                            (
                                "timingFunction".to_string(),
                                JsonValue::Array(vec![
                                    Number(0.0),
                                    Number(0.5),
                                    Number(1.0),
                                    Number(1.0),
                                ]),
                            ),
                        ])),
                    ),
                ]),
                None,
                Ok(Token {
                    value: Value::Transition(Transition {
                        duration: Duration {
                            milliseconds: 500.0,
                        },
                        delay: Duration {
                            milliseconds: 100.0,
                        },
                        timing_function: CubicBezier {
                            p1x: 0.0,
                            p1y: 0.5,
                            p2x: 1.0,
                            p2y: 1.0,
                        },
                    }),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("shadow".to_owned())),
                    (
                        "$value".to_string(),
                        Object(HashMap::from([
                            ("color".to_string(), String("#000000".to_owned())),
                            ("offsetX".to_string(), String("2px".to_owned())),
                            ("offsetY".to_string(), String("2px".to_owned())),
                            ("blur".to_string(), String("5px".to_owned())),
                            ("spread".to_string(), String("0px".to_owned())),
                        ])),
                    ),
                ]),
                None,
                Ok(Token {
                    value: Value::Shadow(Shadow {
                        color: Color::from_hex("#000000").unwrap(),
                        offset_x: Dimension::from_str("2px").unwrap(),
                        offset_y: Dimension::from_str("2px").unwrap(),
                        blur: Dimension::from_str("5px").unwrap(),
                        spread: Dimension::from_str("0px").unwrap(),
                    }),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("gradient".to_owned())),
                    (
                        "$value".to_string(),
                        JsonValue::Array(vec![
                            Object(HashMap::from([
                                ("color".to_string(), String("#FF5733".to_owned())),
                                ("position".to_string(), Number(0.0)),
                            ])),
                            Object(HashMap::from([
                                ("color".to_string(), String("#00FF00".to_owned())),
                                ("position".to_string(), Number(1.0)),
                            ])),
                        ]),
                    ),
                ]),
                None,
                Ok(Token {
                    value: Value::Gradient(Gradient {
                        stops: vec![
                            GradientStop {
                                color: Color {
                                    r: 255,
                                    g: 87,
                                    b: 51,
                                    a: 255,
                                },
                                position: 0.0,
                            },
                            GradientStop {
                                color: Color {
                                    r: 0,
                                    g: 255,
                                    b: 0,
                                    a: 255,
                                },
                                position: 1.0,
                            },
                        ],
                    }),
                    description: None,
                }),
            ),
            (
                HashMap::from([
                    ("$type".to_string(), String("typography".to_owned())),
                    (
                        "$value".to_string(),
                        Object(HashMap::from([
                            (
                                "fontFamily".to_string(),
                                String("Arial, sans-serif".to_owned()),
                            ),
                            ("fontSize".to_string(), String("16px".to_owned())),
                            ("fontWeight".to_string(), String("bold".to_owned())),
                            ("letterSpacing".to_string(), String("1px".to_owned())),
                            ("lineHeight".to_string(), Number(1.5)),
                        ])),
                    ),
                ]),
                None,
                Ok(Token {
                    value: Value::Typography(Typography {
                        font_family: FontFamily {
                            primary: "Arial, sans-serif".to_owned(),
                            fallbacks: vec![],
                        },
                        font_size: Dimension::from_str("16px").unwrap(),
                        font_weight: FontWeight::from_str("bold").unwrap(),
                        letter_spacing: Dimension::from_str("1px").unwrap(),
                        line_height: 1.5,
                    }),
                    description: None,
                }),
            ),
            // Add test cases for tokens with a default type
            (
                HashMap::from([
                    ("$value".to_string(), String("#FF5733".to_owned())),
                    ("$description".to_string(), String("Red color".to_owned())),
                ]),
                Some("color".to_owned()),
                Ok(Token {
                    value: Value::Color(Color {
                        r: 255,
                        g: 87,
                        b: 51,
                        a: 255,
                    }),
                    description: Some("Red color".to_owned()),
                }),
            ),
            (
                HashMap::from([("$value".to_string(), String("16px".to_owned()))]),
                Some("dimension".to_owned()),
                Ok(Token {
                    value: Value::Dimension(Dimension::Pixels(16.0)),
                    description: None,
                }),
            ),
            (
                HashMap::from([("$value".to_string(), String("16".to_owned()))]),
                Some("dimension".to_owned()),
                Err(Error::kind(
                    "dimension".to_owned(),
                    Error::InvalidUnit(&["px", "rem"]),
                )),
            ),
        ];

        for (input, default_type, expected) in test_cases {
            let result = Token::from_map(&input, default_type);
            assert_eq!(result, expected);
        }
    }
}
