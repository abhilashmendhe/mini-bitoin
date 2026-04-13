pub fn modulo(num: isize, M: isize) -> isize {
    ((num % M) + M) % M
}