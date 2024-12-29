
# Wholesum network `segmentor` CLI

## Overview

Wholesum network is a p2p verifiable computing network `tailored for ETH L2 sequencer proving`. It builds on top of [Risc0](https://risczero.com/), [Libp2p](https://libp2p.io), and decentralized storage options like [Swarm](https://ethswarm.org) and Filecoin to facilitate verifiable computing at scale. The design of the network follows a p2p parallel proving scheme where Risc0 jobs are passed around, proved, and finally combined into a final proof ready for L1 verification.

### Prerequisites

You would need to get certain environments ready for the client to function properly.

### Risc0 

To install Risc0, please follow the following [guide](https://github.com/risc0/risc0?tab=readme-ov-file#getting-started).

#### Docker

Docker runtime is needed as it is used to run `Risc0` containers. This awesome [guide](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04) from DigitalOcean is helpful in this regard.

## USAGE

<pre>
Usage: segmentor [OPTIONS] --elf [ELF] --out-path [OUT_PATH]

Options:
  -e, --elf &lt;ELF&lt;            
  -p, --po2 &lt;PO2&lt;            segment limit size [default: 20]
  -o, --out-path &lt;OUT_PATH&gt;  path to write segment blobs to
  -h, --help                 Print help
  -V, --version              Print version
</pre>
