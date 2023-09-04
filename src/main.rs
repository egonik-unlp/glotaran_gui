use glotaran_conv_lib::build_file_from_fluorescence_data;
fn main() {
    let filepath = "TRES_plaintext.txt";
    build_file_from_fluorescence_data(200., 0.055, filepath);
}