// Weblog parser module
use regex::Regex;
use weblog::Weblog;

pub fn parse_weblog(line: &str) -> Weblog {
    let re = Regex::new("(?x)
      (\\S+)\\s
      (\\S+)\\s
      (\\S+)\\s
      \\[(\\d{2}/\\w+/\\d{4}:\\d{2}:\\d{2}:\\d{2}\\s+.+?)\\]\\s
      \"(\\w+\\s\\S+\\s\\w+/\\d+\\.\\d+)\"\\s
      (\\d+)\\s
      (\\d+)\\s
      \"(\\S+)\"\\s
      \"(.+)\"
    ").unwrap();

    let mut v: Vec<Weblog> = vec![];

    for cap in re.captures_iter(line) {
        let logline = Weblog::new(
            cap.at(1).unwrap_or("").to_string(),
            cap.at(4).unwrap_or("").to_string(),
            cap.at(5).unwrap_or("").to_string(),
            cap.at(6).unwrap_or("0").parse::<i32>().unwrap(),
            cap.at(7).unwrap_or("0").parse::<i32>().unwrap(),
            cap.at(8).unwrap_or("").to_string(),
            cap.at(9).unwrap_or("").to_string()
        );
        v.push(logline);
    }
    v.pop().unwrap()
}