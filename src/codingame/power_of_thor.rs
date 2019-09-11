pub fn power_of_thor(
    light_x: &i32,
    light_y: &i32,
    initial_tx: &i32,
    initial_ty: &i32,
) -> (String, i32, i32) {
    let mut current_tx = initial_tx.clone();
    let mut current_ty = initial_ty.clone();
    let mut direction = String::new();

    if current_ty - light_y > 0 {
        direction.push('N');
        current_ty -= 1;
    } else if current_ty - light_y < 0 {
        direction.push('S');
        current_ty += 1;
    }

    if current_tx - light_x > 0 {
        direction.push('W');
        current_tx -= 1;
    } else if current_tx - light_x < 0 {
        direction.push('E');
        current_tx += 1;
    }

    (direction, current_tx, current_ty)
}
