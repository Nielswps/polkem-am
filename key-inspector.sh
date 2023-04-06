# Not used in service, but useful to have on hand for debugging
cat keys/0 | xargs -I {} ./../substrate/target/release/subkey inspect "0x{}"

for i in {0..99}; do (cat keys/$1 | xargs -I {} ./../substrate/target/release/subkey inspect "0x{}" | sed -n 's/  Public key (SS58): //p'); done
