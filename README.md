# CMN Topology Tool

Tested on CMN600, should be compatible with CMN650.

Commands:

### determine-topology

Invoke like:

```sh
./measurement determine-topology \
    --numa-config "monolithic" \
    --benchmark-binary-path ./benchmark \
    --benchmark-binary-args "--num-iterations 10000 --num-samples 2000" 
```

Refer to src/benchmark/README.md for information on benchmark in determine-core stage.

#### Cache data

hnf, static

#### Memory Data

```shell
./target/release/measurement \
  --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
  --mesh-x 8 --mesh-y 6 \
  launch --binary ~/build/STREAM/stream.100M
```

#### Storage Data

```shell
./target/release/measurement \
    --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
    --mesh-x 8 --mesh-y 6 \
    launch --binary dd \
    --args 'if=/dev/zero of=/tmp/benchmark oflag=direct bs=2M count=8k status=none'
```
#### Network Data

Regular

```shell
./target/release/measurement \
    --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
    --mesh-x 8 --mesh-y 6 \
    launch \
    --binary iperf3 --args '-c 10.97.4.4 -t 20'
```

HSN

```shell
./target/release/measurement \
    --events "mxp_n_dat_txflit_valid,mxp_e_dat_txflit_valid,mxp_s_dat_txflit_valid,mxp_w_dat_txflit_valid,mxp_p0_dat_txflit_valid,mxp_p1_dat_txflit_valid,01:rnid_txdat_flits,01:rnid_rxdat_flits" \
    --mesh-x 8 --mesh-y 6 \
    launch \
    --env 'FI_CXI_LLRING_MODE=never' \
    --binary cxi_read_bw --args '-n 50000 10.115.3.6'
```


### launch

### launch-multi