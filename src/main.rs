use std::fmt;

mod tokenize;
use tokenize::expressionize;
use tokenize::TokenField;

/* expr : appl | num
   appl : fun num num
   num: INT
   fun: +, - */

macro_rules! make_token_field_inspector{
  ($($v:expr),+)  => {{
    let mut m = ::std::collections::HashMap::new();
    for (i, item) in [$($v,)+].iter().enumerate() {
      m.insert(item, i);
    }
    let inspect_token_field = move |tok_field: &TokenField, prop: &str|
    -> TokenField {
      let index = *(m.get(&prop).unwrap()) as usize;
      let field_vec = match tok_field {
        TokenField::Field(z) => z,
        _ => panic!("expected token field")
      };
      if field_vec.len() <= index {
        panic!(format!("inspect_token_field: invalid field id ({}, {})\nfields: {:#?}", prop.to_string(), index, field_vec));
      }
      return field_vec[index].clone();
    };
    inspect_token_field
  }}
}

#[derive(Debug)]
struct IRLang {
  name: TokenField,
  terminals: TokenField,
  non_terminals: TokenField
}

fn make_ir_lang(token_field: TokenField) -> IRLang {
  let get_field = make_token_field_inspector!("form", "id", "terminals", "nonterminals");
  let name = get_field(&token_field, "id");
  let terminals = get_field(&token_field, "terminals");
  let non_terminals = get_field(&token_field, "nonterminals");
  return IRLang {
    name: name,
    terminals: terminals,
    non_terminals: non_terminals
  }
}

trait Predicate {
  fn predicate(&self);
}

impl fmt::Debug for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Predicate>")
    }
}

#[derive(Debug)]
struct _SyntaxTerminal {
  name: TokenField,
  predicate: Predicate,
}

fn main() {
  let _lang_def = "
    (define-lang plus-minus
      [(integer? x)
       (primative? pr)]
      [expr ()
          (pr x0 x1)])";

  let _syntax_class = "
    (define-teriminal primitive
      (λ (x) (set-contains? (set + -)) x))";

  let _with_string = "(define x \"hello there friend!\")";
  let _one_thing = "52 (define hello 1)";
  let _get_field = make_token_field_inspector!("def");

  let _expr = expressionize(_lang_def.to_string());
  let _def = make_ir_lang(_get_field(&_expr, "def"));
  println!("{:#?}", _expr);
}
