use regex::Regex;
/*
 * case 1
 * 通配符(.)
 * 匹配一个除换行符\n以外的任意符号
 */
#[test]
fn wildcard() {
    let s = "apple ape agree age amaze animate advertise a\ne";

    let re = Regex::new(r"a.e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|mat| mat.as_str()).collect();
    assert_eq!(matches, ["ape", "age", "aze", "ate"]);

    let re = Regex::new(r"a..e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|mat| mat.as_str()).collect();
    assert_eq!(matches, ["agre", "adve"]);

    let re = Regex::new(r"a...e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|mat| mat.as_str()).collect();
    assert_eq!(matches, ["apple", "agree", "amaze"]);
}
