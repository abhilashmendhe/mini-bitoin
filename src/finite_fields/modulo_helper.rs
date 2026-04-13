pub fn modulo(num: isize, m: isize) -> isize {
    ((num % m) + m) % m
}