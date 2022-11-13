#!/usr/bin/env bash
set -ex

mkdir -p /tmp/mnt{0,1,2}

../target/debug/rcore-fs-dfs /tmp/mnt0 0 &
sleep 2
../target/debug/rcore-fs-dfs /tmp/mnt1 1 &
sleep 2
../target/debug/rcore-fs-dfs /tmp/mnt2 2 &
sleep 2

mount | grep /tmp/mnt

mkdir -p /tmp/mnt0/{level1dir0,level1dir1}/{level2dir0,level2dir1,level2dir2}
tree /tmp/mnt2

mkdir -p /tmp/mnt1/{level1dir0,level1dir1,level1dir2}/{level2dir3,level2dir4,level2dir5}
tree /tmp/mnt2

echo "test writing file" > /tmp/mnt2/test
cat /tmp/mnt0/test

pkill rcore-fs-dfs
umount /tmp/mnt{0,1,2}
