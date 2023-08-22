/// Params defines the EVM module parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// evm denom represents the token denomination used to run the EVM state
    /// transitions.
    #[prost(string, tag = "1")]
    pub evm_denom: ::prost::alloc::string::String,
    /// enable create toggles state transitions that use the vm.Create function
    #[prost(bool, tag = "2")]
    pub enable_create: bool,
    /// enable call toggles state transitions that use the vm.Call function
    #[prost(bool, tag = "3")]
    pub enable_call: bool,
    /// extra eips defines the additional EIPs for the vm.Config
    #[prost(int64, repeated, packed = "false", tag = "4")]
    pub extra_eips: ::prost::alloc::vec::Vec<i64>,
    /// chain config defines the EVM chain configuration parameters
    #[prost(message, optional, tag = "5")]
    pub chain_config: ::core::option::Option<ChainConfig>,
    /// Allow unprotected transactions defines if replay-protected (i.e non EIP155
    /// signed) transactions can be executed on the state machine.
    #[prost(bool, tag = "6")]
    pub allow_unprotected_txs: bool,
}
/// ChainConfig defines the Ethereum ChainConfig parameters using *sdk.Int values
/// instead of *big.Int.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainConfig {
    /// Homestead switch block (nil no fork, 0 = already homestead)
    #[prost(string, tag = "1")]
    pub homestead_block: ::prost::alloc::string::String,
    /// TheDAO hard-fork switch block (nil no fork)
    #[prost(string, tag = "2")]
    pub dao_fork_block: ::prost::alloc::string::String,
    /// Whether the nodes supports or opposes the DAO hard-fork
    #[prost(bool, tag = "3")]
    pub dao_fork_support: bool,
    /// EIP150 implements the Gas price changes
    /// (<https://github.com/ethereum/EIPs/issues/150>) EIP150 HF block (nil no fork)
    #[prost(string, tag = "4")]
    pub eip150_block: ::prost::alloc::string::String,
    /// EIP150 HF hash (needed for header only clients as only gas pricing changed)
    #[prost(string, tag = "5")]
    pub eip150_hash: ::prost::alloc::string::String,
    /// EIP155Block HF block
    #[prost(string, tag = "6")]
    pub eip155_block: ::prost::alloc::string::String,
    /// EIP158 HF block
    #[prost(string, tag = "7")]
    pub eip158_block: ::prost::alloc::string::String,
    /// Byzantium switch block (nil no fork, 0 = already on byzantium)
    #[prost(string, tag = "8")]
    pub byzantium_block: ::prost::alloc::string::String,
    /// Constantinople switch block (nil no fork, 0 = already activated)
    #[prost(string, tag = "9")]
    pub constantinople_block: ::prost::alloc::string::String,
    /// Petersburg switch block (nil same as Constantinople)
    #[prost(string, tag = "10")]
    pub petersburg_block: ::prost::alloc::string::String,
    /// Istanbul switch block (nil no fork, 0 = already on istanbul)
    #[prost(string, tag = "11")]
    pub istanbul_block: ::prost::alloc::string::String,
    /// Eip-2384 (bomb delay) switch block (nil no fork, 0 = already activated)
    #[prost(string, tag = "12")]
    pub muir_glacier_block: ::prost::alloc::string::String,
    /// Berlin switch block (nil = no fork, 0 = already on berlin)
    #[prost(string, tag = "13")]
    pub berlin_block: ::prost::alloc::string::String,
    /// London switch block (nil = no fork, 0 = already on london)
    #[prost(string, tag = "17")]
    pub london_block: ::prost::alloc::string::String,
    /// Eip-4345 (bomb delay) switch block (nil = no fork, 0 = already activated)
    #[prost(string, tag = "18")]
    pub arrow_glacier_block: ::prost::alloc::string::String,
    ///   EIP-5133 (bomb delay) switch block (nil = no fork, 0 = already activated)
    #[prost(string, tag = "20")]
    pub gray_glacier_block: ::prost::alloc::string::String,
    /// Virtual fork after The Merge to use as a network splitter
    #[prost(string, tag = "21")]
    pub merge_netsplit_block: ::prost::alloc::string::String,
}
/// State represents a single Storage key value pair item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// TransactionLogs define the logs generated from a transaction execution
/// with a given hash. It it used for import/export data as transactions are not
/// persisted on blockchain state after an upgrade.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionLogs {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
/// Log represents an protobuf compatible Ethereum Log that defines a contract
/// log event. These events are generated by the LOG opcode and stored/indexed by
/// the node.
///
/// Consensus fields:
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    /// address of the contract that generated the event
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// list of topics provided by the contract.
    #[prost(string, repeated, tag = "2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// supplied by the contract, usually ABI-encoded
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// block in which the transaction was included
    #[prost(uint64, tag = "4")]
    pub block_number: u64,
    /// hash of the transaction
    #[prost(string, tag = "5")]
    pub tx_hash: ::prost::alloc::string::String,
    /// index of the transaction in the block
    #[prost(uint64, tag = "6")]
    pub tx_index: u64,
    /// hash of the block in which the transaction was included
    #[prost(string, tag = "7")]
    pub block_hash: ::prost::alloc::string::String,
    /// index of the log in the block
    #[prost(uint64, tag = "8")]
    pub index: u64,
    /// The Removed field is true if this log was reverted due to a chain
    /// reorganisation. You must pay attention to this field if you receive logs
    /// through a filter query.
    #[prost(bool, tag = "9")]
    pub removed: bool,
}
/// TxResult stores results of Tx execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    /// contract_address contains the ethereum address of the created contract (if
    /// any). If the state transition is an evm.Call, the contract address will be
    /// empty.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// bloom represents the bloom filter bytes
    #[prost(bytes = "vec", tag = "2")]
    pub bloom: ::prost::alloc::vec::Vec<u8>,
    /// tx_logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, optional, tag = "3")]
    pub tx_logs: ::core::option::Option<TransactionLogs>,
    /// ret defines the bytes from the execution.
    #[prost(bytes = "vec", tag = "4")]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// reverted flag is set to true when the call has been reverted
    #[prost(bool, tag = "5")]
    pub reverted: bool,
    /// gas_used notes the amount of gas consumed while execution
    #[prost(uint64, tag = "6")]
    pub gas_used: u64,
}
/// AccessTuple is the element type of an access list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    /// hex formatted ethereum address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// hex formatted hashes of the storage keys
    #[prost(string, repeated, tag = "2")]
    pub storage_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TraceConfig holds extra parameters to trace functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceConfig {
    /// custom javascript tracer
    #[prost(string, tag = "1")]
    pub tracer: ::prost::alloc::string::String,
    /// overrides the default timeout of 5 seconds for JavaScript-based tracing
    /// calls
    #[prost(string, tag = "2")]
    pub timeout: ::prost::alloc::string::String,
    /// number of blocks the tracer is willing to go back
    #[prost(uint64, tag = "3")]
    pub reexec: u64,
    /// disable stack capture
    #[prost(bool, tag = "5")]
    pub disable_stack: bool,
    /// disable storage capture
    #[prost(bool, tag = "6")]
    pub disable_storage: bool,
    /// print output during capture end
    #[prost(bool, tag = "8")]
    pub debug: bool,
    /// maximum length of output, but zero means unlimited
    #[prost(int32, tag = "9")]
    pub limit: i32,
    /// Chain overrides, can be used to execute a trace using future fork rules
    #[prost(message, optional, tag = "10")]
    pub overrides: ::core::option::Option<ChainConfig>,
    /// enable memory capture
    #[prost(bool, tag = "11")]
    pub enable_memory: bool,
    /// enable return data capture
    #[prost(bool, tag = "12")]
    pub enable_return_data: bool,
}
/// GenesisState defines the evm module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// accounts is an array containing the ethereum genesis accounts.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<GenesisAccount>,
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// GenesisAccount defines an account to be initialized in the genesis state.
/// Its main difference between with Geth's GenesisAccount is that it uses a
/// custom storage type and that it doesn't contain the private key field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisAccount {
    /// address defines an ethereum hex formated address of an account
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// code defines the hex bytes of the account code.
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// storage defines the set of state key values for the account.
    #[prost(message, repeated, tag = "3")]
    pub storage: ::prost::alloc::vec::Vec<State>,
}
/// MsgEthereumTx encapsulates an Ethereum transaction as an SDK message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEthereumTx {
    /// inner transaction data
    ///
    /// caches
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<::prost_types::Any>,
    /// DEPRECATED: encoded storage size of the transaction
    #[prost(double, tag = "2")]
    pub size: f64,
    /// transaction hash in hex format
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
    /// ethereum signer address in hex format. This address value is checked
    /// against the address derived from the signature (V, R, S) using the
    /// secp256k1 elliptic curve
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
}
/// LegacyTx is the transaction data of regular Ethereum transactions.
/// NOTE: All non-protected transactions (i.e non EIP155 signed) will fail if the
/// AllowUnprotectedTxs parameter is disabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyTx {
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    /// gas price defines the value for each gas unit
    #[prost(string, tag = "2")]
    pub gas_price: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag = "3")]
    pub gas: u64,
    /// hex formatted address of the recipient
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// value defines the unsigned integer value of the transaction amount.
    #[prost(string, tag = "5")]
    pub value: ::prost::alloc::string::String,
    /// input defines the data payload bytes of the transaction.
    #[prost(bytes = "vec", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// v defines the signature value
    #[prost(bytes = "vec", tag = "7")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// r defines the signature value
    #[prost(bytes = "vec", tag = "8")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// s define the signature value
    #[prost(bytes = "vec", tag = "9")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
/// AccessListTx is the data of EIP-2930 access list transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessListTx {
    /// destination EVM chain ID
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    /// gas price defines the value for each gas unit
    #[prost(string, tag = "3")]
    pub gas_price: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag = "4")]
    pub gas: u64,
    /// hex formatted address of the recipient
    #[prost(string, tag = "5")]
    pub to: ::prost::alloc::string::String,
    /// value defines the unsigned integer value of the transaction amount.
    #[prost(string, tag = "6")]
    pub value: ::prost::alloc::string::String,
    /// input defines the data payload bytes of the transaction.
    #[prost(bytes = "vec", tag = "7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "8")]
    pub accesses: ::prost::alloc::vec::Vec<AccessTuple>,
    /// v defines the signature value
    #[prost(bytes = "vec", tag = "9")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// r defines the signature value
    #[prost(bytes = "vec", tag = "10")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// s define the signature value
    #[prost(bytes = "vec", tag = "11")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
/// DynamicFeeTx is the data of EIP-1559 dinamic fee transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFeeTx {
    /// destination EVM chain ID
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    /// gas tip cap defines the max value for the gas tip
    #[prost(string, tag = "3")]
    pub gas_tip_cap: ::prost::alloc::string::String,
    /// gas fee cap defines the max value for the gas fee
    #[prost(string, tag = "4")]
    pub gas_fee_cap: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag = "5")]
    pub gas: u64,
    /// hex formatted address of the recipient
    #[prost(string, tag = "6")]
    pub to: ::prost::alloc::string::String,
    /// value defines the the transaction amount.
    #[prost(string, tag = "7")]
    pub value: ::prost::alloc::string::String,
    /// input defines the data payload bytes of the transaction.
    #[prost(bytes = "vec", tag = "8")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "9")]
    pub accesses: ::prost::alloc::vec::Vec<AccessTuple>,
    /// v defines the signature value
    #[prost(bytes = "vec", tag = "10")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// r defines the signature value
    #[prost(bytes = "vec", tag = "11")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// s define the signature value
    #[prost(bytes = "vec", tag = "12")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionOptionsEthereumTx {}
/// MsgEthereumTxResponse defines the Msg/EthereumTx response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEthereumTxResponse {
    /// ethereum transaction hash in hex format. This hash differs from the
    /// Tendermint sha256 hash of the transaction bytes. See
    /// <https://github.com/tendermint/tendermint/issues/6539> for reference
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// returned data from evm function (result or data supplied with revert
    /// opcode)
    #[prost(bytes = "vec", tag = "3")]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// vm error is the error returned by vm execution
    #[prost(string, tag = "4")]
    pub vm_error: ::prost::alloc::string::String,
    /// gas consumed by the transaction
    #[prost(uint64, tag = "5")]
    pub gas_used: u64,
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the evm Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// EthereumTx defines a method submitting Ethereum transactions.
        pub async fn ethereum_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgEthereumTx>,
        ) -> std::result::Result<
            tonic::Response<super::MsgEthereumTxResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Msg/EthereumTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Msg", "EthereumTx"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
    /// code hash is the hex-formatted code bytes from the EOA.
    #[prost(string, tag = "2")]
    pub code_hash: ::prost::alloc::string::String,
    /// nonce is the account's sequence number.
    #[prost(uint64, tag = "3")]
    pub nonce: u64,
}
/// QueryCosmosAccountRequest is the request type for the Query/CosmosAccount RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCosmosAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCosmosAccountResponse is the response type for the Query/CosmosAccount
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCosmosAccountResponse {
    /// cosmos_address is the cosmos address of the account.
    #[prost(string, tag = "1")]
    pub cosmos_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    /// account_number is the account numbert
    #[prost(uint64, tag = "3")]
    pub account_number: u64,
}
/// QueryValidatorAccountRequest is the request type for the
/// Query/ValidatorAccount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorAccountRequest {
    /// cons_address is the validator cons address to query the account for.
    #[prost(string, tag = "1")]
    pub cons_address: ::prost::alloc::string::String,
}
/// QueryValidatorAccountResponse is the response type for the
/// Query/ValidatorAccount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorAccountResponse {
    /// account_address is the cosmos address of the account in bech32 format.
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    /// account_number is the account number
    #[prost(uint64, tag = "3")]
    pub account_number: u64,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// address is the ethereum hex address to query the balance for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
}
/// QueryStorageRequest is the request type for the Query/Storage RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageRequest {
    /// / address is the ethereum hex address to query the storage state for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// key defines the key of the storage state
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// QueryStorageResponse is the response type for the Query/Storage RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageResponse {
    /// key defines the storage state value hash associated with the given key.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// QueryCodeRequest is the request type for the Query/Code RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeRequest {
    /// address is the ethereum hex address to query the code for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCodeResponse is the response type for the Query/Code RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeResponse {
    /// code represents the code bytes from an ethereum address.
    #[prost(bytes = "vec", tag = "1")]
    pub code: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTxLogsRequest is the request type for the Query/TxLogs RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTxLogsRequest {
    /// hash is the ethereum transaction hex hash to query the logs for.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryTxLogs is the response type for the Query/TxLogs RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTxLogsResponse {
    /// logs represents the ethereum logs generated from the given transaction.
    #[prost(message, repeated, tag = "1")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryParamsRequest defines the request type for querying x/evm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/evm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params define the evm module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// EthCallRequest defines EthCall request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthCallRequest {
    /// same json format as the json rpc api.
    #[prost(bytes = "vec", tag = "1")]
    pub args: ::prost::alloc::vec::Vec<u8>,
    /// the default gas cap to be used
    #[prost(uint64, tag = "2")]
    pub gas_cap: u64,
}
/// EstimateGasResponse defines EstimateGas response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateGasResponse {
    /// the estimated gas
    #[prost(uint64, tag = "1")]
    pub gas: u64,
}
/// QueryTraceTxRequest defines TraceTx request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceTxRequest {
    /// msgEthereumTx for the requested transaction
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgEthereumTx>,
    /// TraceConfig holds extra parameters to trace functions.
    #[prost(message, optional, tag = "3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// the predecessor transactions included in the same block
    /// need to be replayed first to get correct context for tracing.
    #[prost(message, repeated, tag = "4")]
    pub predecessors: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// block number of requested transaction
    #[prost(int64, tag = "5")]
    pub block_number: i64,
    /// block hex hash of requested transaction
    #[prost(string, tag = "6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block time of requested transaction
    #[prost(message, optional, tag = "7")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// QueryTraceTxResponse defines TraceTx response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceTxResponse {
    /// response serialized in bytes
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTraceBlockRequest defines TraceTx request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceBlockRequest {
    /// txs messages in the block
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// TraceConfig holds extra parameters to trace functions.
    #[prost(message, optional, tag = "3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// block number
    #[prost(int64, tag = "5")]
    pub block_number: i64,
    /// block hex hash
    #[prost(string, tag = "6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block time
    #[prost(message, optional, tag = "7")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// QueryTraceBlockResponse defines TraceBlock response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceBlockResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryBaseFeeRequest defines the request type for querying the EIP1559 base
/// fee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeRequest {}
/// BaseFeeResponse returns the EIP1559 base fee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeResponse {
    #[prost(string, tag = "1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Account queries an Ethereum account.
        pub async fn account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Account",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Account"));
            self.inner.unary(req, path, codec).await
        }
        /// CosmosAccount queries an Ethereum account's Cosmos Address.
        pub async fn cosmos_account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCosmosAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCosmosAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/CosmosAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "CosmosAccount"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorAccount queries an Ethereum account's from a validator consensus
        /// Address.
        pub async fn validator_account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/ValidatorAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "ValidatorAccount"));
            self.inner.unary(req, path, codec).await
        }
        /// Balance queries the balance of a the EVM denomination for a single
        /// EthAccount.
        pub async fn balance(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBalanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Balance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Balance"));
            self.inner.unary(req, path, codec).await
        }
        /// Storage queries the balance of all coins for a single account.
        pub async fn storage(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryStorageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryStorageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Storage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Storage"));
            self.inner.unary(req, path, codec).await
        }
        /// Code queries the balance of all coins for a single account.
        pub async fn code(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Code",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Code"));
            self.inner.unary(req, path, codec).await
        }
        /// Params queries the parameters of x/evm module.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        /// EthCall implements the `eth_call` rpc api
        pub async fn eth_call(
            &mut self,
            request: impl tonic::IntoRequest<super::EthCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgEthereumTxResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/EthCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "EthCall"));
            self.inner.unary(req, path, codec).await
        }
        /// EstimateGas implements the `eth_estimateGas` rpc api
        pub async fn estimate_gas(
            &mut self,
            request: impl tonic::IntoRequest<super::EthCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateGasResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/EstimateGas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "EstimateGas"));
            self.inner.unary(req, path, codec).await
        }
        /// TraceTx implements the `debug_traceTransaction` rpc api
        pub async fn trace_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraceTxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraceTxResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/TraceTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "TraceTx"));
            self.inner.unary(req, path, codec).await
        }
        /// TraceBlock implements the `debug_traceBlockByNumber` and `debug_traceBlockByHash` rpc api
        pub async fn trace_block(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraceBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraceBlockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/TraceBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "TraceBlock"));
            self.inner.unary(req, path, codec).await
        }
        /// BaseFee queries the base fee of the parent block of the current block,
        /// it's similar to feemarket module's method, but also checks london hardfork status.
        pub async fn base_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBaseFeeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBaseFeeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/BaseFee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "BaseFee"));
            self.inner.unary(req, path, codec).await
        }
    }
}
