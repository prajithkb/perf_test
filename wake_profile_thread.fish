sudo offwaketime-bpfcc -f 20 -p (pgrep thread_perf) > out.stacks && ~/workspace/FlameGraph/flamegraph.pl --color=chain --title="Off Wake Time Flame Graph" --countname=us < out.stacks > thread_wake_cpu.svg