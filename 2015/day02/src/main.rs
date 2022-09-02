fn get_box_wrapping(dimensions: &Vec<u32>) -> u32 {
    let mut wrapping_required: u32 = 2 * dimensions[0] * dimensions[1];
    wrapping_required += 2 * dimensions[0] * dimensions[2];
    wrapping_required += 2 * dimensions[1] * dimensions[2];

    wrapping_required += dimensions[0] * dimensions[1];

    wrapping_required
}

fn get_box_ribbon(dimensions: &Vec<u32>) -> u32 {
    let mut ribbon_required: u32 = 2 * dimensions[0] + 2 * dimensions[1];
    ribbon_required += dimensions[0] * dimensions[1] * dimensions[2];

    ribbon_required
}

fn main() {
    let data = include_str!("../input.txt");
    let boxes = data.lines();
    let mut total_wrapping: u32 = 0;
    let mut total_ribbon: u32 = 0;

    for b in boxes {
        let mut box_dimensions: Vec<u32> = b.split("x").map(|x: &str| x.parse::<u32>().unwrap()).collect();
        box_dimensions.sort();

        total_wrapping += get_box_wrapping(&box_dimensions);
        total_ribbon += get_box_ribbon(&box_dimensions);
    }

    println!("Total square feet of wrapping required: {}", total_wrapping);
    println!("Total length of ribbon required: {}", total_ribbon);
}