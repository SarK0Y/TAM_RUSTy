/*
##[source.crates-io]
##registry = "https://github.com/rust-lang/crates.io-index"
##replace-with='rsproxy'
##[source.rsproxy]
##registry="https://rsproxy.cn/crates.io-index"
##[registries.rsproxy]
##index = "https://rsproxy.cn/crates.io-index"
##[net]
##git-fetch-with-cli = true
[http]
proxy = "socks5h://127.0.0.1:9150" # HTTP proxy to use for HTTP requests (defaults to none)
                    # in libcurl format, e.g., "socks5h://host:port"
##timeout = 30        # Timeout for each HTTP request, in seconds
#cainfo = "cert.pem" # Path to Certificate Authority (CA) bundle (optional)
##check-revoke = false # Indicates whether SSL certs are checked for revocation
#low-speed-limit = 5 # Lower threshold for bytes/sec (10 = default, 0 = disabled)
##multiplexing = false # whether or not to use HTTP/2 multiplexing where possible
##rustflags="--registry cargo-io"
##[source.crates-io]
##replace-with = "vendored-sources"
[source.vendored-sources]
directory = "/home/evg/rust_vendors"
*/
