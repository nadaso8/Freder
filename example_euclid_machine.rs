
fn main () {
    let mut divisor = 57;
    let mut dividend = 249;
    let mut remainder = 0;

    while (remainder != 1) {
        remainder = dividend % divisor;
        dividend = divisor;
        divisor = remainder;
    }

    return; 
}