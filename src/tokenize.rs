use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TokenField {
  Token(String),
  Field(Vec<TokenField>),
  StringToken(String)
}

pub fn expressionize(stream_arg: String) -> TokenField {
  fn match_expr(stream_arg: String, match_delim: String)
  -> (TokenField, String) {
    let mut expr_vec = Vec::new();
    let mut stream = stream_arg.to_string();
    while !stream.is_empty() {
      let (tok, substream) = gettok(&stream);
      if tok_is_delimiter(&tok) {
        if tok == match_delim {
          stream = substream;
          break;
        }
        else if tok == "\"" {
          let (tok, substream) = get_string_tok(&substream);
          expr_vec.push(TokenField::StringToken(tok.to_string()));
          stream = substream;
        }
        else {
          let subexpr_match_delim = match_delimiter(&tok);
          let (subexpr_field, substream) =
            match_expr(substream,subexpr_match_delim);
          stream = substream;
          expr_vec.push(subexpr_field);
        }
      }
      else {
        expr_vec.push(TokenField::Token(tok.to_string()));
        stream = substream;
      }    
    }
    return (TokenField::Field(expr_vec), stream.to_string());
  }
  let (tok, _) = gettok(&stream_arg);
  /* if first token is a delimiter, get its match,
     otherwise, match itself */
  let subexpr_match_delim = match_delimiter(&tok);
  let (token_field, _) = match_expr(stream_arg, subexpr_match_delim);
  return token_field;
}

fn char_is_delimiter(c: char) -> bool {
  let mut delims = HashSet::new();
  for d in "()[]\"".chars() {
    delims.insert(d);
  }
  return delims.contains(&c);
}

fn tok_is_delimiter(s: &String) -> bool {
  return (s.len() == 1) &&
    char_is_delimiter(s.chars().next().unwrap());
}

fn match_delimiter(d: &String) -> String {
  let delim_pairs = [("(",")"), ("[","]"), ("\"", "\"")];
  let mut delim_map = HashMap::new();
  
  for pair in delim_pairs.iter() {
    let (car, cdr) = pair;
    delim_map.insert(car.to_string(), cdr.to_string());
  }
  let matched = match delim_map.get(d) {
    Some(m) => m.to_string(),
    // this will cause a top-level token
    // to match itself
    _ => d.to_string()
  };
  return matched;
}

fn get_string_tok(stream: &String) -> (String, String) {
  let tok_end_index = stream.find('"').unwrap();
  let tok = stream[..tok_end_index].to_string();
  let tok_len = tok.len();
  return(tok.to_string(), stream[(tok_len + 1)..].to_string());
}

fn gettok(stream_arg: &String) -> (String, String) {
  let stream = stream_arg.trim_start();
  let mut tok = "".to_string();
  for c in stream.chars() {
    if char_is_delimiter(c) {
      if tok.is_empty() {
        tok.push(c)
      }
      break;
    }
    else if c.is_whitespace() {
      break;
    }
    else {
      tok.push(c)
    }
  }
  let tok_len = tok.len();
  return (tok.to_string(), stream[tok_len..].to_string());
}

