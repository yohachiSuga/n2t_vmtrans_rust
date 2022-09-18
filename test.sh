#!/bin/bash
set -e

CPUEmulatorPath="../../tools/CPUEmulator.sh"

cargo build
BIN_PATH="./target/debug/vmtrans"

TEST_NAME=("SimpleAdd" "StackTest")
for test_case in "${TEST_NAME[@]}"
do
    echo "###### TEST_CASE: $test_case #######"
    $BIN_PATH -i ./$test_case.vm -o ../07/StackArithmetic/$test_case/$test_case.asm
    $CPUEmulatorPath ../07/StackArithmetic/$test_case/$test_case.tst
done

TEST_NAME=("BasicTest" "PointerTest" "StaticTest")
for test_case in "${TEST_NAME[@]}"
do
    echo "###### TEST_CASE: $test_case #######"
    $BIN_PATH -i ./$test_case.vm -o ../07/MemoryAccess/$test_case/$test_case.asm
    $CPUEmulatorPath ../07/MemoryAccess/$test_case/$test_case.tst
done

TEST_NAME=("BasicLoop" "FibonacciSeries")
for test_case in "${TEST_NAME[@]}"
do
    echo "###### TEST_CASE: $test_case #######"
    $BIN_PATH -i ./$test_case.vm -o ../08/ProgramFlow/$test_case/$test_case.asm
    $CPUEmulatorPath ../08/ProgramFlow/$test_case/$test_case.tst
done

TEST_NAME=("SimpleFunction")
for test_case in "${TEST_NAME[@]}"
do
    echo "###### TEST_CASE: $test_case #######"
    $BIN_PATH -i ./$test_case.vm -o ../08/FunctionCalls/$test_case/$test_case.asm
    $CPUEmulatorPath ../08/FunctionCalls/$test_case/$test_case.tst
done

