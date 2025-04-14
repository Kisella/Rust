use regex::Regex;
/*
 * case 002
 * 字符集[]
 * 匹配一个中括号内的符号之一
 * []中不需要添加任何的分隔符，否则那些"分隔符"也会被视为符号之一，而被筛选出来
 * []中可以使用-来确定字母或数字的范围
 * []中第一个字符若为尖角号(^), 意味着取反
 */
#[test]
fn character_set() {
    let s = "apple ape agree age amaze animate advertise a\ne a&e a@e a6e a9e";

    let re = Regex::new(r"a.e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|mat| mat.as_str()).collect();
    assert_eq!(
        matches,
        ["ape", "age", "aze", "ate", "a&e", "a@e", "a6e", "a9e"]
    );

    let re = Regex::new(r"a[pgzt]e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|mat| mat.as_str()).collect();
    assert_eq!(matches, ["ape", "age", "aze", "ate"]);

    // []中可以使用-来确定字母或数字的范围
    let re = Regex::new(r"a[0-9]e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|mat| mat.as_str()).collect();
    assert_eq!(matches, ["a6e", "a9e"]);

    // []中第一个字符若为尖角号(^), 意味着取反
    let re = Regex::new(r"a[^0-9]e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|mat| mat.as_str()).collect();
    assert_eq!(matches, ["ape", "age", "aze", "ate", "a\ne", "a&e", "a@e"]);
}
