run one of:

bin/run_all_tests_debug.sh 
bin/run_all_tests_release.sh 

bin/testenvelop.sh bin/run_all_tests_debug.sh 
bin/testenvelop.sh bin/run_all_tests_release.sh 

bin/testenvelop_compiler_comparison.sh bin/run_all_tests_debug.sh
bin/testenvelop_compiler_comparison.sh bin/run_all_tests_release.sh

