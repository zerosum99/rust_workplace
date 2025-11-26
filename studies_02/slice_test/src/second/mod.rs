// 첫번째 단어의 길이 계산 함수
pub fn first_words(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()

}
// 첫번째 단어의 참조 처리
pub fn  first_words_str(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}