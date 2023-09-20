//! Parser for email address (`addr-spec`) as defined in Section 3.4.1 of [`RFC5322`].
//! This crate implements only a subset of the grammar and does not support folding white space
//! and comments in email address. Also, the grammar rules that are defined to preserve backwards
//! compatibility are not supported. The grammar implemented is described below:
//!
//! ```text
//! addr-spec      =  local-part "@" domain
//! local-part     =  dot-atom / quoted-string
//! domain         =  dot-atom / domain-literal
//! domain-literal =  "[" *dtext "]"
//! dtext          =  %d33-90 / %d94-126
//!                          ; Printable US-ASCII characters not
//!                          ;  including "[", "]" or "\".
//! atext          =  ALPHA / DIGIT / "!" /
//!                   "#" / "$" / "%" /
//!                   "&" / "'" / "*" /
//!                   "+" / "-" / "/" /
//!                   "=" / "?" / "^" /
//!                   "_" / "`" / "{" /
//!                   "|" / "}" / "~"
//!                          ; Printable US-ASCII characters not
//!                          ;  including specials. Used for atoms.
//! atom           =  1*atext
//! dot-atom       =  1*atext *("." 1*atext)
//! specials       =  "(" / ")" / "<" / ">" / "@" /
//!                   "[" / "]" / ":" / ";" / "\" /
//!                   "," / "." / DQUOTE
//!                          ; Special characters that do not appear in
//!                          ;  atext. Useful for tools that perform
//!                          ;  lexical analysis: each character in
//!                          ;  specials can be used to indicate a
//!                          ;  tokenization point in lexical analysis.
//! quoted-pair    =  "\" (VCHAR / WSP)
//! qtext          =  %d33 / %d35-91 / %d93-126
//!                          ; Printable US-ASCII characters not
//!                          ;  including "\" or the quote character.
//! qcontent       =  qtext / quoted-pair
//! quoted-string  =  DQUOTE *qcontent DQUOTE
//!                          ; The quote and backslash characters may
//!                          ;  appear in a quoted-string so long as they
//!                          ;  appear as a quoted-pair.
//! ; Given below are "Core rules" of [RFC5234] used above
//! ALPHA          =  %x41-5A / %x61-7A   ; A-Z / a-z
//! DIGIT          =  %x30-39             ; 0-9
//! DQUOTE         =  %x22                ; " (Double Quote)
//! VCHAR          =  %x21-7E             ; visible (printing) characters
//! WSP            =  SP / HTAB           ; white space
//! HTAB           =  %x09                ; horizontal tab
//! SP             =  %x20                ; space character
//! ```
//!
//! [`RFC5322`]: https://datatracker.ietf.org/doc/html/rfc5322#section-3.4.1
