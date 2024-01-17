use super::ast::*;
use nom::{
    IResult,
    error::{VerboseError, context},
    character::complete::{alpha1, digit1, multispace1, char, one_of, multispace0},
    sequence::{tuple, delimited, preceded},
    combinator::{map_res, map, recognize},
    multi::separated_list0,
    branch::alt, bytes::complete::tag
};
use nom_supreme::error::{ErrorTree, BaseErrorKind, StackContext, Expectation};


pub type VerboseIResult<T, U> = IResult<T, U, ErrorTree<T>>;

// Parse identifiers (variable names, function names, etc.)
pub fn parse_identifier(input: &str) -> VerboseIResult<&str, String> {
    map(recognize(alpha1), |s: &str| s.to_string())(input)
}

// Parse type specifiers (int, char, etc.)
pub fn parse_type_specifier(input: &str) -> VerboseIResult<&str, TypeSpecifier> {
    alt((
        map(tag("int"), |_| TypeSpecifier::Int),
        map(tag("float"), |_| TypeSpecifier::Float),
        // Add more types 
    ))(input)
}

// Parse integer constants
pub fn parse_constant(input: &str) -> VerboseIResult<&str, Expression> {
    map_res(digit1, |s: &str| s.parse::<i32>().map(Expression::Constant))(input)
}

// Parse parameters (e.g., "int a")
pub fn parse_parameter(input: &str) -> VerboseIResult<&str, FunctionParameter> {
    map(
        tuple((parse_type_specifier, multispace1, parse_identifier)),
        |(type_specifier, _, name)| FunctionParameter {
            type_specifier,
            name,
        },
    )(input)
}

// Parse a binary operation (e.g., "a + b")
pub fn parse_binary_op(input: &str) -> VerboseIResult<&str, Expression> {
    let (input, (left, _, op, _, right)) = tuple((
        parse_identifier,
        multispace0,
        one_of("+-*/"),
        multispace0,
        parse_identifier,
    ))(input)?;

    let operator = match op {
        '+' => BinaryOperator::Add,
        '-' => BinaryOperator::Subtract,
        '*' => BinaryOperator::Multiply,
        '/' => BinaryOperator::Divide,
        _ => unreachable!(),
    };

    Ok((
        input,
        Expression::BinaryOp {
            left: Box::new(Expression::Identifier(left)),
            operator,
            right: Box::new(Expression::Identifier(right)),
        },
    ))
}

// Parse a return statement (e.g., "return a + b;")
pub fn parse_return_statement(input: &str) -> VerboseIResult<&str, Statement> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expression) = parse_binary_op(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, Statement::ReturnStmt(expression)))
}

// Parse a compound statement (e.g., "{ return a + b; }")
pub fn parse_compound_statement(input: &str) -> VerboseIResult<&str, CompoundStatement> {
    let (input, statements) = delimited(
        char('{'),
        delimited(
            multispace0,
            separated_list0(multispace1, parse_return_statement),
            multispace0
        ),
        char('}'),
    )(input)?;

    Ok((input, CompoundStatement { statements }))
}

// Parse a function definition (e.g., "int add(int a, int b) { return a + b; }")
pub fn parse_function_definition(input: &str) -> VerboseIResult<&str, FunctionDefinition> {
    context(
        "function definition",
        tuple((
            parse_type_specifier,
            multispace1,
            parse_identifier,
            delimited(
                char('('),
                separated_list0(
                    preceded(multispace0, char(',')),
                    preceded(multispace0, parse_parameter)
                ),
                char(')'),
            ),
            multispace0,
            parse_compound_statement,
        ))
    )(input)
    .map(|(input, (return_type, _, name, parameters, _, body))| {
        (
            input,
            FunctionDefinition {
                return_type,
                name,
                parameters,
                body,
            },
        )
    })
}


