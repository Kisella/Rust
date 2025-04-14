use regex::Regex;
/*
 * case 3
 * {}: 指定左边原子可以重复的范围
 */
#[test]
fn braces() {
    let s = "aeeee apple ape agree age amaze animate advertise a\ne a&e a@e a6e a9e yuan";

    let re = Regex::new(r"a.{3}e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|m| m.as_str()).collect();
    assert_eq!(matches, ["aeeee", "apple", "agree", "amaze"]);

    /* {2.3}表示可以出现2到3次 */
    /* 默认贪婪匹配: 按照最大匹配数进行匹配 */
    let re = Regex::new(r"a.{2,3}e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|m| m.as_str()).collect();
    assert_eq!(matches, ["aeeee", "apple", "agree", "amaze", "adve"]);

    /* 问号(?)放到重复符号({}*+)后面，懒惰匹配：按照最小匹配数优先匹配 */
    let re = Regex::new(r"a.{2,3}?e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|m| m.as_str()).collect();
    assert_eq!(matches, ["aeee", "apple", "agre", "amaze", "adve"]);

    /* {}右边的数字不写，代表无穷 */
    let re = Regex::new(r"a.{2,}e").unwrap(); // ae之间匹配2到无穷多个任意字符
    let matches: Vec<_> = re.find_iter(s).map(|m| m.as_str()).collect();
    assert_eq!(
        matches,
        [
            "aeeee apple ape agree age amaze animate advertise",
            "a&e a@e a6e a9e"
        ]
    );
    
    let re = Regex::new(r"a[a-z]{1,3}e").unwrap();
    let matches: Vec<_> = re.find_iter(s).map(|m| m.as_str()).collect();
    assert_eq!(matches, ["aeeee", "apple", "ape", "agree", "age", "amaze", "ate", "adve"]);
}
