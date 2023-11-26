cargo build --release --target wasm32-unknown-unknown --all

function Stage() {
    chapter_name=$1
    cd ${chapter_name}
    cargo build --release --target wasm32-unknown-unknown

    wasm-bindgen ../target/wasm32-unknown-unknown/release/${chapter_name}.wasm --out-dir ../book/book/wasm/${chapter_name} --no-modules --no-typescript

    cd ..

    mv ./book/book/wasm/${chapter_name}/${chapter_name}.js ./book/book/wasm/${chapter_name}/game.js
    mv ./book/book/wasm/${chapter_name}/${chapter_name}_bg.wasm ./book/book/wasm/${chapter_name}/game_bg.wasm
    cp wasm.html ./book/book/wasm/${chapter_name}/index.html

}

chapters=($(find . -type d -maxdepth 1 -regex './ch[0-9]\{2,\}-.*' -exec sh -c 'echo ${0#./}' {} \;))
for chapter in "${chapters[@]}"; do
    Stage $chapter
done