#![forbid(unsafe_code)]

use std::str::FromStr;

use rust_decimal::Decimal;
use thiserror::Error;

const MAX_INPUT_CHARS: usize = 4_096;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailRow {
    pub label: String,
    pub value: String,
    pub tone: DetailTone,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DetailTone {
    #[default]
    Neutral,
    Accent,
    Binary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluation {
    pub source: String,
    pub normalized: String,
    pub result: String,
    pub caption: String,
    pub details: Vec<DetailRow>,
    pub approximate: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("{message}")]
pub struct EngineError {
    pub code: &'static str,
    pub message: String,
}

pub struct CalculatorEngine {
    context: fend_core::Context,
}

impl Default for CalculatorEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CalculatorEngine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            context: fend_core::Context::new(),
        }
    }

    /// Evaluate user input locally. No exchange-rate handler is configured, so this
    /// adapter cannot initiate network traffic.
    ///
    /// # Errors
    ///
    /// Returns a stable [`EngineError`] for empty or oversized input, invalid
    /// expressions, unsupported conversions, and unavailable offline rates.
    pub fn evaluate(&mut self, source: &str) -> Result<Evaluation, EngineError> {
        let source = source.trim();
        if source.is_empty() {
            return Err(error("E_EMPTY", "Введите выражение или величину"));
        }
        if source.chars().count() > MAX_INPUT_CHARS {
            return Err(error(
                "E_LIMIT_EXCEEDED",
                "Слишком длинное выражение (максимум 4096 символов)",
            ));
        }

        let normalized = normalize_input(source);
        if let Some(data) = evaluate_data_quantity(source, &normalized)? {
            return Ok(data);
        }

        if normalized.is_empty() {
            return Err(error("E_EMPTY", "Введите выражение или величину"));
        }

        let evaluation_input = rewrite_contextual_percent(&normalized);
        let result = fend_core::evaluate(&evaluation_input, &mut self.context).map_err(|err| {
            let raw = err;
            let lower = raw.to_lowercase();
            if lower.contains("currency") || lower.contains("exchange rate") {
                error(
                    "E_OFFLINE_RATE",
                    "Курсы валют недоступны офлайн. Все остальные вычисления работают локально.",
                )
            } else if lower.contains("division by zero") || lower.contains("divide by zero") {
                error("E_DIV_ZERO", "Деление на ноль невозможно")
            } else {
                error("E_EVALUATION", localize_engine_error(&raw))
            }
        })?;

        let main = result.get_main_result().trim().to_owned();
        if main.is_empty() {
            return Err(error("E_NO_RESULT", "Выражение не вернуло результат"));
        }

        Ok(Evaluation {
            source: source.to_owned(),
            normalized,
            result: main,
            caption: String::new(),
            details: Vec::new(),
            approximate: false,
        })
    }
}

fn error(code: &'static str, message: impl Into<String>) -> EngineError {
    EngineError {
        code,
        message: message.into(),
    }
}

fn localize_engine_error(raw: &str) -> String {
    if raw.contains("unknown identifier") || raw.contains("Unknown identifier") {
        format!("Неизвестная функция, переменная или единица: {raw}")
    } else if raw.contains("expected") || raw.contains("Expected") {
        format!("Проверьте синтаксис выражения: {raw}")
    } else {
        format!("Не удалось вычислить: {raw}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DataDimension {
    Size,
    Rate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DataUnit {
    symbol: &'static str,
    bits_per_unit: Decimal,
    dimension: DataDimension,
    tone: DetailTone,
}

fn int(value: i64) -> Decimal {
    Decimal::from(value)
}

fn unit(
    symbol: &'static str,
    bits_per_unit: i64,
    dimension: DataDimension,
    tone: DetailTone,
) -> DataUnit {
    DataUnit {
        symbol,
        bits_per_unit: int(bits_per_unit),
        dimension,
        tone,
    }
}

fn data_unit(token: &str) -> Option<DataUnit> {
    let unit = match token.trim() {
        "bit" | "bits" => unit("bit", 1, DataDimension::Size, DetailTone::Neutral),
        "B" | "byte" | "bytes" => unit("byte", 8, DataDimension::Size, DetailTone::Neutral),
        "kbit" | "Kbit" => unit("kbit", 1_000, DataDimension::Size, DetailTone::Accent),
        "kB" | "KB" => unit("KB", 8_000, DataDimension::Size, DetailTone::Accent),
        "Mbit" | "Mb" | "mbit" => unit("Mbit", 1_000_000, DataDimension::Size, DetailTone::Accent),
        "MB" => unit("MB", 8_000_000, DataDimension::Size, DetailTone::Accent),
        "Gbit" | "Gb" | "gbit" => unit(
            "Gbit",
            1_000_000_000,
            DataDimension::Size,
            DetailTone::Accent,
        ),
        "GB" => unit("GB", 8_000_000_000, DataDimension::Size, DetailTone::Accent),
        "KiB" | "kib" => unit("KiB", 8 * 1_024, DataDimension::Size, DetailTone::Binary),
        "MiB" | "mib" => unit(
            "MiB",
            8 * 1_048_576,
            DataDimension::Size,
            DetailTone::Binary,
        ),
        "GiB" | "gib" => unit(
            "GiB",
            8_i64 * 1_073_741_824,
            DataDimension::Size,
            DetailTone::Binary,
        ),
        "bit/s" | "bitps" => unit("bit/s", 1, DataDimension::Rate, DetailTone::Neutral),
        "kbit/s" | "kbps" => unit("kbit/s", 1_000, DataDimension::Rate, DetailTone::Accent),
        "Mbit/s" | "Mbps" => unit("Mbit/s", 1_000_000, DataDimension::Rate, DetailTone::Accent),
        "Gbit/s" | "Gbps" => unit(
            "Gbit/s",
            1_000_000_000,
            DataDimension::Rate,
            DetailTone::Accent,
        ),
        "MB/s" => unit("MB/s", 8_000_000, DataDimension::Rate, DetailTone::Accent),
        "MiB/s" => unit(
            "MiB/s",
            8 * 1_048_576,
            DataDimension::Rate,
            DetailTone::Binary,
        ),
        _ => return None,
    };
    Some(unit)
}

fn evaluate_data_quantity(
    source: &str,
    normalized: &str,
) -> Result<Option<Evaluation>, EngineError> {
    let (left, target_token) = match normalized.split_once(" to ") {
        Some((left, target)) => (left.trim(), Some(target.trim())),
        None => (normalized.trim(), None),
    };
    let mut parts = left.split_whitespace();
    let Some(value_token) = parts.next() else {
        return Ok(None);
    };
    let Some(unit_token) = parts.next() else {
        return Ok(None);
    };
    if parts.next().is_some() {
        return Ok(None);
    }

    let Ok(value) = Decimal::from_str(value_token) else {
        return Ok(None);
    };
    let Some(from) = data_unit(unit_token) else {
        return Ok(None);
    };
    let bits = value * from.bits_per_unit;

    if let Some(target_token) = target_token {
        let Some(to) = data_unit(target_token) else {
            return Ok(None);
        };
        if from.dimension != to.dimension {
            return Err(error(
                "E_UNIT_INCOMPATIBLE",
                "Нельзя напрямую преобразовать объём данных в скорость передачи",
            ));
        }
        let converted = bits / to.bits_per_unit;
        return Ok(Some(Evaluation {
            source: source.to_owned(),
            normalized: normalized.to_owned(),
            result: format!("{} {}", format_decimal(converted, 12), to.symbol),
            caption: format!(
                "{} {} → {}",
                format_decimal(value, 12),
                from.symbol,
                to.symbol
            ),
            details: popular_data_rows(bits, from.dimension, Some(to.symbol)),
            approximate: converted.scale() > 6,
        }));
    }

    Ok(Some(Evaluation {
        source: source.to_owned(),
        normalized: normalized.to_owned(),
        result: format!("{} {}", format_decimal(value, 12), from.symbol),
        caption: match from.dimension {
            DataDimension::Size => "Популярные форматы данных".to_owned(),
            DataDimension::Rate => "Популярные форматы скорости".to_owned(),
        },
        details: popular_data_rows(bits, from.dimension, Some(from.symbol)),
        approximate: false,
    }))
}

fn popular_data_rows(
    bits: Decimal,
    dimension: DataDimension,
    excluded: Option<&str>,
) -> Vec<DetailRow> {
    let candidates: Vec<DataUnit> = match dimension {
        DataDimension::Size => vec![
            DataUnit {
                symbol: "Mbit",
                bits_per_unit: int(1_000_000),
                dimension: DataDimension::Size,
                tone: DetailTone::Accent,
            },
            DataUnit {
                symbol: "GB",
                bits_per_unit: int(8_000_000_000),
                dimension: DataDimension::Size,
                tone: DetailTone::Accent,
            },
            DataUnit {
                symbol: "MiB",
                bits_per_unit: int(8_388_608),
                dimension: DataDimension::Size,
                tone: DetailTone::Binary,
            },
            DataUnit {
                symbol: "bit",
                bits_per_unit: int(1),
                dimension: DataDimension::Size,
                tone: DetailTone::Neutral,
            },
        ],
        DataDimension::Rate => vec![
            DataUnit {
                symbol: "Mbit/s",
                bits_per_unit: int(1_000_000),
                dimension: DataDimension::Rate,
                tone: DetailTone::Accent,
            },
            DataUnit {
                symbol: "MB/s",
                bits_per_unit: int(8_000_000),
                dimension: DataDimension::Rate,
                tone: DetailTone::Accent,
            },
            DataUnit {
                symbol: "MiB/s",
                bits_per_unit: int(8_388_608),
                dimension: DataDimension::Rate,
                tone: DetailTone::Binary,
            },
            DataUnit {
                symbol: "bit/s",
                bits_per_unit: int(1),
                dimension: DataDimension::Rate,
                tone: DetailTone::Neutral,
            },
        ],
    };

    candidates
        .iter()
        .filter(|unit| excluded != Some(unit.symbol))
        .map(|unit| DetailRow {
            label: unit.symbol.to_owned(),
            value: format_decimal(bits / unit.bits_per_unit, 6),
            tone: unit.tone,
        })
        .collect()
}

fn format_decimal(value: Decimal, max_dp: u32) -> String {
    let rounded = value.round_dp(max_dp).normalize();
    rounded.to_string()
}

#[must_use]
pub fn normalize_input(source: &str) -> String {
    let source = source
        .trim_start()
        .strip_prefix('=')
        .unwrap_or(source.trim_start())
        .trim_start();
    let separated = separate_cyrillic_unit(source);
    let mut output = Vec::new();
    for token in separated.split_whitespace() {
        let lower = token.to_lowercase();
        let mapped = match lower.as_str() {
            "в" | "во" | "to" | "->" | "→" => "to",
            "мб" | "мбайт" | "мегабайт" | "мегабайта" | "мегабайтів" => {
                "MB"
            }
            "мбит" | "мегабит" | "мегабита" => "Mbit",
            "миб" | "мебибайт" | "мебибайта" => "MiB",
            "гб" | "гбайт" | "гигабайт" | "гигабайта" => "GB",
            "гбит" | "гигабит" | "гигабита" => "Gbit",
            "гиб" | "гибибайт" => "GiB",
            "кб" | "кбайт" | "килобайт" | "килобайта" => "KB",
            "кбит" | "килобит" | "килобита" => "kbit",
            "киб" | "кибибайт" => "KiB",
            "бит" | "бита" | "битов" => "bit",
            "байт" | "байта" | "байтов" => "byte",
            "мбит/с" | "мбит/сек" | "мбит/секунду" => "Mbit/s",
            "гбит/с" | "гбит/сек" => "Gbit/s",
            "мб/с" | "мбайт/с" => "MB/s",
            "миб/с" => "MiB/s",
            "км" | "километр" | "километра" | "километры" | "километров" => {
                "km"
            }
            "см" | "сантиметр" | "сантиметра" | "сантиметры" | "сантиметров" => {
                "cm"
            }
            "мм" | "миллиметр" | "миллиметра" | "миллиметры" | "миллиметров" => {
                "mm"
            }
            "метр" | "метра" | "метры" | "метров" => "m",
            "миля" | "мили" | "миль" => "miles",
            "фут" | "фута" | "футов" => "feet",
            "дюйм" | "дюйма" | "дюймов" => "inches",
            "кг" | "килограмм" | "килограмма" | "килограммы" | "килограммов" => {
                "kg"
            }
            "грамм" | "грамма" | "граммы" | "граммов" => "grams",
            "фунт" | "фунта" | "фунты" | "фунтов" => "pounds",
            "цельсий" | "цельсия" | "°c" => "degC",
            "фаренгейт" | "фаренгейта" | "°f" => "degF",
            "км/ч" => "km/h",
            "м/с" => "m/s",
            _ => token,
        };
        output.push(mapped.to_owned());
    }

    normalize_decimal_commas(&output.join(" "))
        .replace('×', "*")
        .replace('÷', "/")
        .replace('−', "-")
}

fn rewrite_contextual_percent(input: &str) -> String {
    let expression = input.trim();
    let Some(percent_operand) = expression.strip_suffix('%') else {
        return expression.to_owned();
    };

    let mut depth = 0_i32;
    let mut binary_sign = None;
    for (index, ch) in expression.char_indices() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            '+' | '-' if depth == 0 && is_binary_sign(expression, index) => {
                binary_sign = Some((index, ch));
            }
            _ => {}
        }
    }

    let Some((operator_index, operator)) = binary_sign else {
        return rewrite_fractional_percent(percent_operand);
    };
    let left = expression[..operator_index].trim();
    let right = percent_operand[operator_index + operator.len_utf8()..].trim();
    if left.is_empty() || right.is_empty() || right.ends_with('%') {
        return expression.to_owned();
    }

    format!("({left}) {operator} (({left}) * ({right}) / 100)")
}

fn rewrite_fractional_percent(percent_operand: &str) -> String {
    let operand_end = percent_operand.trim_end().len();
    let operand_text = &percent_operand[..operand_end];
    let operand_start = operand_text
        .char_indices()
        .rev()
        .find_map(|(index, ch)| {
            (!ch.is_ascii_digit() && ch != '.' && ch != ',').then_some(index + ch.len_utf8())
        })
        .unwrap_or(0);
    let operand = operand_text[operand_start..].trim();
    if operand.is_empty() {
        return format!("{percent_operand}%");
    }
    format!("{}(({operand}) / 100)", &percent_operand[..operand_start])
}

fn is_binary_sign(expression: &str, index: usize) -> bool {
    expression[..index]
        .trim_end()
        .chars()
        .next_back()
        .is_some_and(|previous| {
            !matches!(
                previous,
                '+' | '-' | '*' | '/' | '^' | '(' | ',' | 'e' | 'E'
            )
        })
}

fn normalize_decimal_commas(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    chars
        .iter()
        .enumerate()
        .map(|(index, ch)| {
            if *ch == ','
                && index > 0
                && index + 1 < chars.len()
                && chars[index - 1].is_ascii_digit()
                && chars[index + 1].is_ascii_digit()
            {
                '.'
            } else if *ch == ';' {
                ','
            } else {
                *ch
            }
        })
        .collect()
}

fn separate_cyrillic_unit(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + 8);
    let mut previous: Option<char> = None;
    for ch in input.chars() {
        if previous.is_some_and(|prev| {
            (prev.is_ascii_digit() || prev == '.' || prev == ',') && is_cyrillic(ch)
        }) {
            result.push(' ');
        }
        result.push(ch);
        previous = Some(ch);
    }
    result
}

fn is_cyrillic(ch: char) -> bool {
    matches!(ch, '\u{0400}'..='\u{04ff}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_arithmetic_with_precedence() {
        let mut engine = CalculatorEngine::new();
        let value = engine.evaluate("2 + 3 * 4").unwrap();
        assert_eq!(value.result, "14");
    }

    #[test]
    fn accepts_one_leading_equals_sign() {
        let mut engine = CalculatorEngine::new();
        assert_eq!(engine.evaluate("=5+5").unwrap().result, "10");
    }

    #[test]
    fn uses_contextual_percent_for_addition_and_subtraction() {
        let mut engine = CalculatorEngine::new();
        assert_eq!(engine.evaluate("100+50%").unwrap().result, "150");
        assert_eq!(engine.evaluate("200 - 10%").unwrap().result, "180");
        assert_eq!(engine.evaluate("100 + 20 + 10%").unwrap().result, "132");
    }

    #[test]
    fn keeps_standalone_and_multiplicative_percent_fractional() {
        let mut engine = CalculatorEngine::new();
        assert_eq!(engine.evaluate("10%").unwrap().result, "0.1");
        assert_eq!(engine.evaluate("200 * 10%").unwrap().result, "20");
        assert_eq!(engine.evaluate("200 / 10%").unwrap().result, "2000");
    }

    #[test]
    fn converts_decimal_megabytes_to_bits_exactly() {
        let mut engine = CalculatorEngine::new();
        let value = engine.evaluate("15 мб в бит").unwrap();
        assert_eq!(value.result, "120000000 bit");
    }

    #[test]
    fn bare_megabytes_include_popular_equivalents() {
        let mut engine = CalculatorEngine::new();
        let value = engine.evaluate("15мб").unwrap();
        assert_eq!(value.result, "15 MB");
        assert!(
            value
                .details
                .iter()
                .any(|row| row.label == "Mbit" && row.value == "120")
        );
        assert!(
            value
                .details
                .iter()
                .any(|row| row.label == "GB" && row.value == "0.015")
        );
        assert!(
            value
                .details
                .iter()
                .any(|row| row.label == "MiB" && row.value == "14.305115")
        );
    }

    #[test]
    fn megabits_are_not_megabytes() {
        let mut engine = CalculatorEngine::new();
        let value = engine.evaluate("15 мбит -> бит").unwrap();
        assert_eq!(value.result, "15000000 bit");
    }

    #[test]
    fn mebibytes_preserve_binary_value() {
        let mut engine = CalculatorEngine::new();
        let value = engine.evaluate("15 миб -> байт").unwrap();
        assert_eq!(value.result, "15728640 byte");
    }

    #[test]
    fn rate_does_not_emit_size_rows() {
        let mut engine = CalculatorEngine::new();
        let value = engine.evaluate("15 Мбит/с").unwrap();
        assert_eq!(value.result, "15 Mbit/s");
        assert!(value.details.iter().all(|row| row.label.ends_with("/s")));
    }

    #[test]
    fn converts_general_units_with_russian_aliases() {
        let mut engine = CalculatorEngine::new();
        let value = engine.evaluate("1 км в метры").unwrap();
        assert_eq!(value.result, "1000 m");
    }

    #[test]
    fn normalizes_decimal_comma() {
        assert_eq!(normalize_input("1,5 + 2,5"), "1.5 + 2.5");
    }
}
