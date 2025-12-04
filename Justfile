run DAY=`date +%d`:
    cargo run --release --bin {{DAY}} < input/{{DAY}}

example DAY=`date +%d` N="1":
    cargo run --release --bin {{DAY}} < examples/{{DAY}}/{{N}}

build DAY=`date +%d`:
    cargo build --release --bin {{DAY}}
