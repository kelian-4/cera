use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(map) => map.get(key),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn as_string(&self, fallback: &str) -> String {
        match self {
            Value::String(s) => s.clone(),
            _ => fallback.to_string(),
        }
    }

    pub fn as_number(&self, fallback: f64) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => fallback,
        }
    }

    pub fn as_i64(&self, fallback: i64) -> i64 {
        match self {
            Value::Number(n) => *n as i64,
            _ => fallback,
        }
    }

    pub fn as_bool(&self, fallback: bool) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => fallback,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }
}

pub struct ParseResult {
    pub value: Option<Value>,
    pub ok: bool,
    pub error: String,
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(text: &str) -> Self {
        Parser {
            chars: text.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> char {
        *self.chars.get(self.pos).unwrap_or(&'\0')
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        if self.pos < self.chars.len() {
            self.pos += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, c: char) -> Result<(), String> {
        if self.peek() != c {
            return Err(format!("attendu '{}'", c));
        }
        self.pos += 1;
        Ok(())
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.peek() {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string().map(Value::String),
            't' | 'f' => self.parse_bool(),
            'n' => self.parse_null(),
            c if c == '-' || c.is_ascii_digit() => self.parse_number(),
            _ => Err("valeur JSON inattendue".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<Value, String> {
        let mut map = BTreeMap::new();
        self.expect('{')?;
        self.skip_whitespace();
        if self.peek() == '}' {
            self.pos += 1;
            return Ok(Value::Object(map));
        }
        loop {
            self.skip_whitespace();
            if self.peek() != '"' {
                return Err("cle attendue dans l'objet".to_string());
            }
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(':')?;
            let val = self.parse_value()?;
            map.insert(key, val);
            self.skip_whitespace();
            match self.advance() {
                ',' => continue,
                '}' => break,
                _ => return Err("',' ou '}' attendu".to_string()),
            }
        }
        Ok(Value::Object(map))
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        let mut arr = Vec::new();
        self.expect('[')?;
        self.skip_whitespace();
        if self.peek() == ']' {
            self.pos += 1;
            return Ok(Value::Array(arr));
        }
        loop {
            let val = self.parse_value()?;
            arr.push(val);
            self.skip_whitespace();
            match self.advance() {
                ',' => continue,
                ']' => break,
                _ => return Err("',' ou ']' attendu".to_string()),
            }
        }
        Ok(Value::Array(arr))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect('"')?;
        let mut out = String::new();
        loop {
            if self.pos >= self.chars.len() {
                return Err("chaine non terminee".to_string());
            }
            let c = self.advance();
            if c == '"' {
                break;
            }
            if c == '\\' {
                let esc = self.advance();
                match esc {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    '/' => out.push('/'),
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    'b' => out.push('\u{8}'),
                    'f' => out.push('\u{c}'),
                    'u' => {
                        if self.pos + 4 > self.chars.len() {
                            return Err("sequence \\u incomplete".to_string());
                        }
                        let hex: String = self.chars[self.pos..self.pos + 4].iter().collect();
                        self.pos += 4;
                        let cp = u32::from_str_radix(&hex, 16)
                            .map_err(|_| "sequence \\u invalide".to_string())?;
                        if let Some(ch) = char::from_u32(cp) {
                            out.push(ch);
                        }
                    }
                    _ => return Err("sequence d'echappement invalide".to_string()),
                }
            } else {
                out.push(c);
            }
        }
        Ok(out)
    }

    fn parse_bool(&mut self) -> Result<Value, String> {
        let rest: String = self.chars[self.pos..].iter().collect();
        if rest.starts_with("true") {
            self.pos += 4;
            return Ok(Value::Bool(true));
        }
        if rest.starts_with("false") {
            self.pos += 5;
            return Ok(Value::Bool(false));
        }
        Err("litteral booleen invalide".to_string())
    }

    fn parse_null(&mut self) -> Result<Value, String> {
        let rest: String = self.chars[self.pos..].iter().collect();
        if rest.starts_with("null") {
            self.pos += 4;
            return Ok(Value::Null);
        }
        Err("litteral 'null' invalide".to_string())
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        if self.peek() == '-' {
            self.pos += 1;
        }
        while self.peek().is_ascii_digit() {
            self.pos += 1;
        }
        if self.peek() == '.' {
            self.pos += 1;
            while self.peek().is_ascii_digit() {
                self.pos += 1;
            }
        }
        if self.peek() == 'e' || self.peek() == 'E' {
            self.pos += 1;
            if self.peek() == '+' || self.peek() == '-' {
                self.pos += 1;
            }
            while self.peek().is_ascii_digit() {
                self.pos += 1;
            }
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        s.parse::<f64>()
            .map(Value::Number)
            .map_err(|_| "nombre invalide".to_string())
    }

    fn run(&mut self) -> ParseResult {
        self.skip_whitespace();
        match self.parse_value() {
            Ok(v) => {
                self.skip_whitespace();
                if self.pos != self.chars.len() {
                    return ParseResult {
                        value: None,
                        ok: false,
                        error: "caracteres en trop apres la valeur JSON".to_string(),
                    };
                }
                ParseResult {
                    value: Some(v),
                    ok: true,
                    error: String::new(),
                }
            }
            Err(e) => ParseResult {
                value: None,
                ok: false,
                error: e,
            },
        }
    }
}

pub fn parse(text: &str) -> ParseResult {
    let mut parser = Parser::new(text);
    parser.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parsing() {
        let text = r#"{"a": 1, "b": "text", "c": [1,2,3], "d": true}"#;
        let result = parse(text);
        assert!(result.ok);
        let root = result.value.unwrap();
        assert_eq!(root.get("a").unwrap().as_i64(0), 1);
        assert_eq!(root.get("b").unwrap().as_string(""), "text");
        assert_eq!(root.get("c").unwrap().as_array().unwrap().len(), 3);
        assert!(root.get("d").unwrap().as_bool(false));
    }

    #[test]
    fn test_json_invalid() {
        let result = parse("not json");
        assert!(!result.ok);
    }

    #[test]
    fn test_json_nested_object() {
        let text = r##"{"theme": {"keyword1": "#ff0000"}}"##;
        let result = parse(text);
        assert!(result.ok);
        let root = result.value.unwrap();
        let theme = root.get("theme").unwrap();
        assert_eq!(theme.get("keyword1").unwrap().as_string(""), "#ff0000");
    }
}
