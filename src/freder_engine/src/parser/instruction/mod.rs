use std::string;
use marlea_engine::trial::reaction_network::{
    ReactionNetwork,
    reaction::{
        Reaction,
        term::Term,
    },
};


enum Variable {
    Constant(String, usize),  // a constant value which will be optimized into the structure of the CRN
    Boolean(String), // a value which must always have a low value NOTE: this is not necesarilly always less than 1
    IntDescrete(String), // a value who's quantity is represented in in the count of a single species
    IntPacked(String, usize), // a value who's quanitty is represented by the presence of some width of species
}

impl Variable {
    fn from() -> Self {
        todo!()
    }
}

enum Condition {
    GreaterThan(Variable, Variable),
    LessThan(Variable, Variable), 
    Equivalent(Variable, Variable),
}

impl Condition {
    fn from() -> Self {
        todo!()
    }
}

enum Instruction {
    CustomBlock(ReactionNetwork, Variable, Variable), // manually define a chemical reaction network. if compiled with the rustc compiler it will instantiate marlea and use the simulated result to run as a standard program
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

impl Instruction {
    fn from() -> Self{ 
        todo!()
    }
}