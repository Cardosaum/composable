#!/bin/bash
# To run benchmarks `node` must be build with benchmarks enabled in release mode 
# `cargo build --release --features runtime-benchmarks`

chains=(
  "./runtime/composable/src/weights,composable-dev"
  "./runtime/picasso/src/weights,picasso-dev"
  "./runtime/dali/src/weights,dali-dev"
)

steps=50
repeat=20

pallets=(
	oracle
	frame_system
	timestamp
	session
	balances
	indices
	membership
	treasury
	scheduler
	collective
	democracy
	collator_selection
	utility
	lending
	dutch_auction
)

# shellcheck disable=SC2068
for i in ${chains[@]}; do
  while IFS=',' read -r output chain; do
    # shellcheck disable=SC2068
    for p in ${pallets[@]}; do
	    ./target/release/composable benchmark \
		    --chain="$chain" \
		    --execution=wasm \
		    --wasm-execution=compiled \
		    --pallet="$p"  \
		    --extrinsic='*' \
		    --steps=$steps  \
		    --repeat=$repeat \
		    --raw  \
		    --output="$output"
    done
  done <<< "$i"
done
