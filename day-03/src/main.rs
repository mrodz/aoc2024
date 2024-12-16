use std::{fs::File, io::Read, ops::Deref};

use anyhow::{bail, Context, Result};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct InputParser;

struct Arguments(Vec<u64>);

impl Arguments {
    fn parse(input: Pair<'_, Rule>) -> Result<Self> {
        if input.as_rule() != Rule::arguments {
            bail!("not rule::arguments");
        }

        let mut result = vec![];

        for child in input.into_inner() {
            if ![Rule::number, Rule::lpar, Rule::comma, Rule::rpar].contains(&child.as_rule()) {
                bail!("bad arguments");
            }

            if child.as_rule() != Rule::number {
                continue;
            }

            result.push(
                child
                    .as_str()
                    .parse()
                    .with_context(|| format!("could not parse u8 from {:?}", child.as_str()))?,
            );
        }

        Ok(Self(result))
    }
}

impl Deref for Arguments {
    type Target = [u64];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum Operation {
    Mul,
}

impl Operation {
    fn parse(input: Pair<'_, Rule>) -> Result<Self> {
        if input.as_rule() != Rule::operation {
            bail!("not rule::operation");
        }

        let ty = input
            .into_inner()
            .next()
            .context("operation missing inner")?
            .as_rule();

        match ty {
            Rule::mul => Ok(Self::Mul),
            other => bail!("{other:?} is not an operation"),
        }
    }

    fn is_valid(&self, arguments: &Arguments) -> bool {
        match self {
            Self::Mul => arguments.len() == 2,
        }
    }
}

struct Invocation {
    operation: Operation,
    arguments: Arguments,
}

impl Invocation {
    fn parse(input: Pair<'_, Rule>) -> Result<Option<Self>> {
        if input.as_rule() != Rule::invocation {
            bail!("not rule::invocation");
        }

        let mut iter = input.into_inner();

        let operation_pair = iter.next().context("rule::invocation missing operation")?;
        let operation = Operation::parse(operation_pair)?;

        let arguments_pair = iter.next().context("rule::invocation missing arguments")?;
        let arguments = Arguments::parse(arguments_pair)?;

        if !operation.is_valid(&arguments) {
            return Ok(None);
        }

        Ok(Some(Self {
            operation,
            arguments,
        }))
    }

    fn execute(&self) -> u64 {
        match self.operation {
            Operation::Mul => self.arguments.iter().product(),
        }
    }
}

fn part_one(input: &str) -> Result<()> {
    let mut root_parse = InputParser::parse(Rule::file, input)?;

    let file = root_parse.next().context("file not present")?;

    let invocations = file.into_inner();
    let mut sum = 0;

    for invocation_pair in invocations {
        if invocation_pair.as_rule() != Rule::invocation {
            continue;
        }

        let Some(invocation) = Invocation::parse(invocation_pair)? else {
            continue;
        };

        sum += invocation.execute();
    }

    println!("Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications? The Answer is {sum}");

    Ok(())
}

fn main() -> Result<()> {
    let mut file = File::options()
        .read(true)
        .open("./input.txt")
        .context("could not open input")?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    part_one(&input)?;
    Ok(())
}
