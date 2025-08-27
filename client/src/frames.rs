fn add_frame(content: &mut Vec<Vec<String>>, with_markers: bool) {
    let content_rows = content.len();
    let content_cols = content[0].len();

    let top_right_corner = "+".to_owned();
    let bottom_left_corner = top_right_corner.clone();
    let bottom_right_corner = top_right_corner.clone();
    let top_left_corner = match with_markers {
        true => "0".to_owned(),
        false => top_right_corner.clone(),
    };

    let mut bottom_border = vec!["-".to_owned(); content_cols];
    let right_border = vec![concat!(RESET_COLOR!(), "|").to_owned(); content_rows];
    let mut top_border = bottom_border.clone();
    let mut left_border = right_border.clone();
    if with_markers {
        for col_marker in 1..content_cols / 8 {
            top_border[col_marker * 8] = col_marker.to_string();
        }
        for row_marker in 1..content_rows / 4 {
            left_border[row_marker * 4] = row_marker.to_string();
        }
    }

    for row in 0..content_rows {
        content[row].insert(0, left_border[row].clone());
        content[row].push(right_border[row].clone());
    }
    let mut top_row = vec![top_left_corner];
    top_row.append(&mut top_border);
    top_row.push(top_right_corner);
    let mut bottom_row = vec![bottom_left_corner];
    bottom_row.append(&mut bottom_border);
    bottom_row.push(bottom_right_corner);

    content.insert(0, top_row);
    content.push(bottom_row);
}