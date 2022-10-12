If you aren't prepared to set aside a computer as your dedicated
benchmarking system, avoiding running anything else on it, well, who can
blame you?

Here's how I set up a Linux laptop to isolate one core, fairly
effectively, for benchmarking use. With this setup, I can be running a
full desktop environment and still get low-variance timing measurements.

These instructions assume you're using systemd. It should be possible to
do the same without it, using cgroups and taskset directly.

# Pick a core

On this laptop, `lscpu` says I have CPUs numbered 0-15. These correspond
to 8 cores, with two hyperthreads each. I've chosen to isolate core 7,
which Linux has numbered as CPUs 7 and 15.

# Keep system processes off that core

First, let's keep system services from using those CPUs. I ran:

```
sudo systemctl set-property system.slice AllowedCPUs=0-6,8-14
```

`AllowedCPUs` configures a cgroup so the kernel will refuse to schedule
processes in that group on those CPUs, no matter how nicely they ask.

# Keep user processes off that core

I placed the following in a new file as part of systemd's `user.conf`. I
named it `/etc/systemd/user.conf.d/50-cpuaffinity.conf`.

```
[Manager]
# Reserve core 7 for benchmarking. The two hyperthreads are numbered CPU 7 and 15.
CPUAffinity=0-6 8-14
```

Note that here I used `CPUAffinity` instead of `AllowedCPUs`. I need to
be allowed to run my benchmarks on the selected core, and that counts as
user processes. So I set all user processes to avoid the core by
default, but in a way that I can override on a per-process basis.

# Pin core clock frequency

To make certain that clock frequency changes and thermal throttling
can't affect timing measurements, I pin this core to its minimum clock
frequency. For this laptop, that's 800MHz; you can find out what the
range for your CPU is by looking at the "hardware limits" line in the
output of `cpupower frequency-info`.

```sh
sudo cpupower --cpu 7,15 frequency-set --max 800MHz
```

# Steer interrupts to other cores

Modern interrupt controllers support choosing which CPUs each interrupt
should be handled on. In Linux, you can view the current configuration
for each interrupt by looking at `/proc/irq/*/smp_affinity_list`.

On this laptop, most interrupts were configured for all CPUs ("0-15") by
default, but a few have more specific configurations. I've chosen not to
touch any interrupt that's already steered to a specific CPU, even if
that's my benchmarking CPU. I found that steering some interrupts away
was necessary to reduce noise, but I haven't needed to steer away all of
them to get reasonable results.

Here's a shell script I run, as root, to set all the other interrupts:

```sh
grep -lFx 0-15 /proc/irq/*/smp_affinity_list | while read f; do
	echo 0-6,8-14 > "$f" || echo "failed to set $f"
done
```

# Run benchmarks on that core

Finally, when I'm ready to run a benchmark with Sightglass, I wrap it
with `taskset` to override the earlier `user.conf` setting of
`CPUAffinity`.

```sh
taskset --cpu-list 7 cargo run benchmark ...
```

# Caveats

Setting the core to its minimum clock frequency means benchmarks can
take a long time. Using a single core means multi-threaded benchmarks
can take a long time. Compiling the spidermonkey.wasm benchmark takes 23
seconds for me in this configuration, while with parallel builds and
full clock speed it takes about half a second. Have patience.

I've found that cache metrics, such as from the Sightglass `--measure
perf-counters` option, are still noisy to the point of uselessness. This
laptop has an L3 cache that is shared between all cores; I don't know if
there's a way to isolate the benchmarking core from that cache.
