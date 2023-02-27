use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::env;
use std::process::exit;

#[derive(Debug, Default, PartialEq)]
struct Thing {
    number: u8,
}

impl TryFrom<&str> for Thing {
    type Error = anyhow::Error;

    fn try_from(num_str: &str) -> std::result::Result<Self, Self::Error> {
        let number = num_str.parse::<u8>().map_err(|e| anyhow!(e))?;
        let thing = Thing { number };

        Ok(thing)
    }
}

impl TryFrom<String> for Thing {
    type Error = anyhow::Error;

    fn try_from(num_string: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(num_string.as_str())
    }
}

fn test(num_str: &str) -> Result<()> {
    println!("INFO: num_str: {:?}", num_str);

    let thing = Thing::try_from(num_str)?;

    println!("INFO: thing: {:?}", thing);

    Ok(())
}

fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];

    if args.len() < 2 {
        println!("ERROR: {}: specify number", program_name);
        exit(1);
    }

    let num_str = &args[1];

    test(num_str)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {:#}", e);
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_thing_try_from_invalid_str() {
        let thing = Thing::try_from("foo").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: invalid digit found in string"
    )]
    fn test_thing_try_from_empty_str() {
        let thing = Thing::try_from(" ").unwrap();
    }

    #[test]
    fn test_thing_try_from() {
        //#[derive(Debug, PartialEq)]
        #[derive(Debug)]
        struct TestData<'a> {
            str: &'a str,
            result: Result<Thing>,
        }

        let tests = &[
            TestData {
                str: "",
                result: Err(anyhow!("cannot parse integer from empty string")),
            },
            TestData {
                str: "not a number",
                result: Err(anyhow!("invalid digit found in string")),
            },
            TestData {
                str: "     ",
                result: Err(anyhow!("invalid digit found in string")),
            },
            TestData {
                str: "123aaa",
                result: Err(anyhow!("invalid digit found in string")),
            },
            TestData {
                str: "a123",
                result: Err(anyhow!("invalid digit found in string")),
            },
            TestData {
                str: "\"",
                result: Err(anyhow!("invalid digit found in string")),
            },
            TestData {
                str: ".",
                result: Err(anyhow!("invalid digit found in string")),
            },
            TestData {
                str: "0",
                result: Ok(Thing { number: 0 }),
            },
            TestData {
                str: "1",
                result: Ok(Thing { number: 1 }),
            },
            TestData {
                str: "254",
                result: Ok(Thing { number: 254 }),
            },
            TestData {
                str: "255",
                result: Ok(Thing { number: 255 }),
            },
            TestData {
                str: "256",
                result: Err(anyhow!("number too large to fit in target type")),
            },
        ];

        for (i, d) in tests.iter().enumerate() {
            let msg = format!("test[{}]: {:?}", i, d);

            let result = Thing::try_from(d.str);

            let msg = format!("{}, result: {:?}", msg, result);

            if std::env::var("DEBUG").is_ok() {
                println!("FIXME: msg: {:?}", msg);
            }

            if d.result.is_ok() {
                assert_eq!(
                    result.as_ref().unwrap(),
                    d.result.as_ref().unwrap(),
                    "{}",
                    msg
                );
                continue;
            }

            let expected_error = format!("{}", d.result.as_ref().unwrap_err());
            let actual_error = format!("{}", result.unwrap_err());
            assert!(actual_error == expected_error, "{}", msg);
        }
    }
}
