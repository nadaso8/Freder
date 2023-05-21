use std::string::String;
use marlea_engine::trial::reaction_network::{
    ReactionNetwork,
    reaction::{
        Reaction,
        term::Term,
    },
};

enum Variable {
    Constant(String, usize),  
    Boolean(String), 
    IntDescrete(String),
    IntPacked(String, usize), 
}

impl From<&str> for Variable {
    fn from(s: &str) -> Self {
        todo!("read over for logical completeness and decide how variables which have an associated size should be determined");
        match s.split(':').collect::<Vec<&str>>()[..] {
            [name_str, "const", size_str] => {
                let name = name_str.to_string();
                let size: usize = size_str.parse().unwrap();
                Variable::Constant(name, size)
            }
            [name_str, "bool"] => {
                let name = name_str.to_string();
                Variable::Boolean(name)
            }
            [name_str, "int", size_str] => {
                let name = name_str.to_string();
                let size: usize = size_str.parse().unwrap();
                Variable::IntPacked(name, size)
            }
            [name_str, "int"] => {
                let name = name_str.to_string();
                Variable::IntDescrete(name)
            }
            [_, type_str, _] => panic!("invalid variable type: {}", type_str),
            _ => panic!("invalid variable declaration"),
        }
    }
}


enum Condition {
    GreaterThan(Variable, Variable),
    LessThan(Variable, Variable), 
    Equivalent(Variable, Variable),
}

impl From<&str> for Condition {
    /// converts from a str written in rust syntax into the various condition types 
    fn from(s: &str) -> Self {
        todo!()
    }
}

enum Instruction {
    CustomBlock(ReactionNetwork, Variable, Variable),
    Assign(Variable),
    Add(Variable, Variable), 
    DestructiveAdd(Variable,Variable),
    Subtract(Variable, Variable),
    DestructiveSubtract(Variable,Variable),
    Multiply(Variable,Variable),
    Divide(Variable,Variable),
    Modulus(Variable,Variable),
    Exponentiate(Variable,Variable),
    Log(Variable,Variable),
    Goto(usize),
    Ternary(Condition,usize,usize),
}

impl From<&str> for Instruction {
    /// converts from a str written in rust syntax into the various Instruction types 
    fn from(s: &str) -> Self {
        todo!()
    }
}

impl Instruction {
    fn synthesize(self, reaction_block_index: usize) -> ReactionNetwork {
        todo!()
    }
}
