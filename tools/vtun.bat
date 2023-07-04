rem https://www.intel.com/content/www/us/en/docs/vtune-profiler/user-guide/2023-0/compiler-switches-perf-analysis-windows-targets.html
"C:\Program Files (x86)\Intel\oneAPI\vtune\latest\bin64\vtune" -collect hotspots -user-data-dir %tmp% %*
