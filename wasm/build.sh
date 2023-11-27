# shellcheck disable=SC2086
function Stage() {
    root_path=$(pwd)
    chapter_name=$1
    cd roguelike/${chapter_name} || exit
    cargo build --release --target wasm32-unknown-unknown

    wasm-bindgen $root_path/target/wasm32-unknown-unknown/release/${chapter_name}.wasm --out-dir $root_path/target/wasm/${chapter_name} --no-modules --no-typescript

    cd $root_path || exit
    mv target/wasm/${chapter_name}/${chapter_name}.js target/wasm/${chapter_name}/game.js
    mv target/wasm/${chapter_name}/${chapter_name}_bg.wasm target/wasm/${chapter_name}/game_bg.wasm
    cp wasm/index-template.html target/wasm/${chapter_name}/index.html
}

SCRIPT_PATH=$(readlink -f "$0")
PARENT_DIR=$(dirname "$(dirname "$SCRIPT_PATH")")

cd ${PARENT_DIR} || exit

rm -rf target/wasm

cargo clean && cargo build --release --target wasm32-unknown-unknown --all
chapters=($(find roguelike -type d -maxdepth 1 -name "ch*"  -exec sh -c 'echo ${0#roguelike/}' {} \;))
for chapter in "${chapters[@]}"; do
    Stage ${chapter}
done