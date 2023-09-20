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

mod terminal {
    pub struct At;
    pub struct OpenBracket;
    pub struct CloseBracket;
    pub struct DText(char);
    pub struct AText(char);
    pub struct Specials(char);
    pub struct Backslash;
    pub struct Dot;
    pub struct Escape(char);
    pub struct QText(char);
    pub struct DQuote;
}

mod non_terminal {
    use super::terminal;
    pub struct AddrSpec(pub LocalPart, pub terminal::At, pub Domain);
    pub enum LocalPart {
        DotAtom(DotAtom),
        QuotedString(QuotedString),
    }
    pub enum Domain {
        DotAtom(DotAtom),
        DomainLiteral(DomainLiteral),
    }
    pub struct DomainLiteral(
        terminal::OpenBracket,
        Vec<terminal::DText>,
        terminal::CloseBracket,
    );
    pub struct Atom(pub Vec<terminal::AText>);
    pub struct DotAtomLiteral(pub terminal::Dot, pub Atom);
    pub struct DotAtom(pub Atom, pub Vec<DotAtomLiteral>);
    pub struct QuotedPair(pub terminal::Backslash, pub terminal::Escape);
    pub enum QContent {
        QText(terminal::QText),
        QuotedPair(QuotedPair),
    }
    pub struct QuotedString(
        pub terminal::DQuote,
        pub Vec<QContent>,
        pub terminal::DQuote,
    );
}
