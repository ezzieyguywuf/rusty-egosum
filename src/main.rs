#[derive(Debug)]
enum MyError {
    InputTooShort,
    InputNotGoSumString
}

fn parse_gosum_line(data: &str) -> Result<String, MyError> {
    let vals: Vec<&str> = data.split_whitespace().take(2).collect();
    let out: String = vals.join(&" ");

    if vals.len() < 2 {
        return Err(MyError::InputTooShort);
    }

    return Ok(out);
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gosum_line_works() {
        let parsed = parse_gosum_line("github.com/dlclark/regexp2 v1.4.0/go.mod h1:2pZnwuY/m+8K6iRw6wQdMtk+rH5tNGR1i55kozfMjCc=");
        let check  = "github.com/dlclark/regexp2 v1.4.0/go.mod";

        assert_eq!(parsed.unwrap(), check);
    }
}
