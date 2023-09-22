//! ## Grammar
//! Parser for email address (`addr-spec`) as defined in Section 3.4.1 of [`RFC5322`].
//! This crate implements only a subset of the grammar and does not support folding white space
//! and comments in email address. Also, the grammar rules that are defined to preserve backwards
//! compatibility are not supported. The grammar implemented is described below:
//!
//! ```text
//! ; non-terminals
//! addr-spec        =  local-part AT domain
//! local-part       =  dot-atom / quoted-string
//! domain           =  dot-atom / domain-literal
//! domain-literal   =  OPEN_BRACKET *DTEXT CLOSE_BRACKET
//! atom             =  1*ATEXT
//! dot-atom-literal =  DOT atom
//! dot-atom         =  atom *dot-atom-literal
//! quoted-pair      =  BACKSLASH ESCAPE
//! qcontent         =  QTEXT / quoted-pair
//! quoted-string    =  DQUOTE *qcontent DQUOTE
//!
//! ; terminals
//! AT               =  "@"  ; @ character
//! OPEN_BRACKET     =  "["  ; square bracket open
//! CLOSE_BRACKET    =  "]"  ; square bracket close
//! DTEXT            =  %d33-90 / %d94-126
//!                          ; Printable US-ASCII characters not
//!                          ;  including "[", "]" or "\".
//! ATEXT            =  ALPHA / DIGIT / "!" /
//!                     "#" / "$" / "%" /
//!                     "&" / "'" / "*" /
//!                     "+" / "-" / "/" /
//!                     "=" / "?" / "^" /
//!                     "_" / "`" / "{" /
//!                     "|" / "}" / "~"
//!                          ; Printable US-ASCII characters not
//!                          ;  including specials. Used for atoms.
//! SPECIALS         =  "(" / ")" / "<" / ">" / "@" /
//!                     "[" / "]" / ":" / ";" / "\" /
//!                     "," / "." / DQUOTE
//!                          ; Special characters that do not appear in
//!                          ;  atext. Useful for tools that perform
//!                          ;  lexical analysis: each character in
//!                          ;  specials can be used to indicate a
//!                          ;  tokenization point in lexical analysis.
//! BACKSLASH        =  "\"  ; \ (backslash)
//! DOT              =  "."  ; . (dot)
//! ESCAPE           =  VCHAR / WSP
//! QTEXT            =  %d33 / %d35-91 / %d93-126
//!                          ; Printable US-ASCII characters not
//!                          ;  including "\" or the quote character.
//! DQUOTE           =  %x22 ; " (Double Quote)
//!
//! ; inline
//! VCHAR            =  %x21-7E             ; visible (printing) characters
//! WSP              =  SP / HTAB           ; white space
//! HTAB             =  %x09                ; horizontal tab
//! SP               =  %x20                ; space character
//! ALPHA            =  %x41-5A / %x61-7A   ; A-Z / a-z
//! DIGIT            =  %x30-39             ; 0-9
//! ```
//!
//! [`RFC5322`]: https://datatracker.ietf.org/doc/html/rfc5322#section-3.4.1
//!
//! ## Finite State Machine
//!
//! The above grammar defines a Regular language. So, we do not need to construct a lexer and
//! a parser. Email address as defined above can be parsed using finite automaton (or regular
//! expressions also will do). In this crate, we construct a finite state machine (module fsm)
//! and parse the given string into email address or fail and emit errors.
//!
//! ```
//! use email_parser::Email;
//! let email: Email = "someone@example.com".parse().unwrap();
//! ```

use crate::fsm::{State, FSM};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// Email parsing errors.
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("cannot parse empty email id")]
    EmptyEmail,
    #[error("invalid RFC5322 formatted email id")]
    InvalidEmail,
}

/// Email parsing is accomplished using a finite state machine. FSM is defined in this module.
/// Finite automaton has several states and transitions. When iterator is completely consumed, if
/// the state is a final state, then given string is valid email address.
mod fsm;

/// This is the core of the crate. Defines email address type which can be constructed by parsing a
/// string literal. As long as it is constructed properly, then it means the email address is valid.
pub struct Email {
    local: String,
    domain: String,
}

/// Support parsing from string literal.
impl FromStr for Email {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = fsm::Machine::new(s);
        let ref state = m.into_iter().last().ok_or(Error::EmptyEmail)?;
        let (one, two) = State::is_final(state)
            .then(|| s.split_once('@').unwrap())
            .ok_or(Error::InvalidEmail)?;
        Ok(Self {
            local: one.to_owned(),
            domain: two.to_owned(),
        })
    }
}

/// Support formatted output.
impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}@{}", self.local, self.domain)
    }
}
