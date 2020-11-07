use crate::types::*;

pub struct TextParser {
    tag: char
}

impl TokenParser for TextParser {
    fn parse(&mut self, tokinizer: &mut Tokinizer<'_>) -> Result<BramaTokenType, (String, u32, u32)> {
        let mut ch: char      = '\0';
        let mut ch_next: char;
        let mut symbol        = String::new();

        tokinizer.increase_index();

        while !tokinizer.is_end() {
            ch      = tokinizer.get_char();
            ch_next = tokinizer.get_next_char();

            if ch == '\\' && ch_next == self.tag {
                symbol.push(ch);
                tokinizer.increase_index();
            }
            else if ch == self.tag {
                tokinizer.increase_index();
                break;
            }
            else {
                symbol.push(ch);
            }

            tokinizer.increase_index();
        }

        if ch != self.tag {
            return Err((String::from("Missing string deliminator"), tokinizer.line, tokinizer.column));
        }

        return Ok(BramaTokenType::Text(symbol.to_owned()));
    }

    fn validate(&mut self, tokinizer: &mut Tokinizer<'_>) -> BramaStatus {
        BramaStatus::Ok
    }
}