/*
Author: Marceline Sorensen
Email: nadaso8th@gmail.com
Date: 14/5/2023

## About 
This project compiles a basic subset of code from rust into a Chemicl Reaciton Network which may 
be simulated by it's sister program Marlea https://github.com/nadaso8/MARlea . For greater expressivity 
the rxn_block struct will allow for custom definitions in cases that may require a hand written approach
for optimal functionality or runtime, or which may compute in an anologue rather than discrete space.

Usage: freder `<QUERY>` `<INPUT_PROJECT>` `[Options]`

## Arguments 
- `<QUERY>` specifies the opperation to be preformed. Possible values: "build", "validate".
- `<INPUT_PROJECT>` input project to be used.

## Options
- -o --output `<TYPE>` Speifies the type of output to generate. 
- -t --test `<INIT_FILE>` Automatically starts marlea with default settings using the providet init file
- --no_pack Prevents the usage of packed values making timelines more readable while debugging.
*/

use freder_engine;

fn main () {

}