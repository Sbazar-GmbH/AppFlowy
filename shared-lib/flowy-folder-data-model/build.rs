use lib_infra::pb_gen;

fn main() {
    pb_gen::gen("flowy-folder-data-model", "./src/protobuf/proto");
}
