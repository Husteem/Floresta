// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use bitcoin::BlockHash;
use bitcoin::Network;
use clap::CommandFactory;
use clap::Parser;
use floresta_node::AssumeValidArg;

#[derive(Parser, Clone, Debug)]
#[command(
    author = "Davidson Souza",
    version = env!("GIT_DESCRIBE"),
    about = "florestad - a lightweight Bitcoin client",
    long_about = env!("LONG_VERSION"),
)]
pub struct Cli {
    #[arg(long, default_value_t = false)]
    /// Whether to disable DNS seeds
    pub disable_dns_seeds: bool,

    #[arg(short, long, value_name = "FILE")]
    /// Sets a custom config file
    pub config_file: Option<PathBuf>,

    #[arg(short, long, default_value_t=Network::Bitcoin)]
    /// Which network should we use
    pub network: Network,

    #[arg(short, long, default_value_t = false)]
    /// Turn debugging information on
    pub debug: bool,

    #[arg(long)]
    /// Option for saving log into data_Dir
    ///
    /// if set, log will be saved into $DATA_DIR/debug.log.
    pub log_to_file: bool,

    #[arg(long, value_name = "PATH")]
    /// Where should we store data. This is the directory where we'll store the chainstate,
    /// the wallet, the logs, the compact block filters, the Utreexo state, etc.
    /// Defaults to `~/.floresta`. The passed value should be an absolute path.
    pub data_dir: Option<PathBuf>,

    #[arg(long)]
    /// Whether Compact Block Filters should be disabled
    ///
    /// Those filters let you query for chain data after IBD, like wallet rescan,
    /// finding a utxo, finding specific tx_ids.
    /// Will cause less disk usage if disabled.
    pub no_cfilters: bool,

    #[arg(long, short, default_value = None, value_name = "address[:<port>]")]
    /// The url of a proxy we should open p2p connections through (e.g. 127.0.0.1:9050)
    pub proxy: Option<String>,

    #[arg(long, value_name = "XPUB")]
    /// Add an xpub to our wallet
    ///
    /// This option can be passed many times, and will accept any SLIP039-valid extended
    /// public key. You only need to pass this once, but there's no harm in passing it
    /// more than once. After you start florestad at least once, passing some xpub, florestad
    /// will follow the first 100 addresses derived from this xpub on each keychain  and
    /// cache any transactions where those addresses appear. You can use either the integrated
    /// json-rpc or electrum server to fetch an address's history, balance and utxos.
    /// Note: if `xpub` are added after IBD or when using `assume-utreexo`, related transactions
    /// may not be discovered. Use the JSON-RPC `rescanblockchain` to pick them up.
    pub wallet_xpub: Option<Vec<String>>,

    #[arg(long, value_name = "DESCRIPTOR")]
    /// Add an output descriptor to our wallet.
    ///
    /// This option can be passed multiple times, as long as each descriptor is valid.
    /// For each valid descriptor, the node will derive the first 100 addresses and cache any
    /// transactions related to those addresses.
    /// You can use the integrated JSON-RPC or Electrum server to fetch the transaction history,
    /// balance, and UTXOs for these addresses.
    /// Note: if `descriptors` are added after IBD or when using `assume-utreexo`, related transactions
    /// may not be discovered. Use the JSON-RPC `rescanblockchain` to pick them up.
    pub wallet_descriptor: Option<Vec<String>>,

    #[arg(long, value_name = "BLOCK_HASH|0", default_value = "hardcoded", value_parser = parse_assume_valid)]
    /// Assume that all blocks prior to and including this block have valid scripts.
    ///
    /// - default: use the hardcoded assume-valid value, reviewed by the Floresta developers
    /// - `--assume-valid <BLOCK_HASH>`: override with your own hash
    /// - `--assume-valid 0`: disable assume-valid and verify all scripts from genesis
    pub assume_valid: AssumeValidArg,

    #[arg(long, short, value_name = "address[:<port>]")]
    /// An address for the ZeroMQ server to listen to
    ///
    /// ZeroMQ is a lightweight message queue for Inter Process Communication. If you connect
    /// with this server, it'll push new blocks after we fully validate it.
    pub zmq_address: Option<String>,

    #[arg(long, value_name = "address[:<port>]")]
    /// A node to connect to. May be specified multiple times.
    ///
    /// If this option is provided, we'll connect **only** to the listed nodes. Each value
    /// should be an ipv4/ipv6/hostname address in the format `<address>[:<port>]`.
    pub connect: Vec<String>,

    #[arg(long, value_name = "address[:<port>]")]
    /// The address where our json-rpc server should listen to, in the format `<address>[:<port>]`
    pub rpc_address: Option<String>,

    #[arg(long, value_name = "HEIGHT")]
    /// Download block filters starting at this height. Negative numbers are relative to the current tip.
    pub filters_start_height: Option<i32>,

    #[arg(long, default_value_t = false)]
    /// Whether assume utreexo should be disabled.
    ///
    /// This option will disable assume utreexo. This feature allows the node to skip validation
    /// of historical blocks, making it ready-to-use in a shorter time than validating all blocks.
    /// Disabling it will cause the node to validate all blocks from genesis.
    ///
    /// By default, floresta will still validate those blocks and transactions on background until
    /// we reach the assumed tip. If you want to stop this behavior, use the --no-backfill flag.
    pub no_assume_utreexo: bool,

    #[arg(long, value_name = "address[:<port>]")]
    /// The address where the Electrum Server should listen to, in the format `<address>[:<port>]`
    pub electrum_address: Option<String>,

    #[arg(long, default_value_t = false)]
    /// Whether to enable the Electrum TLS server.
    pub enable_electrum_tls: bool,

    #[arg(long, value_name = "address[:<port>]")]
    /// The address where the Electrum TLS Server should listen to, in the format `<address>[:<port>]`
    pub electrum_address_tls: Option<String>,

    #[arg(long, default_value_t = false)]
    /// Whether to generate a self-signed TLS certificate on start.
    ///
    /// This option may conflict with other TLS-related flags. Read the TLS section on `doc/run.md` for more information.
    pub generate_cert: bool,

    #[arg(long, value_name = "PATH")]
    /// TLS private key path (defaults to `{data_dir}/tls/key.pem`).
    /// It must be PKCS#8-encoded. You can use `openssl` to generate it:
    ///
    /// ```shell
    /// openssl genpkey -algorithm RSA -out key.pem -pkeyopt rsa_keygen_bits:2048
    /// ```
    pub tls_key_path: Option<PathBuf>,

    #[arg(long, value_name = "PATH")]
    /// TLS certificate path (defaults to `{data_dir}/tls/cert.pem`).
    /// It must be PKCS#8-encoded. You can use `openssl` to generate it from a PKCS#8-encoded private key:
    ///
    /// ```shell
    /// openssl req -x509 -new -key key.pem -out cert.pem -days 365 -subj "/CN=localhost"
    /// ```
    pub tls_cert_path: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    /// Whether we should try to connect with peers using the old, unencrypted V1 P2P protocol,
    /// if we can't make a V2 connection.
    pub allow_v1_fallback: bool,

    #[cfg(unix)]
    #[arg(long, default_value = "false")]
    /// Run florestad as a daemon.
    pub daemon: bool,

    #[cfg(unix)]
    #[arg(long, value_name = "PID_FILE")]
    /// File to write `florestad`'s PID to.
    ///
    /// In case you're using the daemon option, and you want to know the process ID, you can
    /// write it to a file. This option should be an absolute path to a file. Usually, you'd
    /// write it to `$DATA_DIR/florestad.pid`.
    pub pid_file: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    /// Whether backfill should be disabled
    ///
    /// If we assumeutreexo or use pow fraud proofs, you have the option to download and validate
    /// the blocks that were skipped. This will take a long time, but will run on the background
    /// and won't affect the node's operation. You may notice that this will take a lot of CPU
    /// and bandwidth to run.
    ///
    /// The default behavior is verifying the blocks that were skipped during node startup.
    /// This will run in the background and wont't affect node's operation. However,
    /// to disable backfilling, run floresta using this flag.
    pub no_backfill: bool,

    #[arg(long, value_name = "SIZE")]
    /// The maximum number of block indexes we can store in our database (default: 10,000,000)
    pub block_index_size: Option<usize>,

    #[arg(long, value_name = "SIZE")]
    /// The maximum number of block headers in the main chain we can store (default: 10,000,000)
    pub headers_file_size: Option<usize>,

    #[arg(long, value_name = "SIZE")]
    /// The maximum number of alternative fork headers we can track (default: 10,000)
    pub fork_file_size: Option<usize>,
}

impl Cli {
    /// Validate arguments passed to [`Cli`].
    ///
    /// Checks:
    ///   - If `--pid-file` is passed, `--daemon` must also be passed.
    pub fn validate(&self) {
        if let Err(err) = self.check_validity() {
            err.exit();
        }
    }

    fn check_validity(&self) -> Result<(), clap::Error> {
        #[cfg(unix)]
        if self.pid_file.is_some() && !self.daemon {
            return Err(Self::command().error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "--pid-file requires that --daemon be set",
            ));
        }

        if let Some(size) = self.block_index_size {
            if size < 1000 {
                return Err(Self::command().error(
                    clap::error::ErrorKind::ValueValidation,
                    "--block-index-size must be at least 1,000",
                ));
            }
        }

        if let Some(size) = self.headers_file_size {
            if size < 1000 {
                return Err(Self::command().error(
                    clap::error::ErrorKind::ValueValidation,
                    "--headers-file-size must be at least 1,000",
                ));
            }
        }

        if let Some(size) = self.fork_file_size {
            if size < 10 {
                return Err(Self::command().error(
                    clap::error::ErrorKind::ValueValidation,
                    "--fork-file-size must be at least 10",
                ));
            }
        }

        Ok(())
    }
}

fn parse_assume_valid(s: &str) -> Result<AssumeValidArg, String> {
    match s {
        "0" => Ok(AssumeValidArg::Disabled),
        "hardcoded" => Ok(AssumeValidArg::Hardcoded),
        other => other
            .parse::<BlockHash>()
            .map(AssumeValidArg::UserInput)
            .map_err(|e| format!("expected 0 or a block hash, got '{other}': {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_validation_bounds() {
        // Base case with valid bounds
        let base_cli = Cli {
            disable_dns_seeds: false,
            config_file: None,
            network: Network::Bitcoin,
            debug: false,
            log_to_file: false,
            data_dir: None,
            no_cfilters: false,
            proxy: None,
            wallet_xpub: None,
            wallet_descriptor: None,
            assume_valid: AssumeValidArg::Hardcoded,
            zmq_address: None,
            connect: vec![],
            rpc_address: None,
            filters_start_height: None,
            no_assume_utreexo: false,
            electrum_address: None,
            enable_electrum_tls: false,
            electrum_address_tls: None,
            generate_cert: false,
            tls_key_path: None,
            tls_cert_path: None,
            allow_v1_fallback: false,
            #[cfg(unix)]
            daemon: false,
            #[cfg(unix)]
            pid_file: None,
            no_backfill: false,
            block_index_size: Some(1000),
            headers_file_size: Some(1000),
            fork_file_size: Some(10),
        };

        // Assert valid base case passes
        assert!(base_cli.check_validity().is_ok());

        // Test invalid block_index_size (under 1000)
        let mut cli = base_cli.clone();
        cli.block_index_size = Some(999);
        let res = cli.check_validity();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().kind(),
            clap::error::ErrorKind::ValueValidation
        );

        // Test invalid headers_file_size (under 1000)
        let mut cli = base_cli.clone();
        cli.headers_file_size = Some(999);
        let res = cli.check_validity();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().kind(),
            clap::error::ErrorKind::ValueValidation
        );

        // Test invalid fork_file_size (under 10)
        let mut cli = base_cli.clone();
        cli.fork_file_size = Some(9);
        let res = cli.check_validity();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().kind(),
            clap::error::ErrorKind::ValueValidation
        );

        // Test valid boundary values
        let mut cli = base_cli.clone();
        cli.block_index_size = Some(1000);
        cli.headers_file_size = Some(1000);
        cli.fork_file_size = Some(10);
        assert!(cli.check_validity().is_ok());

        // Test valid values far above bounds
        let mut cli = base_cli.clone();
        cli.block_index_size = Some(10000000);
        cli.headers_file_size = Some(10000000);
        cli.fork_file_size = Some(10000);
        assert!(cli.check_validity().is_ok());
    }
}
