pub use balances::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod balances {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("availableBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("availableBalance"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceHeld"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceHeld"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balances"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawOnBehalf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawOnBehalf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Withdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdrawal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignature",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureS",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAccountNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidAccountNonce",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BALANCES_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0`\x02U4\x80\x15a\0\x15W`\0\x80\xFD[Pa\t\x1D\x80a\0%`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0[W\x80cp\xA0\x821\x14a\0\xD2W\x80c~\xCE\xBE\0\x14a\x01\x08W\x80c\xAB/\x0EQ\x14a\x01>W\x80c\xF3\xFE\xF3\xA3\x14a\x01FW`\0\x80\xFD[\x80c\x059\x1E\x8F\x14a\0\x82W\x80c'\xE25\xE3\x14a\0\x97W\x80cbLk\xE7\x14a\0\xC9W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x07\xBDV[a\x01YV[\0[a\0\xB7a\0\xA56`\x04a\x08(V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB7`\x02T\x81V[a\0\xB7a\0\xE06`\x04a\x08(V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\0\xB7a\x01\x166`\x04a\x08(V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\0\xB7a\x03!V[a\0\x95a\x01T6`\x04a\x08LV[a\x036V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T`\x01\x81\x01\x90\x91U\x81Q\x80\x84\x01\x95\x90\x95R\x84\x82\x01R3``\x85\x01R\x93\x89\x16`\x80\x84\x01R`\xA0\x80\x84\x01\x89\x90R\x84Q\x80\x85\x03\x90\x91\x01\x81R`\xC0\x90\x93\x01\x90\x93R\x81Q\x91\x90\x92\x01 \x90a\x01\xD2\x82\x86\x86\x86a\x03\x96V[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x02nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x02\x81\x883g\x01cEx]\x8A\0\0a\x03\xC4V[a\x02\x9D\x88\x88a\x02\x98g\x01cEx]\x8A\0\0\x8Aa\x08xV[a\x03\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x88a\x02\xE8g\x01cEx]\x8A\0\0\x8Aa\x08xV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[`\0`\x02TGa\x031\x91\x90a\x08xV[\x90P\x90V[a\x03A3\x83\x83a\x03\xC4V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R3\x91\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x80`\0\x80a\x03\xA8\x88\x88\x88\x88a\x05\x96V[\x92P\x92P\x92Pa\x03\xB8\x82\x82a\x06\x90V[P\x90\x96\x95PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x81\x11\x15a\x04SW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInsufficient balance\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x83\x92\x90a\x04\x88\x90\x84\x90a\x08xV[\x92PP\x81\x90UP\x80`\x02`\0\x82\x82Ta\x04\xA1\x91\x90a\x08xV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x03a\x04\xF9Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 U[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa\x05\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FWithdrawal failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02eV[PPPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x05\xD1WP`\0\x91P`\x03\x90P\x82a\x06\x86V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x06%W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x06|WP`\0\x92P`\x01\x91P\x82\x90Pa\x06\x86V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x06\xA4Wa\x06\xA4a\x08\xB8V[\x03a\x06\xADWPPV[`\x01\x82`\x03\x81\x11\x15a\x06\xC1Wa\x06\xC1a\x08\xB8V[\x03a\x06\xF8W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x07\x0CWa\x07\x0Ca\x08\xB8V[\x03a\x07FW`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x02eV[`\x03\x82`\x03\x81\x11\x15a\x07ZWa\x07Za\x08\xB8V[\x03a\x07\x94W`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x02eV[PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xBAW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x07\xD6W`\0\x80\xFD[\x865a\x07\xE1\x81a\x07\x98V[\x95P` \x87\x015a\x07\xF1\x81a\x07\x98V[\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a\x08\x0EW`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08:W`\0\x80\xFD[\x815a\x08E\x81a\x07\x98V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08_W`\0\x80\xFD[\x825a\x08j\x81a\x07\x98V[\x94` \x93\x90\x93\x015\x93PPPV[\x81\x81\x03\x81\x81\x11\x15a\x08\xB2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA8\xB9\xD1\x96\x98;~\xDA\x14\x87-#\xB4\xA1\x10\xE3\xEF\x96\xA3&\xDB\xDA\xEAnl\xF3V0\xAA\xA6[\xF5dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static BALANCES_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0[W\x80cp\xA0\x821\x14a\0\xD2W\x80c~\xCE\xBE\0\x14a\x01\x08W\x80c\xAB/\x0EQ\x14a\x01>W\x80c\xF3\xFE\xF3\xA3\x14a\x01FW`\0\x80\xFD[\x80c\x059\x1E\x8F\x14a\0\x82W\x80c'\xE25\xE3\x14a\0\x97W\x80cbLk\xE7\x14a\0\xC9W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x07\xBDV[a\x01YV[\0[a\0\xB7a\0\xA56`\x04a\x08(V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB7`\x02T\x81V[a\0\xB7a\0\xE06`\x04a\x08(V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\0\xB7a\x01\x166`\x04a\x08(V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\0\xB7a\x03!V[a\0\x95a\x01T6`\x04a\x08LV[a\x036V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x80T`\x01\x81\x01\x90\x91U\x81Q\x80\x84\x01\x95\x90\x95R\x84\x82\x01R3``\x85\x01R\x93\x89\x16`\x80\x84\x01R`\xA0\x80\x84\x01\x89\x90R\x84Q\x80\x85\x03\x90\x91\x01\x81R`\xC0\x90\x93\x01\x90\x93R\x81Q\x91\x90\x92\x01 \x90a\x01\xD2\x82\x86\x86\x86a\x03\x96V[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x02nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x02\x81\x883g\x01cEx]\x8A\0\0a\x03\xC4V[a\x02\x9D\x88\x88a\x02\x98g\x01cEx]\x8A\0\0\x8Aa\x08xV[a\x03\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x88a\x02\xE8g\x01cEx]\x8A\0\0\x8Aa\x08xV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[`\0`\x02TGa\x031\x91\x90a\x08xV[\x90P\x90V[a\x03A3\x83\x83a\x03\xC4V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R3\x91\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x80`\0\x80a\x03\xA8\x88\x88\x88\x88a\x05\x96V[\x92P\x92P\x92Pa\x03\xB8\x82\x82a\x06\x90V[P\x90\x96\x95PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x81\x11\x15a\x04SW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInsufficient balance\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x83\x92\x90a\x04\x88\x90\x84\x90a\x08xV[\x92PP\x81\x90UP\x80`\x02`\0\x82\x82Ta\x04\xA1\x91\x90a\x08xV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x03a\x04\xF9Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 U[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa\x05\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FWithdrawal failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02eV[PPPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x05\xD1WP`\0\x91P`\x03\x90P\x82a\x06\x86V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x06%W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x06|WP`\0\x92P`\x01\x91P\x82\x90Pa\x06\x86V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x06\xA4Wa\x06\xA4a\x08\xB8V[\x03a\x06\xADWPPV[`\x01\x82`\x03\x81\x11\x15a\x06\xC1Wa\x06\xC1a\x08\xB8V[\x03a\x06\xF8W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x07\x0CWa\x07\x0Ca\x08\xB8V[\x03a\x07FW`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x02eV[`\x03\x82`\x03\x81\x11\x15a\x07ZWa\x07Za\x08\xB8V[\x03a\x07\x94W`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x02eV[PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xBAW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x07\xD6W`\0\x80\xFD[\x865a\x07\xE1\x81a\x07\x98V[\x95P` \x87\x015a\x07\xF1\x81a\x07\x98V[\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a\x08\x0EW`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08:W`\0\x80\xFD[\x815a\x08E\x81a\x07\x98V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08_W`\0\x80\xFD[\x825a\x08j\x81a\x07\x98V[\x94` \x93\x90\x93\x015\x93PPPV[\x81\x81\x03\x81\x81\x11\x15a\x08\xB2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA8\xB9\xD1\x96\x98;~\xDA\x14\x87-#\xB4\xA1\x10\xE3\xEF\x96\xA3&\xDB\xDA\xEAnl\xF3V0\xAA\xA6[\xF5dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static BALANCES_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Balances<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Balances<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Balances<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Balances<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Balances<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Balances)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Balances<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BALANCES_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                BALANCES_ABI.clone(),
                BALANCES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `availableBalance` (0xab2f0e51) function
        pub fn available_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([171, 47, 14, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceHeld` (0x624c6be7) function
        pub fn balance_held(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 76, 107, 231], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xf3fef3a3) function
        pub fn withdraw(
            &self,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 254, 243, 163], (target, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawOnBehalf` (0x05391e8f) function
        pub fn withdraw_on_behalf(
            &self,
            account: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 57, 30, 143], (account, target, value, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Withdrawal` event
        pub fn withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Balances<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ECDSAInvalidSignature` with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ECDSAInvalidSignature", abi = "ECDSAInvalidSignature()")]
    pub struct ECDSAInvalidSignature;
    ///Custom Error type `ECDSAInvalidSignatureLength` with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ECDSAInvalidSignatureLength",
        abi = "ECDSAInvalidSignatureLength(uint256)"
    )]
    pub struct ECDSAInvalidSignatureLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ECDSAInvalidSignatureS` with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ECDSAInvalidSignatureS", abi = "ECDSAInvalidSignatureS(bytes32)")]
    pub struct ECDSAInvalidSignatureS {
        pub s: [u8; 32],
    }
    ///Custom Error type `InvalidAccountNonce` with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidAccountNonce",
        abi = "InvalidAccountNonce(address,uint256)"
    )]
    pub struct InvalidAccountNonce {
        pub account: ::ethers::core::types::Address,
        pub current_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BalancesErrors {
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        InvalidAccountNonce(InvalidAccountNonce),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BalancesErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <ECDSAInvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ECDSAInvalidSignature(decoded));
            }
            if let Ok(decoded)
                = <ECDSAInvalidSignatureLength as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ECDSAInvalidSignatureLength(decoded));
            }
            if let Ok(decoded)
                = <ECDSAInvalidSignatureS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ECDSAInvalidSignatureS(decoded));
            }
            if let Ok(decoded)
                = <InvalidAccountNonce as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAccountNonce(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BalancesErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAccountNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BalancesErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAccountNonce as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BalancesErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ECDSAInvalidSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidAccountNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BalancesErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for BalancesErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for BalancesErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for BalancesErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<InvalidAccountNonce> for BalancesErrors {
        fn from(value: InvalidAccountNonce) -> Self {
            Self::InvalidAccountNonce(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(address,address,uint256)")]
    pub struct WithdrawalFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `availableBalance` function with signature `availableBalance()` and selector `0xab2f0e51`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "availableBalance", abi = "availableBalance()")]
    pub struct AvailableBalanceCall;
    ///Container type for all input parameters for the `balanceHeld` function with signature `balanceHeld()` and selector `0x624c6be7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceHeld", abi = "balanceHeld()")]
    pub struct BalanceHeldCall;
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(address)` and selector `0x27e235e3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256)` and selector `0xf3fef3a3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256)")]
    pub struct WithdrawCall {
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawOnBehalf` function with signature `withdrawOnBehalf(address,address,uint256,uint8,bytes32,bytes32)` and selector `0x05391e8f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdrawOnBehalf",
        abi = "withdrawOnBehalf(address,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct WithdrawOnBehalfCall {
        pub account: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BalancesCalls {
        AvailableBalance(AvailableBalanceCall),
        BalanceHeld(BalanceHeldCall),
        BalanceOf(BalanceOfCall),
        Balances(BalancesCall),
        Nonces(NoncesCall),
        Withdraw(WithdrawCall),
        WithdrawOnBehalf(WithdrawOnBehalfCall),
    }
    impl ::ethers::core::abi::AbiDecode for BalancesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AvailableBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AvailableBalance(decoded));
            }
            if let Ok(decoded)
                = <BalanceHeldCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceHeld(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded)
                = <WithdrawOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawOnBehalf(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BalancesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AvailableBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceHeld(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Balances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BalancesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AvailableBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceHeld(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawOnBehalf(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AvailableBalanceCall> for BalancesCalls {
        fn from(value: AvailableBalanceCall) -> Self {
            Self::AvailableBalance(value)
        }
    }
    impl ::core::convert::From<BalanceHeldCall> for BalancesCalls {
        fn from(value: BalanceHeldCall) -> Self {
            Self::BalanceHeld(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for BalancesCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalancesCall> for BalancesCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for BalancesCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for BalancesCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawOnBehalfCall> for BalancesCalls {
        fn from(value: WithdrawOnBehalfCall) -> Self {
            Self::WithdrawOnBehalf(value)
        }
    }
    ///Container type for all return fields from the `availableBalance` function with signature `availableBalance()` and selector `0xab2f0e51`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AvailableBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceHeld` function with signature `balanceHeld()` and selector `0x624c6be7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceHeldReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balances` function with signature `balances(address)` and selector `0x27e235e3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
}
