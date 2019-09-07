mkdir ergo_cf_release
cargo build --release
cp target/release/ergo_cf ergo_cf_release
echo "http://0.0.0.0:9052" > ergo_cf_release/node.ip