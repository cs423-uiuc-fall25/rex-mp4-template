#!/bin/zsh
set -e

N_THREAD=3
ITER=5

for i in {1..$N_THREAD}; do
    bash -c "while true; do true; done" & # busy wait
    PIDS+=($!)
done

echo "Started ${#PIDS[@]} busy waiters: ${PIDS[@]}"

for i in {1..$ITER}; do
    sleep 2;
    echo "-- Iter $i CPU usage --"
    ./loader ${PIDS[@]}
done

kill "${PIDS[@]}"