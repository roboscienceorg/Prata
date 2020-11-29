#!/bin/bash
bash install.sh
cp Build/prata.so Tests/

echo "Running Python tests..."
python Tests/test_general_use_python.py
python Tests/test_statistics.py
python Tests/test_time_python.py
python Tests/test_stats_performance.py
python Tests/test_throughput.py

echo "Running Julia tests..."
julia Tests/test_general_use_julia.jl
julia Tests/test_time_julia.jl
