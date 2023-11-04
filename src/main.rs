fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }

    a.sort_by(|a: &f32, b: &f32| a.partial_cmp(b).unwrap());
    let middle = a.len() / 2;

    let med: f32 = if (a.len() % 2) == 0 {
        ((a[middle - 1]) + (a[middle])) / 2.0
    } else {
        a[middle]
    };

    Some(med)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
