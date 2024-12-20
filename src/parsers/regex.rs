use regex::Regex;

use crate::{Context, ContextParserT, Failure, ParserType, StringParserT, Success};

/// Parses for a given regex pattern
/// ## Example
/// ```
/// use microparsec::{RegexParser, ContextParserT, StringParserT};
///
/// let number_parser = RegexParser::new(r"\+\d{2}\s\d{3}\s\d{5}", "Phone number");
/// let res = number_parser.parse("+12 345 67890");
/// assert_eq!(res.unwrap().val, "+12 345 67890");
///
/// let res = number_parser.parse("+12 45 6890");
/// assert_eq!(
///     res.unwrap_err().get_error_message(),
///     "[Parser error] Expected `Phone number` at position: 0"
/// );
/// ```
#[derive(Clone)]
pub struct RegexParser {
    regex: String,
    generic_error: String,
}

impl RegexParser {
    pub fn new<A: AsRef<str>, B: AsRef<str>>(regex: A, expected: B) -> Self {
        let regex = regex.as_ref().to_string();
        let generic_error = expected.as_ref().to_string();

        RegexParser {
            regex,
            generic_error,
        }
    }
}

impl ContextParserT<String> for RegexParser {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Regex
    }

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<String>, Failure> {
        let regex = match Regex::new(&self.regex) {
            Ok(regex) => regex,
            Err(_) => panic!("Invalid regex: {}", self.regex),
        };

        let sliced_ctx = &ctx.txt[ctx.pos..];
        let mat = regex.find(sliced_ctx);
        if let Some(mat) = mat {
            if mat.start() == 0 {
                ctx.pos += mat.end();
                return Ok(Success::new(mat.as_str().to_string(), ctx));
            }
        }

        Err(Failure::new(
            &self.generic_error,
            ctx,
            vec![ParserType::Regex],
        ))
    }
}

impl StringParserT<String> for RegexParser {}
