cargo doc --no-deps
mkdir -p docs
cp -rf target/doc/* ./docs
cp -f index.html docs/index.html
