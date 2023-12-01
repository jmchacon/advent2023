cargo b --release
for i in `seq 1 10`; do echo day$i; cargo r -p day$i -q --release; done
