call ./install.bat
set "dir1=Build/Wheels"
for %%X in ("%dir1%\*.whl") DO pip install %%~dpnfX

echo "Running Python tests..."
python Tests/test_general_use_python.py
python Tests/test_statistics.py
python Tests/test_stats_performance.py

echo "Running Julia tests..."
julia Tests/test_general_use_julia.jl
julia Tests/test_time_julia.jl
