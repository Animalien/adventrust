use itertools::Itertools;

use adventlib::*;

struct BoxyBox {
    pub pos: (usize, usize, usize),
    pub circuit_index_index: Option<usize>,
}

struct BoxPair {
    pub index0: usize,
    pub index1: usize,
    pub dist_sq: usize,
}

struct Circuit {
    pub box_indices: Vec<usize>,
}

fn process(filename: &str, num_connections: usize, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {filename}");

    // read the boxes

    let mut boxes = Vec::<BoxyBox>::new();
    boxes.reserve(lines.len());
    for line in &lines {
        boxes.push(BoxyBox { pos: line.split(',').map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap(), circuit_index_index: None });
    }

    // create box_pairs, and initialize squared distances

    let mut box_pairs = Vec::<BoxPair>::new();
    for i in 0..boxes.len() {
        for j in (i+1)..boxes.len() {
            let box0 = &boxes[i];
            let box1 = &boxes[j];

            let diff = (box1.pos.0.abs_diff(box0.pos.0), box1.pos.1.abs_diff(box0.pos.1), box1.pos.2.abs_diff(box0.pos.2));
            let dist_sq = diff.0 * diff.0 + diff.1 * diff.1 + diff.2 * diff.2;

            box_pairs.push(BoxPair { index0: i, index1: j, dist_sq });
        }
    }

    // sort box-pairs by distance squared, increasing

    box_pairs.sort_by_key(|p| p.dist_sq);

    // make # num connections, building circuits as we go

    let mut circuit_list = Vec::<Circuit>::new();
    let mut circuit_index_list = Vec::<usize>::new();
    for box_pair in box_pairs.iter_mut().take(num_connections) {
        let box0 = &boxes[box_pair.index0];
        let box1 = &boxes[box_pair.index1];

        if box0.circuit_index_index.is_some() {
            if box1.circuit_index_index.is_some() {
                // merge the two circuits, box1's circuit into box0's circuit
                let old_circuit_indices: Vec<usize> = circuit_list[box1.circuit_index_index.unwrap()].box_indices.drain(..).collect();
                circuit_list[box0.circuit_index_index.unwrap()].box_indices.extend(old_circuit_indices);
                circuit_index_list[box1.circuit_index_index.unwrap()] = box0.circuit_index_index.unwrap();
            } else {
                // put box1 in box0's circuit
                boxes[box_pair.index1].circuit_index_index = box0.circuit_index_index;
                circuit_list[circuit_index_list[boxes[box_pair.index0].circuit_index_index.unwrap()]].box_indices.push(box_pair.index1);
            }
        } else {
            if box1.circuit_index_index.is_some() {
                // put box0 in box1's circuit
                boxes[box_pair.index0].circuit_index_index = box1.circuit_index_index;
                circuit_list[circuit_index_list[boxes[box_pair.index1].circuit_index_index.unwrap()]].box_indices.push(box_pair.index0);
            } else {
                // create a new circuit and put both boxes in there
                let new_circuit = Circuit { box_indices: vec![box_pair.index0, box_pair.index1] };
                let circuit_index = circuit_list.len();
                circuit_list.push(new_circuit);
                let circuit_index_index = circuit_index_list.len();
                circuit_index_list.push(circuit_index);
                boxes[box_pair.index0].circuit_index_index = Some(circuit_index_index);
                boxes[box_pair.index1].circuit_index_index = Some(circuit_index_index);
            }
        }
    }

    if verbose {
        println!("  Dump of circuits:");
        for (index, circuit) in circuit_list.iter().enumerate() {
            println!("    Circuit at index {index} has boxes:  {:?}", circuit.box_indices);
        }
    }

    // now find the three largest circuits

    let mut largest_circuit_index0: Option<usize> = None;
    let mut largest_circuit_index1: Option<usize> = None;
    let mut largest_circuit_index2: Option<usize> = None;

    for (index, circuit) in circuit_list.iter().enumerate() {
        if largest_circuit_index0.is_none() || (largest_circuit_index0.is_some() && (circuit.box_indices.len() > circuit_list[largest_circuit_index0.unwrap()].box_indices.len())) {
            largest_circuit_index2 = largest_circuit_index1;
            largest_circuit_index1 = largest_circuit_index0;
            largest_circuit_index0 = Some(index);
        } else if largest_circuit_index1.is_none() || (largest_circuit_index1.is_some() && (circuit.box_indices.len() > circuit_list[largest_circuit_index1.unwrap()].box_indices.len())) {
            largest_circuit_index2 = largest_circuit_index1;
            largest_circuit_index1 = Some(index);
        } else if largest_circuit_index2.is_none() || (largest_circuit_index2.is_some() && (circuit.box_indices.len() > circuit_list[largest_circuit_index2.unwrap()].box_indices.len())) {
            largest_circuit_index2 = Some(index);
        }
    }

    let largest_circuit_size0 = circuit_list[largest_circuit_index0.unwrap()].box_indices.len();
    let largest_circuit_size1 = circuit_list[largest_circuit_index1.unwrap()].box_indices.len();
    let largest_circuit_size2 = circuit_list[largest_circuit_index2.unwrap()].box_indices.len();

    println!("  Multiple of sizes of three largest circuits = {largest_circuit_size0} * {largest_circuit_size1} * {largest_circuit_size2} = {}", largest_circuit_size0 * largest_circuit_size1 * largest_circuit_size2);
}

fn main() {
    process("Example.txt", 10, true);
    process("Input.txt", 1000, true);
}
