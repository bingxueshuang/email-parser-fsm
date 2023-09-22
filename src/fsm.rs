//!
//! --- Transition diagram or transition table ---

use std::str::Chars;

/// FSM is an abstraction over behavior of deterministic finite automata. A DFA has a set of states
/// (generic type S) and alphabets (all possible symbols). One of them is a start state (start fn).
/// A transition takes the DFA from one state to other by consuming a symbol. If the input is
/// completely consumed and DFA is in a final state (or accepting state) then we say that the input
/// belongs to the language accepted by the DFA.
pub trait FSM<S> {
    type Symbol;
    fn transition(state: S, symbol: Self::Symbol) -> S;
    fn is_final(state: &S) -> bool;
    fn start() -> S;
}

/// The set of possible states in a DFA that represents a language accepting all valid email
/// addresses. [State::Error] is a dead state (or trap state).
#[derive(Clone, Debug, Copy)]
pub enum State {
    AddrSpec,
    LocalAtom,
    LocalQText,
    LocalDot,
    LocalEscape,
    LocalQString,
    LocalPart,
    DomainAtom,
    DomainDText,
    DomainDot,
    DomainLiteral,
    Error,
}

/// Certain symbols and group of symbols useful for determining transition rules.
impl State {
    const DQUOTE: char = '"';
    const DOT: char = '.';
    const BACKSLASH: char = '\\';
    const AT: char = '@';
    const OPEN_BRACKET: char = '[';
    const CLOSE_BRACKET: char = ']';
    fn is_atext(c: char) -> bool {
        let n: u32 = c.into();
        c == '!'
            || c == '#'
            || c == '$'
            || c == '%'
            || c == '&'
            || c == '\''
            || c == '*'
            || c == '+'
            || c == '-'
            || c == '/'
            || c == '='
            || c == '?'
            || c == '^'
            || c == '_'
            || c == '`'
            || c == '{'
            || c == '}'
            || c == '~'
            || (0x41 <= n && n <= 0x5A) // A-Z
            || (0x61 <= n && n <= 0x7A) // a-z
            || (0x30 <= n && n <= 0x39) // 0-9
    }
    fn is_qtext(c: char) -> bool {
        let n: u32 = c.into();
        n == 33 || (35 <= n && n <= 91) || (93 <= n && n <= 126)
    }
    fn is_dtext(c: char) -> bool {
        let n: u32 = c.into();
        (33 <= n && n <= 90) && (94 <= n && n <= 126)
    }
    fn is_escape(c: char) -> bool {
        let n: u32 = c.into();
        (0x21 <= n && n <= 0x7E) // VCHAR
            || n == 0x20 // SPACE
            || n == 0x09 // HTAB
    }
}

/// State implements FSM and defines a DFA for language accepting all valid email addresses.
/// The set of states in DFA is itself. All transitions are defined in the implementation itself.
/// Start state of DFA is [State::AddrSpec].
impl FSM<State> for State {
    type Symbol = char;
    fn transition(state: Self, c: char) -> State {
        match state {
            Self::AddrSpec => match c {
                Self::DQUOTE => State::LocalQText,
                c if Self::is_atext(c) => State::LocalAtom,
                _ => State::Error,
            },
            Self::LocalAtom => match c {
                Self::DOT => State::LocalDot,
                Self::AT => State::LocalPart,
                c if Self::is_atext(c) => State::LocalAtom,
                _ => State::Error,
            },
            Self::LocalQText => match c {
                Self::BACKSLASH => State::LocalEscape,
                Self::DQUOTE => State::LocalQString,
                c if Self::is_qtext(c) => State::LocalQText,
                _ => State::Error,
            },
            Self::LocalDot => match c {
                c if Self::is_atext(c) => State::LocalAtom,
                _ => State::Error,
            },
            Self::LocalEscape => match c {
                c if Self::is_escape(c) => State::LocalQText,
                _ => State::Error,
            },
            Self::LocalQString => match c {
                Self::AT => State::LocalPart,
                _ => State::Error,
            },
            Self::LocalPart => match c {
                Self::OPEN_BRACKET => State::DomainDText,
                c if Self::is_atext(c) => State::DomainAtom,
                _ => Self::Error,
            },
            Self::DomainAtom => match c {
                Self::DOT => State::DomainDot,
                c if Self::is_atext(c) => State::DomainAtom,
                _ => Self::Error,
            },
            Self::DomainDText => match c {
                Self::CLOSE_BRACKET => State::DomainLiteral,
                c if Self::is_dtext(c) => State::DomainDText,
                _ => Self::Error,
            },
            Self::DomainDot => match c {
                c if Self::is_atext(c) => State::DomainAtom,
                _ => Self::Error,
            },
            Self::DomainLiteral => match c {
                _ => Self::Error,
            },
            Self::Error => Self::Error,
        }
    }
    fn is_final(state: &Self) -> bool {
        match state {
            Self::DomainLiteral | Self::DomainAtom => true,
            _ => false,
        }
    }
    fn start() -> Self {
        State::AddrSpec
    }
}

/// Iterator for the DFA implemented by [Machine]. Each step through the iterator consumes
/// an input symbol (from input iterator) and transitions the DFA through the corresponding state.
/// The iterator is exhausted when the input iterator gets exhausted. If any invalid character or
/// invalid email address syntax is encountered, then transition is to [State::Error]. For every
/// input symbol, this state consumes the symbol and remains in [State::Error]. Thus, parse errors
/// can be checked just by investigating the state of DFA when input gets exhausted.
///
/// Examining the last state of DFA can be done via [Iterator::last] method. If it is [None], then
/// empty input and parse failed. Otherwise, there will be at least one transition and hence we get
/// some last state. By checking if it is an accepting state, parsing success can be determined.
pub struct MachineIterator<'a> {
    input: Chars<'a>,
    state: State,
}

/// MachineIterator just wraps over input iterator and performs transitions at every step.
/// It keeps track of current state as well. Thus, next state is determined using current state as
/// well as the input symbol based on the transition rules defined.
impl<'a> Iterator for MachineIterator<'a> {
    type Item = State;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.input.next()?;
        self.state = State::transition(self.state, c);
        Some(self.state)
    }
}

/// Machine is the core export of the module. It is an [IntoIterator] and consuming the iterator
/// determines if given string literal is a valid email address or not.
pub struct Machine<'a> {
    input: &'a str,
}

impl<'a> Machine<'a> {
    pub fn new(s: &'a str) -> Self {
        Machine { input: s }
    }
}

impl<'a> IntoIterator for Machine<'a> {
    type Item = State;
    type IntoIter = MachineIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MachineIterator {
            state: State::AddrSpec,
            input: self.input.chars(),
        }
    }
}
