/// Author: Marceline Sorensen
/// Email: nadaso8th@gmail.com 
/// 
/// # about 
/// This module reads in and tokenizes a file for compilation while removing comments and
/// expanding any macros which may be used. These tokens may then be processed agains an
/// abstraction tree defining a logically equivalent reaction network to generate.
/// 

enum Token {
    // Identifier
    Name(String),
    // Variables
    Const,
    Let,
    Mut,
    // Control Flow
    If,
    Match,
    Fn,
    While,
    Loop,
    For,
    // Comparisons
    LessThan,
    GreaterThan,
    Equal,
    // Opperations
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiate,
    Modulus,
    Open_Parenthesis,
    Closed_Parenthesis,
    Open_curly_Brace,
    Closed_Curly_Brace,
    Comma,
    Period,

}
