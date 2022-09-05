use std::cmp::Ordering;

pub fn power_of_thor(
    light_x: &i32,
    light_y: &i32,
    initial_tx: &i32,
    initial_ty: &i32,
) -> (String, i32, i32) {
    let mut current_tx = *initial_tx;
    let mut current_ty = *initial_ty;
    let mut direction = String::new();

    match (current_ty - light_y).cmp(&0) {
        Ordering::Greater => {
            direction.push('N');
            current_ty -= 1;
        }
        Ordering::Less => {
            direction.push('S');
            current_ty += 1;
        }
        Ordering::Equal => (),
    }

    match (current_tx - light_x).cmp(&0) {
        Ordering::Greater => {
            direction.push('W');
            current_tx -= 1;
        }
        Ordering::Less => {
            direction.push('E');
            current_tx += 1;
        }
        Ordering::Equal => (),
    }

    (direction, current_tx, current_ty)
}
