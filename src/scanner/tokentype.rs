#[derive(Debug)] 
pub enum TokenType {
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  Identifier, String, Number,

  And, Class, Else, False, Fun, For, If, Null, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}
