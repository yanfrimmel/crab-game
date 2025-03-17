use macroquad::prelude::Vec2;

// Check if two line segments intersect
pub fn line_segment_intersection(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> bool {
    let a1 = p2.y - p1.y;
    let b1 = p1.x - p2.x;
    let c1 = a1 * p1.x + b1 * p1.y;

    let a2 = p4.y - p3.y;
    let b2 = p3.x - p4.x;
    let c2 = a2 * p3.x + b2 * p3.y;

    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0.0 {
        // Lines are parallel
        false
    } else {
        let x = (b2 * c1 - b1 * c2) / determinant;
        let y = (a1 * c2 - a2 * c1) / determinant;

        // Check if the intersection point lies within both line segments
        let on_segment1 = x >= p1.x.min(p2.x)
            && x <= p1.x.max(p2.x)
            && y >= p1.y.min(p2.y)
            && y <= p1.y.max(p2.y);
        let on_segment2 = x >= p3.x.min(p4.x)
            && x <= p3.x.max(p4.x)
            && y >= p3.y.min(p4.y)
            && y <= p3.y.max(p4.y);
        on_segment1 && on_segment2
    }
}

// Check if two triangles intersect
pub fn triangle_triangle_intersection(tri1: (Vec2, Vec2, Vec2), tri2: (Vec2, Vec2, Vec2)) -> usize {
    let mut result: usize = 0;
    //// Triangle 1's three edges
    //let tri1_edges = [(tri1.0, tri1.1), (tri1.1, tri1.2), (tri1.2, tri1.0)];
    //
    //// Triangle 2's three edges
    //let tri2_edges = [(tri2.0, tri2.1), (tri2.1, tri2.2), (tri2.2, tri2.0)];
    //
    //// Check if any edge of triangle 1 intersects with any edge of triangle 2
    //for tri1_edge in &tri1_edges {
    //    for tri2_edge in &tri2_edges {
    //        if line_segment_intersection(tri1_edge.0, tri1_edge.1, tri2_edge.0, tri2_edge.1) {
    //            result += 1;
    //        }
    //    }
    //}

    // Check if any vertex of triangle 1 is inside triangle 2
    for vertex in [tri1.0, tri1.1, tri1.2] {
        if point_in_triangle(vertex, tri2) {
            result += 1;
        }
    }

    // Check if any vertex of triangle 2 is inside triangle 1
    for vertex in [tri2.0, tri2.1, tri2.2] {
        if point_in_triangle(vertex, tri1) {
            result += 1;
        }
    }

    // If none of the above checks passed, the triangles do not intersect
    result
}

// Check if a triangle intersects with a rectangle
pub fn triangle_rectangle_intersection(
    tri: (Vec2, Vec2, Vec2),
    rect: (f32, f32, f32, f32), // (x, y, width, height)
) -> bool {
    let (rect_x, rect_y, rect_w, rect_h) = rect;

    // Rectangle's four edges
    let rect_edges = [
        (
            Vec2::new(rect_x, rect_y),
            Vec2::new(rect_x + rect_w, rect_y),
        ), // Top edge
        (
            Vec2::new(rect_x + rect_w, rect_y),
            Vec2::new(rect_x + rect_w, rect_y + rect_h),
        ), // Right edge
        (
            Vec2::new(rect_x, rect_y + rect_h),
            Vec2::new(rect_x + rect_w, rect_y + rect_h),
        ), // Bottom edge
        (
            Vec2::new(rect_x, rect_y),
            Vec2::new(rect_x, rect_y + rect_h),
        ), // Left edge
    ];

    // Triangle's three edges
    let tri_edges = [(tri.0, tri.1), (tri.1, tri.2), (tri.2, tri.0)];

    // Check if any triangle edge intersects with any rectangle edge
    for tri_edge in &tri_edges {
        for rect_edge in &rect_edges {
            if line_segment_intersection(tri_edge.0, tri_edge.1, rect_edge.0, rect_edge.1) {
                return true;
            }
        }
    }

    // Check if any triangle vertex is inside the rectangle
    let rect_min_x = rect_x;
    let rect_max_x = rect_x + rect_w;
    let rect_min_y = rect_y;
    let rect_max_y = rect_y + rect_h;

    for vertex in [tri.0, tri.1, tri.2] {
        if vertex.x >= rect_min_x
            && vertex.x <= rect_max_x
            && vertex.y >= rect_min_y
            && vertex.y <= rect_max_y
        {
            return true;
        }
    }

    // Check if any rectangle vertex is inside the triangle
    let rect_vertices = [
        Vec2::new(rect_x, rect_y),
        Vec2::new(rect_x + rect_w, rect_y),
        Vec2::new(rect_x + rect_w, rect_y + rect_h),
        Vec2::new(rect_x, rect_y + rect_h),
    ];

    for vertex in rect_vertices {
        if point_in_triangle(vertex, tri) {
            return true;
        }
    }

    false
}

// Check if a point is inside a triangle
fn point_in_triangle(p: Vec2, tri: (Vec2, Vec2, Vec2)) -> bool {
    let (a, b, c) = tri;

    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);

    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_neg && has_pos)
}

fn sign(p1: Vec2, p2: Vec2, p3: Vec2) -> f32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}

// Helper function to rotate a point around a pivot
pub fn rotate_point(point: Vec2, pivot: Vec2, angle: f32) -> Vec2 {
    let sin = angle.sin();
    let cos = angle.cos();

    // Translate point back to origin
    let translated_point = Vec2::new(point.x - pivot.x, point.y - pivot.y);

    // Rotate point
    let rotated_point = Vec2::new(
        translated_point.x * cos - translated_point.y * sin,
        translated_point.x * sin + translated_point.y * cos,
    );

    // Translate point back
    Vec2::new(rotated_point.x + pivot.x, rotated_point.y + pivot.y)
}
