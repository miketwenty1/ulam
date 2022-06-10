pub fn is_prime(num: u32) -> bool {
    let num64 = num as u64;

    primal::is_prime(num64)
}
