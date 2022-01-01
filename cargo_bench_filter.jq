map(select(.type == "bench")) 
  | map( (.name | capture("day(?<day>\\d{1,2})::tests::bench_(?<part>.+)")) + {us: (.median / 1000), dev: (.deviation / 1000), tpt: (if .mib_per_second? then "\(.mib_per_second) MiB/s" else "" end)} )
  | sort_by(.day | tonumber)
  | map("| `Day \(.day)` | `\(.part)` | \(.us) ± \(.dev) | \(.tpt) |")
  | ["| Day | Part | Mean [µs] | Throughput |", "|:---|:---|---:|---:|"] + .
  | .[]