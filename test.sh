#!/bin/bash

CPUEmulatorPath="../../tools/CPUEmulator.sh"

cargo build
BIN_PATH="./target/debug/vmtrans"
TEST_NAME=("BasicTest" "PointerTest" "StaticTest")
for test_case in "${TEST_NAME[@]}"
do
    echo "###### TEST_CASE: $test_case #######"
    $BIN_PATH -i ./$test_case.vm -o ../07/MemoryAccess/$test_case/$test_case.asm
    $CPUEmulatorPath ../07/MemoryAccess/$test_case/$test_case.tst
done