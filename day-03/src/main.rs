use std::{fs::File, io::Read, ops::Deref};

use anyhow::{bail, Context as C, Result};
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
    Enable,
    Disable,
}

struct Context {
    count: u64,
    enabled: bool,
    freeze_enabled: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            count: 0,
            enabled: true,
            freeze_enabled: true,
        }
    }
}

impl Context {
    fn new() -> Self {
        Self::default()
    }

    fn new_mutable() -> Self {
        Self {
            freeze_enabled: false,
            ..Default::default()
        }
    }

    const fn is_enabled(&self) -> bool {
        self.freeze_enabled || self.enabled
    }

    const fn count(&self) -> u64 {
        self.count
    }

    fn enable(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn add(&mut self, count: u64) {
        self.count += count;
    }
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
            Rule::enable => Ok(Self::Enable),
            Rule::disable => Ok(Self::Disable),
            other => bail!("{other:?} is not an operation"),
        }
    }

    fn is_valid(&self, arguments: &Arguments) -> bool {
        match self {
            Self::Mul => arguments.len() == 2,
            Self::Enable | Self::Disable => arguments.len() == 0,
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

    fn execute(&self, ctx: &mut Context) {
        match self.operation {
            Operation::Mul => {
                if ctx.is_enabled() {
                    ctx.add(self.arguments.iter().product())
                }
            }
            Operation::Enable => ctx.enable(true),
            Operation::Disable => ctx.enable(false),
        }
    }
}

fn sum_of_program(input: &str, context: &mut Context) -> Result<()> {
    let mut root_parse = InputParser::parse(Rule::file, input)?;

    let file = root_parse.next().context("file not present")?;

    let invocations = file.into_inner();

    for invocation_pair in invocations {
        if invocation_pair.as_rule() != Rule::invocation {
            continue;
        }

        let Some(invocation) = Invocation::parse(invocation_pair)? else {
            continue;
        };

        invocation.execute(context);
    }


    Ok(())
}

fn part_one(input: &str) -> Result<()> {
    let mut context = Context::new();
    sum_of_program(input, &mut context)?;
    println!("Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications? The Answer is {}", context.count());
    Ok(())
}

fn part_two(input: &str) -> Result<()> {
    let mut context = Context::new_mutable();
    sum_of_program(input, &mut context)?;
    println!("Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications? The Answer is {}", context.count());
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
    part_two(&input)?;
    
    Ok(())
}
