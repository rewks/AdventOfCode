fn main() {
    let data = include_str!("../input.txt");

    let mut edges: Vec<u32> = Vec::new();
    let mut p1_triangle_count = 0;
    let mut p2_triangle_count = 0;

    for line in data.lines() {
        edges.push(line[..5].trim_start().parse().unwrap());
        edges.push(line[5..10].trim_start().parse().unwrap());
        edges.push(line[10..].trim_start().parse().unwrap());
    }

    for i in (0..edges.len()).step_by(3) {
        let a = edges[i].min(edges[i + 1]);
        let b = edges[i].max(edges[i + 1]).min(edges[i + 2]);
        let c = edges[i].max(edges[i + 1]).max(edges[i + 2]);
        if  a + b > c {
            p1_triangle_count += 1;
        }
    }

    let mut n = 0;
    for i in 0..(edges.len() / 3) {
        let a = edges[i + (n * 6)].min(edges[i + (n * 6) + 3]);
        let b = edges[i + (n * 6)].max(edges[i + (n * 6) + 3]).min(edges[i + (n * 6) + 6]);
        let c = edges[i + (n * 6)].max(edges[i + (n * 6) + 3]).max(edges[i + (n * 6) + 6]);
        if  a + b > c {
            p2_triangle_count += 1;
        }
        if (i + 1) % 3 == 0 {
            n += 1;
        }
    }

    println!("Part 1 number of triangles: {}", p1_triangle_count);
    println!("Part 2 number of triangles: {}", p2_triangle_count);
}