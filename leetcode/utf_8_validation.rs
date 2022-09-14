use std::convert::TryFrom;

pub fn valid_utf8(data: Vec<i32>) -> bool {
    let data = data
        .iter()
        .map(|&i| u8::try_from(i).unwrap())
        .collect::<Vec<_>>();
    let mut r = -1;
    for i in data {
        if r > 0 {
            if (0b1000_0000..=0b1011_1111).contains(&i) {
                r -= 1;
                continue;
            } else {
                return false;
            }
        }

        r = if i <= 0b0111_1111 {
            0
        } else if (0b1100_0000..=0b1101_1111).contains(&i) {
            1
        } else if (0b1100_0000..=0b1110_1111).contains(&i) {
            2
        } else if (0b1111_0000..=0b1111_0111).contains(&i) {
            3
        } else {
            return false;
        };
    }
    r == 0
}
