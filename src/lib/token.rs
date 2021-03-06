// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

/// A single token representing a Markdown element.
///
/// A Markdown document is, thus, a Vec of `Token`s.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// The most simple element, containing a String
    Str(String),
    /// A paragraph, containing a list of elements
    Paragraph(Vec<Token>),
    /// A header with a header number and the title
    Header(i32, Vec<Token>),
    /// *Emphasis*, a.k.a. italics
    Emphasis(Vec<Token>),
    /// **Strong**, a.k.a. bold
    Strong(Vec<Token>),
    /// `Code`, a.k.a. verbatim
    Code(Vec<Token>),
    /// > A quote
    BlockQuote(Vec<Token>),
    /// Code block with language and content
    CodeBlock(String, Vec<Token>), 

    /// Unordered list, with a vector of `Item`s
    List(Vec<Token>),
    /// Ordered list, with a starting number, and a list of `Item`s
    OrderedList(usize, Vec<Token>),
    /// Item of a list
    Item(Vec<Token>),

    /// Table with number of rows, and a list of `TableHead` and `TableRows`
    Table(i32, Vec<Token>),
    /// Table header, contains `TableCell`s
    TableHead(Vec<Token>),
    /// Row of a table, contains `TableCell`s
    TableRow(Vec<Token>),
    /// Cell of a table
    TableCell(Vec<Token>),

    /// A footnote, contains the content it is pointing to.
    Footnote(Vec<Token>),

    /// Horizontal rule
    Rule,
    /// Softbreak, usually rendered by a space
    SoftBreak,
    /// Hardbreak
    HardBreak,

    /// A link with an url, a title, and the linked text
    Link(String, String, Vec<Token>),
    /// An image with a source url, a title and an alt tex
    Image(String, String, Vec<Token>), 
}

use Token::*;

impl Token {
    /// Returns the inner list of tokens contained in this token (if any)
    pub fn inner(&self) -> Option<&[Token]> {
        match *self {
            Rule
                | SoftBreak
                | HardBreak
                | Str(_) => None,
            Paragraph(ref v) 
                | Header(_, ref v)
                | Emphasis(ref v)
                | Strong(ref v)
                | Code(ref v)
                | BlockQuote(ref v)
                | CodeBlock(_, ref v)
                | List(ref v)
                | OrderedList(_, ref v)
                | Item(ref v)
                | Table(_, ref v)
                | TableHead(ref v)
                | TableRow(ref v)
                | TableCell(ref v)
                | Footnote(ref v)
                | Link(_,_,ref v)
                | Image(_,_,ref v) => Some(v)
        }
    }

    /// Returns the inner list of tokens contained in this token (if any) (mutable version)
    pub fn inner_mut(&mut self) -> Option<&mut Vec<Token>> {
        match *self {
            Rule
                | SoftBreak
                | HardBreak
                | Str(_) => None,
            Paragraph(ref mut v) 
                | Header(_, ref mut v)
                | Emphasis(ref mut v)
                | Strong(ref mut v)
                | Code(ref mut v)
                | BlockQuote(ref mut v)
                | CodeBlock(_, ref mut v)
                | List(ref mut v)
                | OrderedList(_, ref mut v)
                | Item(ref mut v)
                | Table(_, ref mut v)
                | TableHead(ref mut v)
                | TableRow(ref mut v)
                | TableCell(ref mut v)
                | Footnote(ref mut v)
                | Link(_,_,ref mut v)
                | Image(_,_,ref mut v) => Some(v)
        }
    }
}

