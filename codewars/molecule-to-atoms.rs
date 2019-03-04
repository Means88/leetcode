use std::result::Result::{Err, Ok};
use std::collections::HashMap;

fn main() {
  // println!("{:?}", parse_molecule("K4[ON(SO3)2]2"));
  println!("{:?}", parse_molecule("{[Co(NH3)4(OH)2]3Co}(SO4)3"));
}

/**
 * Digits := [1-9][0-9]*
 * Atom := [A-Z][a-z]*
 * Atoms := Atom Digits?
 * AtomGroups
 *  := (SubMolecule+) Digits
 *   | [SubMolecule+] Digits
 *   | {SubMolecule+} Digits
 * SubMolecule := Atoms | AtomGroups
 * Molecule := SubMolecule+
 */

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

#[derive(Debug)]
struct ParseResult<'a, T> {
  rest: &'a str,
  result: T,
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
  let mut map: HashMap<String, usize> = HashMap::new();
  let mut rest = s;

  while rest.len() > 0 {
    let submolecule_result = consume_submolecule(rest);
    if submolecule_result.is_err() {
      return Err(submolecule_result.unwrap_err());
    }
    let submolecule = submolecule_result.unwrap();
    rest = submolecule.rest;
    // println!("{:?}", submolecule.result);
    for (atom, count) in submolecule.result {
      *map.entry(atom).or_insert(0) += count;
    }
  }

  let mut result: Vec<Atom> = Vec::new();
  for (k, v) in map.iter() {
    result.push((k.to_string(), *v));
  }

  Ok(result)
}

fn consume_submolecule<'a>(s: &'a str) -> Result<ParseResult<'a, Molecule>, ParseError> {
  if s.len() == 0 {
    return Err(ParseError{});
  }
  let first = s.chars().nth(0).unwrap();
  if first == '(' || first == '[' || first == '{' {
    return consume_atom_groups(s);
  }

  let mut map: HashMap<String, usize> = HashMap::new();
  let mut rest = s;

  if rest.len() == 0 || !rest.chars().nth(0).unwrap().is_uppercase() {
    return Err(ParseError{});
  }

  while rest.len() > 0 && rest.chars().nth(0).unwrap().is_uppercase() {
    let atoms_result = consume_atoms(rest);
    // println!("atoms result {:?}", atoms_result);
    if atoms_result.is_err() {
      return Err(atoms_result.unwrap_err());
    }
    let parse_result = atoms_result.unwrap();
    rest = parse_result.rest;
    let result = parse_result.result;

    *map.entry(result.0).or_insert(0) += result.1;
  }

  let mut result: Vec<Atom> = Vec::new();
  for (k, v) in map.iter() {
    result.push((k.to_string(), *v));
  }

  Ok(ParseResult {
    rest,
    result,
  })
}

fn consume_atom_groups<'a>(s: &'a str) -> Result<ParseResult<'a, Molecule>, ParseError> {
  // println!("atom groups {}", s);
  let mut map: HashMap<String, usize> = HashMap::new();
  let mut rest = s;

  if s.len() == 0 {
    return Err(ParseError{});
  }
  let start = s.chars().nth(0).unwrap();
  let end: char;
  if start == '(' {
    end = ')';
  } else if start == '[' {
    end = ']';
  } else if start == '{' {
    end = '}';
  } else {
    return Err(ParseError{});
  }

  rest = &rest[1..];

  while rest.len() > 0 {
    if rest.chars().nth(0).unwrap() == end {
      break;
    }

    let submolecule_result = consume_submolecule(&rest);
    if submolecule_result.is_err() {
      return Err(submolecule_result.unwrap_err());
    }
    let submolecule = submolecule_result.unwrap();
    rest = submolecule.rest;

    for (atom, count) in submolecule.result {
      *map.entry(atom).or_insert(0) += count;
    }
  }

  if rest.len() == 0 {
    return Err(ParseError{});
  }
  if rest.chars().nth(0).unwrap() != end {
    return Err(ParseError{});
  }
  rest = &rest[1..];

  let mut multiple = 1;
  let digits_result = consume_digits(&rest);
  if !digits_result.is_err() {
    let digits_result = digits_result.unwrap();
    rest = digits_result.rest;
    multiple = digits_result.result;
  }

  let mut result: Vec<Atom> = Vec::new();
  for (k, v) in map.iter() {
    result.push((k.to_string(), *v * multiple));
  }

  Ok(ParseResult {
    rest,
    result,
  })
}

fn consume_atoms<'a>(s: &'a str) -> Result<ParseResult<'a, Atom>, ParseError> {
  // println!("atoms {}", s);
  let atom_result = consume_atom(s);
  if atom_result.is_err() {
    return Err(atom_result.unwrap_err());
  }

  let ParseResult {
    rest: atom_rest,
    result: atom,
  } = atom_result.unwrap();

  if atom_rest.len() == 0 || !atom_rest.chars().nth(0).unwrap().is_digit(10) {
    return Ok(ParseResult {
      rest: atom_rest,
      result: (atom, 1),
    });
  }

  let digits_result = consume_digits(&atom_rest);
  if digits_result.is_err() {
    return Err(digits_result.unwrap_err());
  }
  let ParseResult {
    rest: digits_rest,
    result: digits,
  } = digits_result.unwrap();

  Ok(ParseResult {
    rest: digits_rest,
    result: (atom, digits),
  })
}

fn consume_digits<'a>(s: &'a str) -> Result<ParseResult<'a, usize>, ParseError> {
  let mut digits_str = String::new();

  let mut backup =  s;
  let mut split = head(s);
  if split.0 == "0" || !is_digit(split.0) {
    return Err(ParseError{});
  }
  while is_digit(split.0) {
    digits_str.push_str(&split.0);
    backup = split.1;
    split = head(split.1);
  }
  let result = digits_str.parse::<usize>();
  if result.is_err() {
    return Err(ParseError{});
  }

  Ok(ParseResult {
    rest: backup,
    result: result.unwrap(),
  })
}

fn consume_atom<'a>(s: &'a str) -> Result<ParseResult<'a, String>, ParseError> {
  let mut atom_str = String::new();

  let mut split = head(s);
  if !is_uppercase(split.0) {
    return Err(ParseError{});
  }
  atom_str.push_str(split.0);
  let mut backup = split.1;
  split = head(split.1);
  while is_lowercase(split.0) {
    atom_str.push_str(split.0);
    backup = split.1;
    split = head(split.1);
  }

  Ok(ParseResult {
    rest: backup,
    result: atom_str,
  })
}

fn head<'a>(s: &'a str) -> (&'a str, &'a str) {
  if s.len() == 0 {
    return ("", "")
  }
  (&s[..1], &s[1..])
}

fn is_digit(s: &str) -> bool {
  is_single_char(s, &|x: &char| x.is_digit(10))
}

fn is_uppercase(s: &str) -> bool {
  is_single_char(s, &|x: &char| x.is_uppercase())
}

fn is_lowercase(s: &str) -> bool {
  is_single_char(s, &|x: &char| x.is_lowercase())
}

fn is_single_char(s: &str, f: &Fn(&char) -> bool) -> bool {
  if s.len() != 1 {
    return false;
  }
  let ch = s.chars().nth(0);
  if ch.is_none() {
    return false;
  }
  f(&ch.unwrap())
}
