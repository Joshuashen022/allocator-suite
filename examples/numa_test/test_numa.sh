cargo build --release
for i in {0..3}
do
        nohup taskset -c 33  ./target/release/numa_test 2>&1 >> numa_log  &
done

for i in {0..3}
do
        nohup taskset -c 3  ./target/release/numa_test 2>&1 >> numa_log  &
done


for i in {0..3}
do
        nohup taskset -c 100  ./target/release/numa_test 2>&1 >> numa_log  &
done