#![allow(dead_code, unused_variables)]
use crate::{Graph, Vertex};
use nalgebra::Vector3;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Face {
    pub outer_loop: Vec<usize>,
    pub inner_loops: Vec<Vec<usize>>,
}

pub fn identify_faces(graph: &Graph) -> Vec<Face> {
    let mut visited = HashSet::new();
    let mut faces = Vec::new();

    let mut face_result: Vec<Face> = Vec::new();

    for u in 0..graph.vertices.len() {
        if !visited.contains(&u) {
            let mut current_face = Vec::new();
            dfs(&graph, u, &mut current_face, &mut visited, &mut faces);
        }
    }

    for face in &mut faces {
        // Check for inner loops
        let mut potential_inner_loops = Vec::new();
        for i in 0..face.len() {
            let start = face[i];
            let mut loop_vertices = Vec::new();
            let mut j = (i + 1) % face.len();
            while j != i {
                loop_vertices.push(face[j]);
                j = (j + 1) % face.len();
            }
            if loop_vertices.len() > 2 && is_inner_loop(&loop_vertices, face, &graph.vertices) {
                potential_inner_loops.push(loop_vertices);
            }
        }

        let outer_loop = face.clone();
        let inner_loops: Vec<Vec<usize>> = potential_inner_loops.iter().cloned().collect();
        face_result.push(Face {
            outer_loop,
            inner_loops,
        });
    }

    face_result
}

pub fn calculate_face_normals(faces: Vec<Face>, vertices: &Vec<Vertex>) -> Vec<Vector3<f32>> {
    let mut normals = Vec::new();

    for face in faces {
        let v1 = vertices[face.outer_loop[1]] - vertices[face.outer_loop[0]];
        let v1 = Vector3::new(v1.x, v1.y, v1.z);

        let v2 = vertices[face.outer_loop[2]] - vertices[face.outer_loop[0]];
        let v2 = Vector3::new(v2.x, v2.y, v2.z);

        let normal = v1.cross(&v2);
        normals.push(normal.normalize());
    }

    normals
}

fn is_inner_loop(loop_vertices: &[usize], face_vertices: &[usize], vertices: &[Vertex]) -> bool {
    // Winding number calculation
    let point = &vertices[loop_vertices[0]]; // Choose a point inside the loop
    let mut winding_number = 0;

    for i in 0..loop_vertices.len() {
        let p1 = &vertices[loop_vertices[i]];
        let p2 = &vertices[loop_vertices[(i + 1) % loop_vertices.len()]];
        if p1.y <= point.y && p2.y > point.y {
            let x_intersect = (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
            if x_intersect > point.x {
                winding_number += 1;
            }
        } else if p1.y > point.y && p2.y <= point.y {
            let x_intersect = (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
            if x_intersect < point.x {
                winding_number -= 1;
            }
        }
    }

    winding_number != 0
}

fn dfs(
    graph: &Graph,
    u: usize,
    current_face: &mut Vec<usize>,
    visited: &mut HashSet<usize>,
    face_vec: &mut Vec<Vec<usize>>,
) {
    visited.insert(u);
    current_face.push(u);

    for v in &graph.edges[&u] {
        if !visited.contains(v) {
            dfs(graph, *v, current_face, visited, face_vec);
        } else if *v == current_face[0] && current_face.len() > 2 {
            // Found a potential face
            face_vec.push(current_face.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inner_loop() {
        // Test cases
        let simple_face = vec![0, 1, 2];
        let inner_loop_face = vec![0, 1, 2, 3, 4, 5, 6];
        // ... other test cases

        // Create sample vertices
        let vertices = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
        ];

        // Test with different loop configurations
        assert!(!is_inner_loop(&simple_face[1..], &simple_face, &vertices));
        // ... other test cases
    }

    #[test]
    fn test_is_inner_loop_simple() {
        let vertices = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ];

        let outer_loop = vec![0, 1, 2, 3];
        let inner_loop = vec![1, 2];

        assert!(!is_inner_loop(&inner_loop, &outer_loop, &vertices));
    }

    #[test]
    fn test_is_inner_loop_complex() {
        // Create sample vertices

        let vertices = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.5, 0.5, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(0.5, 1.5, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(-0.5, 0.5, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(-1.5, -0.5, 0.0),
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(-0.5, -1.5, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
        ];

        // Create a complex face with overlapping inner loops
        let complex_face = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let inner_loop1 = vec![2, 3, 4];
        let inner_loop2 = vec![6, 7, 8];

        // Test if inner loops are correctly identified
        assert!(is_inner_loop(&inner_loop1, &complex_face, &vertices));
        assert!(is_inner_loop(&inner_loop2, &complex_face, &vertices));
    }
}
