# CMN Topology Tool

This repository contains software responsible for generating all data necessary to derive the ARM CMN topology. 
It has been tested on an Ampere Altra and Altra Max with a CMN-600, but it should be compatible with any CMN-600 equipped CPUs, including future versions of the CMN.

It relies on performance counters exposed by the CMN PMU, namely:
- `mxp_n_dat_txflit_valid`
- `mxp_e_dat_txflit_valid`
- `mxp_s_dat_txflit_valid`
- `mxp_w_dat_txflit_valid`
- `mxp_p0_dat_txflit_valid`
- `mxp_p1_dat_txflit_valid`

Note: Access to these events requires either elevated permissions or setting `kernel.perf_event_paranoid` to -1!

The tool has been tested on a Linux openSUSE Leap 15 SP5, kernel version 5.14.21 and compiled using `rustc` version 1.75.0.

This repo also contains a minified extract of the [core-to-core latency benchmark](https://github.com/nviennot/core-to-core-latency) written by Nicolas Viennot, MIT License, 
which is placed in `src/benchmark`.

You can build this project using `cargo build -r` in the main directory. The binaries will be in `target/release/`.

## Subcommands

### determine-topology

This is the main command, responsible for gathering and determining the CMN topology.

Invoke like:

```sh
./measurement determine-topology \
    --numa-config "monolithic" \
    --benchmark-binary-path path/to/benchmark \
    --benchmark-binary-args "--num-iterations 10000 --num-samples 2000" 
```

Refer to src/benchmark/README.md for information on the `benchmark` executable.

### launch

This command launches a binary and simultaneously measures counted performance events per CMN MXP.

To record specific events, use the following command:

```bash
./measurement \
    --events "<list,of,events>" \
    --mesh-x 8 \
    --mesh-y 8 \
    --cores-per-dsu 2 \
    --nodeid-length 9 \
    launch \
    --binary path/to/binary \
    --args "--args for --binary"
```

The `--events` parameter accepts a list of CMN events. Each event is then recorded for each MXP Port on the mesh. 
You can alternatively address individual ports per MXP using the syntax `$port:$event`. 
Example: `0:mxp_n_dat_txflit_valid` would only record the `mxp_n_dat_txflit_valid` event for port 0 for all MXPs. 

### launch-multi

This command launches several binaries, as specified in a config file, and simultaneously measures the collected perf events.

The configuration file can be as follows: 
```json
{
    "executables": [
      {
        "binary": "executable_1",
        "args": "--run-the-thing",
        "shell": "/bin/bash",
        "env": {
          "ENV1": "val"
        },
        "core_map": "list,of,cores,from-to"
      }
    ]
}
```

## Get specific information for CMN Topology Visualisation

In order to derive information about the location of memory, storage, and network controllers, you can use the following commands
and analyse the output using the visualisation script at https://gitlab.lrz.de/friese-arm-cmn.

#### Memory Data

This uses the STREAM memory benchmark.

```shell
./target/release/measurement \
  --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
  --mesh-x 8 --mesh-y 6 \
  launch --binary stream.100M
```

#### Storage Data

This uses the GNU `dd` tool.

```shell
./target/release/measurement \
    --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
    --mesh-x 8 --mesh-y 6 \
    launch --binary dd \
    --args 'if=/dev/zero of=/tmp/benchmark oflag=direct bs=2M count=8k status=none'
```
#### Network Data

##### TCP/IP

This uses `iperf3`.

```shell
./target/release/measurement \
    --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
    --mesh-x 8 --mesh-y 6 \
    launch \
    --binary iperf3 --args '-c 10.97.4.4 -t 20'
```

##### Specific High-Speed-Network

For Slingshot, this uses the `cxi_read_bw`.

```shell
./target/release/measurement \
    --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
    --mesh-x 8 --mesh-y 6 \
    launch \
    --env 'FI_CXI_LLRING_MODE=never' \
    --binary cxi_read_bw --args '-n 50000 10.115.3.6'
```

# License

MIT License
