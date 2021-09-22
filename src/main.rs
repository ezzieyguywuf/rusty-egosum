use std::io;
use std::fs;

#[derive(Debug, PartialEq)]
enum MyError {
    InputTooShort,
    InputTooLong
}

fn parse_gosum_line(data: &str) -> Result<String, MyError> {
    let vals: Vec<&str> = data.split_whitespace().collect();

    if vals.len() < 2 {
        return Err(MyError::InputTooShort);
    } else if vals.len() > 3 {
        return Err(MyError::InputTooLong);
    }
    
    let out: String = vals[..2].join(&" ");

    return Ok(out);
}

fn quote(data: &str) -> String {
    let mut out = String::from("\"");

    out.push_str(data);
    out.push_str(&"\"");

    return out;
}

fn main() -> io::Result<()> {
    let res = fs::read_to_string("go.sum");

    match res {
        Ok(data) => {
            let out = 
                    data
                    .lines()
                    .map(|line| {
                        let parsed = parse_gosum_line(line).unwrap_or_default();
                        let mut out = String::from("\"");

                        out.push_str(&parsed);
                        out.push_str(&"\"");

                        out
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
            println!("{}", out);
        }
        Err(_) => println!("Could not find go.sum in the current directory. Bailing out.")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_on_too_short() {
        assert_eq!(parse_gosum_line("abc").unwrap_err(), MyError::InputTooShort);
    }

    #[test]
    fn ok_on_long_enough() {
        assert!(parse_gosum_line("abc cde").is_ok());
        assert!(parse_gosum_line("abc cde fgh").is_ok());
    }

    #[test]
    // Per https://golang.org/ref/mod#go-sum-files , should only contain 3 sections
    fn err_on_too_long() {
        assert_eq!(parse_gosum_line("abc cde fgh\t\thijk").expect_err("Failed to error on input that is too long"),
                MyError::InputTooLong);
    }

    #[test]
    fn parse_valid_string() {
        let parsed = parse_gosum_line("github.com/dlclark/regexp2 v1.4.0/go.mod h1:2pZnwuY/m+8K6iRw6wQdMtk+rH5tNGR1i55kozfMjCc=");
        let check  = "github.com/dlclark/regexp2 v1.4.0/go.mod";

        assert_eq!(parsed.unwrap(), check);
    }

    #[test]
    fn wrap_in_quotes() {
        assert_eq!(quote("test"), "\"test\"");
        assert_eq!(quote("test abc def"), "\"test abc def\"");
        assert_eq!(quote("test\t\t foobar^&"), "\"test\t\t foobar^&\"");
    }
}
