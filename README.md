### Prerequisites

Before using this tool, you must generate a `protoboards.txt` file containing the full enumeration of valid 2048 board placements.

This can be done using the companion project:

- 🔧 [`ProtoboardEnum_2048`](https://github.com/jwm-dev/ProtoboardEnum_2048)

```bash
# Generate protoboards.txt
git clone https://github.com/jwm-dev/ProtoboardEnum_2048
cd ProtoboardEnum_2048
cargo run --release
```

**Then move or link it into BoardViewer_2048's working directory**
