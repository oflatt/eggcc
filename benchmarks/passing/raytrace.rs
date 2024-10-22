// ARGS: 0.0 0.0 0.0 50.0 50.0

// builds a cube mesh with two sides
// z is positive going up, x goes positive to the right
// y is positive going into the screen
// then builds a bvh tree from the cube mesh
fn main(xpos: f64, ypos: f64, zpos: f64, width: f64, height: f64) {
    let num_triangles: i64 = 100;
    let mut first_point: [f64; 3] = [0.0; 3];
    let mut first_triangle: [[f64; 3]; 3] = [first_point; 3];
    // 100 triangles, 3 points per triangle
    // hack: need a single point to initialize the array
    let mut triangles: [[[f64; 3]; 3]; 100] = [first_triangle; 100];

    // allocate triangles
    let mut ind: i64 = 0;
    while ind < 100 {
        let point1: [f64; 3] = [0.0; 3];
        let point2: [f64; 3] = [0.0; 3];
        let point3: [f64; 3] = [0.0; 3];
        let triangle: [[f64; 3]; 3] = [point1, point2, point3];
        triangles[ind as usize] = triangle;
        ind = ind + 1;
    }

    // drop the first triangle
    drop(first_point);
    drop(first_triangle);

    build_cube(xpos, ypos, width, height, triangles);

    // For each bvh, store the indices of the two children
    // Use -1 for no child
    let first_child: [i64; 2] = [-1, -1];
    let mut bvh_children: [[i64; 2]; 100] = [first_child; 100];
    let dummy_point: [f64; 3] = [0.0; 3];
    let dummy_bbox: [[f64; 3]; 2] = [dummy_point, dummy_point];
    // For each bvh, store the start position
    let mut bvh_bbox: [[[f64; 3]; 2]; 100] = [dummy_bbox; 100];
    // for each bvh, store the start index into triangles
    let mut bvh_start: [i64; 100] = [0; 100];
    // for each bvh, store the size of the interval into triangles
    let mut bvh_size: [i64; 100] = [0; 100];

    // initialize bvh_children
    let mut ind: i64 = 1;
    while ind < 100 {
        let mut next_children: [i64; 2] = [-1, -1];
        bvh_children[ind as usize] = next_children;
        ind = ind + 1;
    }

    // initialize bvh_bbox
    let mut ind: i64 = 0;
    while ind < 100 {
        let start_point: [f64; 3] = [0.0; 3];
        let end_point: [f64; 3] = [0.0; 3];
        let next_bbox: [[f64; 3]; 2] = [start_point, end_point];
        bvh_bbox[ind as usize] = next_bbox;
        ind = ind + 1;
    }

    // build the bvh
    build_bvh(
        &mut triangles,
        &mut bvh_children,
        &mut bvh_bbox,
        &mut bvh_start,
        &mut bvh_size,
    );

    // define the output screen
    let dummy_row: [f64; 100] = [0.0; 100];
    let output: [[f64; 100]; 100] = [dummy_row; 100];
    drop(dummy_row);
    let i: i64 = 0;
    while i < 100 {
        let row: [f64; 100] = [0.0; 100];
        output[i as usize] = row;
        i = i + 1;
    }

    // print out the screen
    let rowi: i64 = 0;
    while rowi < 100 {
        let col: i64 = 0;
        while col < 100 {
            let to_print: f64 = output[rowi as usize][col as usize];
            println!("{}", to_print);
            col = col + 1;
        }
        rowi = rowi + 1;
    }

    let res: f64 = triangles[10][0][0];

    // drop screen
    let i: i64 = 0;
    while i < 100 {
        drop(output[i as usize]);
        i = i + 1;
    }
    drop(output);

    // drop dummy point and dummy bbox
    drop(dummy_point);
    drop(dummy_bbox);

    // drop the triangles
    let mut ind: i64 = 0;
    while ind < 100 {
        let mut point: i64 = 0;
        while point < 3 {
            drop(triangles[ind as usize][point as usize]);
            point = point + 1;
        }
        drop(triangles[ind as usize]);
        ind = ind + 1;
    }
    drop(triangles);

    // drop bvh_children, bvh_bbox
    let mut ind: i64 = 0;
    while ind < 100 {
        drop(bvh_children[ind as usize]);
        drop(bvh_bbox[ind as usize][0]);
        drop(bvh_bbox[ind as usize][1]);
        drop(bvh_bbox[ind as usize]);
        ind = ind + 1;
    }
    drop(bvh_children);
    drop(bvh_bbox);
    drop(bvh_start);
    drop(bvh_size);
    println!("{}", res);
}

/*
fn sample_ray(px: f64, py: f64) -> [[f64; 3]; 2] {
    // virtual camera is at (0, 0, 0)
    // extending to (0.2, 0.0, 0.2)
    // light is at (0.0, -0.5, 0.0)
    let mut light: [f64; 3] = [0.0, -0.5, 0.0];

    let mut camera_pos: [f64; 3] = [px, py, 0.0];
    let mut diff: [f64; 3] = vec_sub(camera_pos, light);
    let mut dir: [f64; 3] = vec_normalize(diff);
    let mut ray: [[f64; 3]; 2] = [camera_pos, dir];
    drop(light);
    drop(diff);
    return ray;
} */

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

fn vec_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    return [a[0] * s, a[1] * s, a[2] * s];
}

fn abs(x: f64) -> f64 {
    if x < 0.0 {
        return -x;
    } else {
        return x;
    }
}

fn sqrt(x: f64) -> f64 {
    let mut guess: f64 = x;
    let tolerance: f64 = 1e-10; // Set precision level

    while abs(guess * guess - x) > tolerance {
        guess = (guess + x / guess) / 2.0;
    }

    guess
}

fn vec_len(a: [f64; 3]) -> f64 {
    return sqrt(a[0] * a[0] + a[1] * a[1] + a[2] * a[2]);
}
/*


fn vec_normalize(a: [f64; 3]) -> [f64; 3] {
    let len: f64 = vec_len(a);
    return [a[0] / len, a[1] / len, a[2] / len];
}
*/
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

// Constructs BVH tree, re-arranging the order of triangles
// so that the BVH can refer to intervals of in the triangles array.
fn build_bvh(
    triangles: &mut [[[f64; 3]; 3]; 100],
    bvh_children: &mut [[i64; 2]; 100],
    bvh_bbox: &mut [[[f64; 3]; 2]; 100],
    bvh_start: &mut [i64; 100],
    bvh_size: &mut [i64; 100],
) {
    make_leaf_node(
        0,
        100,
        triangles,
        bvh_children,
        bvh_bbox,
        bvh_start,
        bvh_size,
        0,
    );
    let mut bvh_next_free: i64 = 1;
    let mut node_stack: [i64; 100] = [0; 100];
    let mut stack_size: i64 = 1;

    let mut directions: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    let num_partitions_try: i64 = 32;

    while stack_size > 0 {
        let node: i64 = node_stack[stack_size as usize - 1];
        stack_size = stack_size - 1;

        let best_partition: [f64; 3] = [0.0; 3];
        let large_num: f64 = 9999999999999999.0;
        let best_cost: f64 = large_num;
        let direction_index: i64 = 0;
        while direction_index < 3 {
            let partition_i: i64 = 0;
            let partition_i_float: f64 = 0.0;
            while partition_i < num_partitions_try {
                let scaled_max: [f64; 3] = vec_scale(bvh_bbox[node][1], 1.0 / 32.0);
                let scaled_min: [f64; 3] = vec_scale(bvh_bbox[node][0], 1.0 / 32.0);
                let subtracted: [f64; 3] = vec_sub(scaled_max, scaled_min);
                let dist: f64 = dot(subtracted, directions[direction_index]) * partition_i_float;
                drop(scaled_max);
                drop(scaled_min);
                drop(subtracted);

                let partition: [f64; 3] = vec_scale(directions[direction_index], dist);
                // TODO calculate correct partition cost
                let cost: f64 = 1.0;

                if cost < best_cost {
                    drop(best_partition);
                    best_cost = cost;
                    best_partition = partition;
                } else {
                    drop(partition);
                }

                partition_i = partition_i + 1;
                partition_i_float = partition_i_float + 1.0;
            }
            direction_index = direction_index + 1;
        }

        let middle: i64 = partition(
            node,
            triangles,
            bvh_children,
            bvh_bbox,
            bvh_start,
            bvh_size,
            &best_partition,
        );
        let left_child: i64 = bvh_next_free;
        make_leaf_node(
            bvh_start[node as usize],
            middle,
            triangles,
            bvh_children,
            bvh_bbox,
            bvh_start,
            bvh_size,
            bvh_next_free,
        );
        bvh_next_free = bvh_next_free + 1;
        let right_child: i64 = bvh_next_free;
        make_leaf_node(
            middle,
            bvh_start[node as usize] + bvh_size[node as usize],
            triangles,
            bvh_children,
            bvh_bbox,
            bvh_start,
            bvh_size,
            bvh_next_free,
        );
        bvh_next_free = bvh_next_free + 1;

        bvh_children[node as usize][0] = left_child;
        bvh_children[node as usize][1] = right_child;

        if bvh_size[left_child as usize] > 1 {
            node_stack[stack_size as usize] = left_child;
            stack_size = stack_size + 1;
        }

        if bvh_size[right_child as usize] > 1 {
            node_stack[stack_size as usize] = right_child;
            stack_size = stack_size + 1;
        }

        drop(best_partition);
    }

    drop(node_stack);
    drop(directions[0]);
    drop(directions[1]);
    drop(directions[2]);
    drop(directions);
}

fn partition(
    node: i64,
    triangles: &mut [[[f64; 3]; 3]; 100],
    bvh_children: &mut [[i64; 2]; 100],
    bvh_bbox: &mut [[[f64; 3]; 2]; 100],
    bvh_start: &mut [i64; 100],
    bvh_size: &mut [i64; 100],
    partition: &[f64; 3],
) -> i64 {
    // always partition the first child in it's own box
    // TODO make this sort the triangles and actually calculate cost
    return bvh_start[node as usize] + 1;
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn point_max(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    return [max(a[0], b[0]), max(a[1], b[1]), max(a[2], b[2])];
}

fn point_min(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    return [min(a[0], b[0]), min(a[1], b[1]), min(a[2], b[2])];
}

fn triangle_min(tri: [[f64; 3]; 3]) -> [f64; 3] {
    let point1: [f64; 3] = tri[0];
    let point2: [f64; 3] = tri[1];
    let point3: [f64; 3] = tri[2];
    let min1: [f64; 3] = point_min(point1, point2);
    let min2: [f64; 3] = point_min(min1, point3);
    drop(min1);
    return min2;
}

fn triangle_max(tri: [[f64; 3]; 3]) -> [f64; 3] {
    let point1: [f64; 3] = tri[0];
    let point2: [f64; 3] = tri[1];
    let point3: [f64; 3] = tri[2];
    let max1: [f64; 3] = point_max(point1, point2);
    let max2: [f64; 3] = point_max(max1, point3);
    drop(max1);
    return max2;
}

fn bbox_triangle(tri: [[f64; 3]; 3]) -> [[f64; 3]; 2] {
    let min: [f64; 3] = triangle_min(tri);
    let max: [f64; 3] = triangle_max(tri);
    let bbox: [[f64; 3]; 2] = [min, max];
    return bbox;
}

fn bbox_union(a: [[f64; 3]; 2], b: [[f64; 3]; 2]) -> [[f64; 3]; 2] {
    let min: [f64; 3] = point_min(a[0], b[0]);
    let max: [f64; 3] = point_max(a[1], b[1]);
    let bbox: [[f64; 3]; 2] = [min, max];
    return bbox;
}

fn make_leaf_node(
    start: i64,
    end: i64,
    triangles: &mut [[[f64; 3]; 3]; 100],
    bvh_children: &mut [[i64; 2]; 100],
    bvh_bbox: &mut [[[f64; 3]; 2]; 100],
    bvh_start: &mut [i64; 100],
    bvh_size: &mut [i64; 100],
    node_id: i64,
) {
    bvh_start[start as usize] = start;
    bvh_size[start as usize] = end - start;

    bvh_children[start as usize][0] = node_id;
    bvh_children[start as usize][1] = node_id;

    let mut i: i64 = start;
    let mut bbox: [[f64; 3]; 2] = bbox_triangle(triangles[i as usize]);
    while i < end {
        let next_bbox: [[f64; 3]; 2] = bbox_triangle(triangles[i as usize]);
        let bbox_old: [[f64; 3]; 2] = bbox;
        bbox = bbox_union(bbox, next_bbox);
        drop(bbox_old[0]);
        drop(bbox_old[1]);
        drop(bbox_old);
        drop(next_bbox[0]);
        drop(next_bbox[1]);
        drop(next_bbox);

        i = i + 1;
    }

    drop(bvh_bbox[node_id as usize][0]);
    drop(bvh_bbox[node_id as usize][1]);
    drop(bvh_bbox[node_id as usize]);
    bvh_bbox[node_id as usize] = bbox;
}

fn build_cube(xpos: f64, ypos: f64, width: f64, height: f64, mut triangles: [[[f64; 3]; 3]; 100]) {
    let num_rectangles: i64 = 25;
    let rect_width: f64 = width / 25.0;

    // build the front face, clockwise triangles
    let mut i: i64 = 0;
    let mut ti: i64 = 0;
    let mut cx: f64 = xpos + width / 2.0;
    while i < num_rectangles {
        // top left corner
        triangles[ti as usize][0][0] = cx;
        triangles[ti as usize][0][1] = ypos;
        triangles[ti as usize][0][2] = 0.0;

        // top right corner
        triangles[ti as usize][1][0] = cx + rect_width;
        triangles[ti as usize][1][1] = ypos;
        triangles[ti as usize][1][2] = 0.0;

        // bottom left corner
        triangles[ti as usize][2][0] = cx;
        triangles[ti as usize][2][1] = ypos;
        triangles[ti as usize][2][2] = -height;

        ti = ti + 1;
        // top right corner
        triangles[ti as usize][0][0] = cx + rect_width;
        triangles[ti as usize][0][1] = ypos;
        triangles[ti as usize][0][2] = 0.0;

        // bottom right corner
        triangles[ti as usize][1][0] = cx + rect_width;
        triangles[ti as usize][1][1] = ypos;
        triangles[ti as usize][1][2] = -height;

        // bottom left corner
        triangles[ti as usize][2][0] = cx;
        triangles[ti as usize][2][1] = ypos;
        triangles[ti as usize][2][2] = -height;

        ti = ti + 1;
        cx = cx + rect_width;
        i = i + 1;
    }

    // build the top face
    let mut i: i64 = 0;
    let mut cx: f64 = xpos + width / 2.0;
    while i < num_rectangles {
        // front left corner
        triangles[ti as usize][0][0] = cx;
        triangles[ti as usize][0][1] = ypos;
        triangles[ti as usize][0][2] = 0.0;

        // back left corner
        triangles[ti as usize][1][0] = cx;
        triangles[ti as usize][1][1] = ypos + height;
        triangles[ti as usize][1][2] = 0.0;

        // back right corner
        triangles[ti as usize][2][0] = cx + rect_width;
        triangles[ti as usize][2][1] = ypos + height;
        triangles[ti as usize][2][2] = 0.0;

        ti = ti + 1;
        // front left corner
        triangles[ti as usize][0][0] = cx;
        triangles[ti as usize][0][1] = ypos;
        triangles[ti as usize][0][2] = 0.0;

        // back right corner
        triangles[ti as usize][1][0] = cx + rect_width;
        triangles[ti as usize][1][1] = ypos + height;
        triangles[ti as usize][1][2] = 0.0;

        // front right corner
        triangles[ti as usize][2][0] = cx + rect_width;
        triangles[ti as usize][2][1] = ypos;
        triangles[ti as usize][2][2] = 0.0;

        ti = ti + 1;
        cx = cx + rect_width;
        i = i + 1;
    }
}
