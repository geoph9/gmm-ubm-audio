// This file is responsible for reading the training and test set .npy files and matching them to
//   their corresponding labels.
// It contains a function that returns these features as 2 ndarrays where the rows
//   are the the audio files and the columns are the features (e.g. 13 columns for 13 mfccs).

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use ndarray::{array, Array2, Array1};
use ndarray_npy::{ReadNpyError, ReadNpyExt};



fn read_train_test(train_dir: &Path,
                   test_dir: &Path,
                   train_labels_path: &Path,
                   test_labels_path: &Path) -> (Array2<f64>, HashMap<&Path, &str>,
                                                Array2<f64>, HashMap<&Path, &str>) {
    let train_feas = read_features(train_dir);
    let train_labels = read_labels(train_labels_path, train_dir);

    let test_feas = read_features(test_dir);
    let test_labels = read_labels(test_labels_path, test_dir);


    (train_feas, train_labels, test_feas, test_labels)
}


fn read_labels(path: &Path, fea_dir: &Path) -> HashMap<&Path, &str> {
    // A hashmap mapping from each path to its label
    let mut labels: HashMap<&Path, &str> = HashMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(label) = line {
                // v[0] should be the path and v[1] should be the label
                let v: Vec<&str> = label.split_whitespace().collect();
                if !v[0].exists() {
                    panic!("Wav path does not exist");
                } else {
                    let wav_name = wav_path.file_name().
                        unwrap().to_str().
                        unwrap().replace("wav", "npy");
                    // The corresponding .npy array of the features should exist.
                    // So, we need to convert the current path to match the .npy feature array.
                    let fea_path = Path::new(fea_dir).join(wav_name).as_path();
                    if !fea_path.exists() {
                        panic!("Could not locate the feature path. You need to extract the \
                        .npy features at first.");
                    }
                }
                // Match the current .npy path to its label
                labels.insert(fea_path, v[1]);
            }
        }
    } else {
        panic!("Could not read label file.");
    }
    labels
}


fn read_features(fea_dir: &Path) -> Array2<f64> {
    let mut features: Vec<Array1<f64>> = Vec::new();
    for path in glob(fea_dir.join("*.npy")).expect("Failed to read glob pattern") {
        if let Ok(fea_path) = path {
            let reader = File::open(fea_path);
            let feas = Array1::<f64>::read_npy(reader).
                expect("Could not open numpy array.");
            features.push(feas);
        }
    }
    if features.len() == 0 {
        panic!("Could not find any features...");
    }
    let n_mfccs = features.get(0).
        expect("Found invalid value as array element...").dim();
    Array2::from_shape_fn((features.len(), n_mfccs), |(i, h)| {
        *features.
            get(i).
            expect("Unexpected value in vector (should be a 1dim ndarray).").
            get(j).
            expect("Unexpected value in ndarray.")
    })
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// Credits: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
