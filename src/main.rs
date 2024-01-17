mod parser;
mod ast;
use parser::*;
fn main() {
    let c_function = "int add(int a, int b) {return a + b;}";
    let res = parse_identifier(c_function);
    let res2 = parse_type_specifier(c_function);
    let res3 = parse_parameter(c_function);
    let res4 = parse_binary_op("a+b");
    let res5 = parse_return_statement("return a + b;");
    let res6 = parse_compound_statement("{   return a + b; }");

    println!("{:?} \n {:?} \n {:?} \n {:?} \n {:?} \n {:?} \n" , res, res2, res3, res4, res5, res6);

    match parse_function_definition(c_function) {
        Ok((_, function_ast)) => {
            println!("Parsed successfully! AST: {:?}", function_ast);
        }
        Err(error) => {
            println!("Error parsing the function: {:?}", error);
        }
    }
    
    let func = "int add(int a, int b) {a = 5; b = 6; return a + b;}" ;

}

