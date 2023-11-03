pub use impact_evaluator::*;
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
pub mod impact_evaluator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EVALUATE_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EVALUATE_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_SCORE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_SCORE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addMeasurements"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addMeasurements"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminAdvanceRound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adminAdvanceRound"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("currentRoundEnd"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("currentRoundEnd"),
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
                    ::std::borrow::ToOwned::to_owned("currentRoundEndBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentRoundEndBlockNumber",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("currentRoundIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("currentRoundIndex"),
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
                    ::std::borrow::ToOwned::to_owned("currentRoundRoundReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentRoundRoundReward",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextRoundLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nextRoundLength"),
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
                    ::std::borrow::ToOwned::to_owned("previousRoundIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previousRoundIndex"),
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
                    ::std::borrow::ToOwned::to_owned("previousRoundRoundReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "previousRoundRoundReward",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("previousRoundTotalScores"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "previousRoundTotalScores",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "callerConfirmation",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("roundReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("roundReward"),
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
                    ::std::borrow::ToOwned::to_owned("setNextRoundLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setNextRoundLength"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nextRoundLength"),
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
                    ::std::borrow::ToOwned::to_owned("setRoundReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRoundReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_roundReward"),
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
                    ::std::borrow::ToOwned::to_owned("setScores"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setScores"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roundIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("scores"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64[]"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateScores"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateScores"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("scores"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64[]"),
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
                    ::std::borrow::ToOwned::to_owned("MeasurementsAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MeasurementsAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("roundIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoundStart"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoundStart"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("roundIndex"),
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
                    ::std::borrow::ToOwned::to_owned("AccessControlBadConfirmation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlBadConfirmation",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessControlUnauthorizedAccount",
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
                                    name: ::std::borrow::ToOwned::to_owned("neededRole"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IMPACTEVALUATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\n`\x08Uh\x05k\xC7^-c\x10\0\0`\tU`\0`\x0BU4\x80\x15b\0\0(W`\0\x80\xFD[P`@Qb\0 \xA48\x03\x80b\0 \xA4\x839\x81\x01`@\x81\x90Rb\0\0K\x91b\0\x02\x13V[b\0\0X`\0\x82b\0\0\x97V[Pb\0\0\x85\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x82b\0\0\x97V[Pb\0\0\x90b\0\x01FV[Pb\0\x02\x87V[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16b\0\x01<W`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\xF33\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01b\0\x01@V[P`\0[\x92\x91PPV[`\0`\x0BTGb\0\x01X\x91\x90b\0\x02[V[\x90P`\0`\tT\x82\x10b\0\x01oW`\tTb\0\x01qV[\x81[\x90P\x80`\x0B`\0\x82\x82Tb\0\x01\x87\x91\x90b\0\x02qV[\x90\x91UPP`\x02T`\x05U`\0`\x06U`\x04T`\x07U`\x03T\x15b\0\x01\xBCW`\x02Tb\0\x01\xB6\x90`\x01b\0\x02qV[b\0\x01\xBFV[`\0[`\x02U`\x08Tb\0\x01\xD1\x90Cb\0\x02qV[`\x03U`\x04\x81\x90U`\x02T`@Q\x90\x81R\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4s\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\0` \x82\x84\x03\x12\x15b\0\x02&W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02>W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15b\0\x01@Wb\0\x01@b\0\x02EV[\x80\x82\x01\x80\x82\x11\x15b\0\x01@Wb\0\x01@b\0\x02EV[a\x1E\r\x80b\0\x02\x97`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xD1W`\x005`\xE0\x1C\x80cxs\xCD\xE2\x11a\0\xF7W\x80c\xB5}\x01.\x11a\0\x95W\x80c\xF0\x93+\xD7\x11a\0dW\x80c\xF0\x93+\xD7\x14a\x05\x8DW\x80c\xF1\x99\xF5m\x14a\x05\xADW\x80c\xF3\xFE\xF3\xA3\x14a\x05\xC3W\x80c\xFDY\xA1\x95\x14a\x05\xE3W`\0\x80\xFD[\x80c\xB5}\x01.\x14a\x05\x17W\x80c\xBD\x99\x8C\x96\x14a\x05-W\x80c\xD5Gt\x1F\x14a\x05MW\x80c\xDA\x84\\\xE0\x14a\x05mW`\0\x80\xFD[\x80c\x93\x84\xC8\x85\x11a\0\xD1W\x80c\x93\x84\xC8\x85\x14a\x04\xC1W\x80c\x99\x1B\"\x08\x14a\x04\xD7W\x80c\x9F\x7F\x9D\xA2\x14a\x04\xEDW\x80c\xA2\x17\xFD\xDF\x14a\x05\x02W`\0\x80\xFD[\x80cxs\xCD\xE2\x14a\x03\xF9W\x80c~\xCE\xBE\0\x14a\x04-W\x80c\x91\xD1HT\x14a\x04pW`\0\x80\xFD[\x80c*Nbz\x11a\x01oW\x80cU5\xDB\xF6\x11a\x01>W\x80cU5\xDB\xF6\x14a\x03jW\x80cbLk\xE7\x14a\x03\x8AW\x80ch\x96\xEFK\x14a\x03\xA0W\x80cp\xA0\x821\x14a\x03\xB6W`\0\x80\xFD[\x80c*Nbz\x14a\x02\xFEW\x80c//\xF1]\x14a\x03\x14W\x80c0\xCB\x93\xB5\x14a\x034W\x80c6V\x8A\xBE\x14a\x03JW`\0\x80\xFD[\x80c\x1C\x9B(\xBF\x11a\x01\xABW\x80c\x1C\x9B(\xBF\x14a\x02IW\x80c$\x8A\x9C\xA3\x14a\x02mW\x80c'\xE25\xE3\x14a\x02\x9DW\x80c'\xFFb#\x14a\x02\xCAW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xDDW\x80c\x059\x1E\x8F\x14a\x02\x12W\x80c\x05\xF3xS\x14a\x024W`\0\x80\xFD[6a\x01\xD8W\0[`\0\x80\xFD[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01\xFDa\x01\xF86`\x04a\x17\xB6V[a\x06\x03V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1EW`\0\x80\xFD[Pa\x022a\x02-6`\x04a\x18!V[a\x06\x9CV[\0[4\x80\x15a\x02@W`\0\x80\xFD[Pa\x022a\x08\x81V[4\x80\x15a\x02UW`\0\x80\xFD[Pa\x02_`\x04T\x81V[`@Q\x90\x81R` \x01a\x02\tV[4\x80\x15a\x02yW`\0\x80\xFD[Pa\x02_a\x02\x886`\x04a\x18\x8CV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x02\xA9W`\0\x80\xFD[Pa\x02_a\x02\xB86`\x04a\x18\xA5V[`\n` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02\xE5f\x03\x8D~\xA4\xC6\x80\0\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\tV[4\x80\x15a\x03\nW`\0\x80\xFD[Pa\x02_`\x03T\x81V[4\x80\x15a\x03 W`\0\x80\xFD[Pa\x022a\x03/6`\x04a\x18\xC2V[a\t#V[4\x80\x15a\x03@W`\0\x80\xFD[Pa\x02_`\x07T\x81V[4\x80\x15a\x03VW`\0\x80\xFD[Pa\x022a\x03e6`\x04a\x18\xC2V[a\tNV[4\x80\x15a\x03vW`\0\x80\xFD[Pa\x02_a\x03\x856`\x04a\x19pV[a\t\xACV[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x02_`\x0BT\x81V[4\x80\x15a\x03\xACW`\0\x80\xFD[Pa\x02_`\x02T\x81V[4\x80\x15a\x03\xC2W`\0\x80\xFD[Pa\x02_a\x03\xD16`\x04a\x18\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\n` R`@\x90 T\x90V[4\x80\x15a\x04\x05W`\0\x80\xFD[Pa\x02_\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x049W`\0\x80\xFD[Pa\x02_a\x04H6`\x04a\x18\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[4\x80\x15a\x04|W`\0\x80\xFD[Pa\x01\xFDa\x04\x8B6`\x04a\x18\xC2V[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x04\xCDW`\0\x80\xFD[Pa\x02_`\x08T\x81V[4\x80\x15a\x04\xE3W`\0\x80\xFD[Pa\x02_`\x05T\x81V[4\x80\x15a\x04\xF9W`\0\x80\xFD[P`\x03Ta\x02_V[4\x80\x15a\x05\x0EW`\0\x80\xFD[Pa\x02_`\0\x81V[4\x80\x15a\x05#W`\0\x80\xFD[Pa\x02_`\x06T\x81V[4\x80\x15a\x059W`\0\x80\xFD[Pa\x02_a\x05H6`\x04a\x1A\xC9V[a\n\0V[4\x80\x15a\x05YW`\0\x80\xFD[Pa\x022a\x05h6`\x04a\x18\xC2V[a\x0B\x81V[4\x80\x15a\x05yW`\0\x80\xFD[Pa\x022a\x05\x886`\x04a\x18\x8CV[a\x0B\xA6V[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x022a\x05\xA86`\x04a\x18\x8CV[a\x0CCV[4\x80\x15a\x05\xB9W`\0\x80\xFD[Pa\x02_`\tT\x81V[4\x80\x15a\x05\xCFW`\0\x80\xFD[Pa\x022a\x05\xDE6`\x04a\x1B\x06V[a\rpV[4\x80\x15a\x05\xEFW`\0\x80\xFD[Pa\x022a\x05\xFE6`\x04a\x1B2V[a\r\xD0V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x06\x96WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x82\x01\x90U\x87\x90`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x82\x01R\x90\x81\x01\x91\x90\x91R3``\x82\x01R\x90\x87\x16`\x80\x82\x01R`\xA0\x81\x01\x86\x90R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x072\x82\x86\x86\x86a\x0F\xA7V[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xCEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\xE1\x883g\x01cEx]\x8A\0\0a\x0F\xD5V[a\x07\xFD\x88\x88a\x07\xF8g\x01cEx]\x8A\0\0\x8Aa\x1C.V[a\x0F\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x88a\x08Hg\x01cEx]\x8A\0\0\x8Aa\x1C.V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\t\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[a\t!a\x11\xA2V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\t>\x81a\x12cV[a\tH\x83\x83a\x12pV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\t\x9DW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xA7\x82\x82a\x13lV[PPPV[`\x02T`@Q`\0\x91\x903\x90\x7F\xDBL\xFD\xC7\x95\xED\xA1d\x07\xC0\x8D\xAB&\xED\xC5\x83\xF4~-\x10\xE9\xE5l%\x0FE6\x12@\x93\xE8x\x90a\t\xE7\x90\x86\x90\x85\x90a\x1CAV[`@Q\x80\x91\x03\x90\xA2`\x03TC\x10a\x06\x96Wa\x06\x96a\x11\xA2V[`\0\x80\x80[\x83Q\x81\x10\x15a\nGW\x83\x81\x81Q\x81\x10a\n Wa\n a\x1C\xB4V[` \x02` \x01\x01Q\x82a\n3\x91\x90a\x1C\xE3V[\x91P\x80a\n?\x81a\x1D\x0BV[\x91PPa\n\x05V[Pf\x03\x8D~\xA4\xC6\x80\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a\n\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FSum of scores too big\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\x06Tf\x03\x8D~\xA4\xC6\x80\0\x90a\n\xE3\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x1DCV[\x11\x15a\x0BqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSum of scores including historic`D\x82\x01R\x7F too big\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0B\x9C\x81a\x12cV[a\tH\x83\x83a\x13lV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0C>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\tUV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0C\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\0\x81\x11a\rkW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNext round length must be positi`D\x82\x01R\x7Fve\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[`\x08UV[a\r{3\x83\x83a\x0F\xD5V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R3\x91\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x0EhW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[\x80Q\x82Q\x14a\x0E\xF8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[`\x02T`\x05T\x14\x15\x80\x15a\x0F\rWP`\x05T\x83\x14[a\x0FsW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCan only score previous round\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\0a\x0F~\x82a\n\0V[\x90Pa\x0F\x8A\x83\x83a\x14'V[\x80`\x06`\0\x82\x82Ta\x0F\x9C\x91\x90a\x1DCV[\x90\x91UPPPPPPV[`\0\x80`\0\x80a\x0F\xB9\x88\x88\x88\x88a\x15.V[\x92P\x92P\x92Pa\x0F\xC9\x82\x82a\x16(V[P\x90\x96\x95PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x90 T\x81\x11\x15a\x10dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInsufficient balance\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x81 \x80T\x83\x92\x90a\x10\x99\x90\x84\x90a\x1C.V[\x92PP\x81\x90UP\x80`\x0B`\0\x82\x82Ta\x10\xB2\x91\x90a\x1C.V[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x81 T\x90\x03a\x11\nWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x81 U[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa\t\xA7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FWithdrawal failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\0`\x0BTGa\x11\xB2\x91\x90a\x1C.V[\x90P`\0`\tT\x82\x10a\x11\xC7W`\tTa\x11\xC9V[\x81[\x90P\x80`\x0B`\0\x82\x82Ta\x11\xDD\x91\x90a\x1DCV[\x90\x91UPP`\x02T`\x05U`\0`\x06U`\x04T`\x07U`\x03T\x15a\x12\x0EW`\x02Ta\x12\t\x90`\x01a\x1DCV[a\x12\x11V[`\0[`\x02U`\x08Ta\x12!\x90Ca\x1DCV[`\x03U`\x04\x81\x90U`\x02T`@Q\x90\x81R\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4s\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[a\x12m\x813a\x170V[PV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x13dW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x13\x023\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x06\x96V[P`\0a\x06\x96V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x13dW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x06\x96V[`\0[\x82Q\x81\x10\x15a\t\xA7W`\0\x83\x82\x81Q\x81\x10a\x14GWa\x14Ga\x1C\xB4V[` \x02` \x01\x01Q\x90P`\0f\x03\x8D~\xA4\xC6\x80\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x07T\x85\x85\x81Q\x81\x10a\x14zWa\x14za\x1C\xB4V[` \x02` \x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x14\x96\x91\x90a\x1DVV[a\x14\xA0\x91\x90a\x1DmV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\xDE\xAD\x03a\x14\xDEW\x80`\x0B`\0\x82\x82Ta\x14\xD3\x91\x90a\x1C.V[\x90\x91UPa\x15\x19\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\n` R`@\x81 \x80T\x83\x92\x90a\x15\x13\x90\x84\x90a\x1DCV[\x90\x91UPP[PP\x80\x80a\x15&\x90a\x1D\x0BV[\x91PPa\x14*V[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x15iWP`\0\x91P`\x03\x90P\x82a\x16\x1EV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xBDW=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x16\x14WP`\0\x92P`\x01\x91P\x82\x90Pa\x16\x1EV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x16<Wa\x16<a\x1D\xA8V[\x03a\x16EWPPV[`\x01\x82`\x03\x81\x11\x15a\x16YWa\x16Ya\x1D\xA8V[\x03a\x16\x90W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16\xA4Wa\x16\xA4a\x1D\xA8V[\x03a\x16\xDEW`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xC5V[`\x03\x82`\x03\x81\x11\x15a\x16\xF2Wa\x16\xF2a\x1D\xA8V[\x03a\x17,W`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xC5V[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x17,W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x07\xC5V[`\0` \x82\x84\x03\x12\x15a\x17\xC8W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x17\xF8W`\0\x80\xFD[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12mW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18:W`\0\x80\xFD[\x865a\x18E\x81a\x17\xFFV[\x95P` \x87\x015a\x18U\x81a\x17\xFFV[\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a\x18rW`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0` \x82\x84\x03\x12\x15a\x18\x9EW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xB7W`\0\x80\xFD[\x815a\x17\xF8\x81a\x17\xFFV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\xD5W`\0\x80\xFD[\x825\x91P` \x83\x015a\x18\xE7\x81a\x17\xFFV[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19hWa\x19ha\x18\xF2V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x19\x83W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\x9BW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x19\xAFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x19\xC1Wa\x19\xC1a\x18\xF2V[a\x19\xF1\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x19!V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x1A\x07W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A=Wa\x1A=a\x18\xF2V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1AXW`\0\x80\xFD[\x815` a\x1Ama\x1Ah\x83a\x1A#V[a\x19!V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1A\x8CW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1A\xBEW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xB1W`\0\x80\x81\xFD[\x83R\x91\x83\x01\x91\x83\x01a\x1A\x90V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1A\xDBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xF2W`\0\x80\xFD[a\x1A\xFE\x84\x82\x85\x01a\x1AGV[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x19W`\0\x80\xFD[\x825a\x1B$\x81a\x17\xFFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BGW`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1BgW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1B{W`\0\x80\xFD[\x815a\x1B\x89a\x1Ah\x82a\x1A#V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a\x1B\xA8W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x1B\xCFW\x845a\x1B\xC0\x81a\x17\xFFV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x1B\xADV[\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a\x1B\xE7W`\0\x80\xFD[PPa\x1B\xF5\x86\x82\x87\x01a\x1AGV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\x96Wa\x06\x96a\x1B\xFFV[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x1CoW` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x1CRV[P`\0``\x82\x85\x01\x01R``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1D\x04Wa\x1D\x04a\x1B\xFFV[P\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1D<Wa\x1D<a\x1B\xFFV[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x06\x96Wa\x06\x96a\x1B\xFFV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\x96Wa\x06\x96a\x1B\xFFV[`\0\x82a\x1D\xA3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA6\xD4\x97\xBD\x03\xAE\x19\xC9\x91\x83n\xFE*\x04\x19\xF2S\xA0\x88&\x81\x0B\xD3$\x9B_\x90g\x90\x99v\xDEdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static IMPACTEVALUATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xD1W`\x005`\xE0\x1C\x80cxs\xCD\xE2\x11a\0\xF7W\x80c\xB5}\x01.\x11a\0\x95W\x80c\xF0\x93+\xD7\x11a\0dW\x80c\xF0\x93+\xD7\x14a\x05\x8DW\x80c\xF1\x99\xF5m\x14a\x05\xADW\x80c\xF3\xFE\xF3\xA3\x14a\x05\xC3W\x80c\xFDY\xA1\x95\x14a\x05\xE3W`\0\x80\xFD[\x80c\xB5}\x01.\x14a\x05\x17W\x80c\xBD\x99\x8C\x96\x14a\x05-W\x80c\xD5Gt\x1F\x14a\x05MW\x80c\xDA\x84\\\xE0\x14a\x05mW`\0\x80\xFD[\x80c\x93\x84\xC8\x85\x11a\0\xD1W\x80c\x93\x84\xC8\x85\x14a\x04\xC1W\x80c\x99\x1B\"\x08\x14a\x04\xD7W\x80c\x9F\x7F\x9D\xA2\x14a\x04\xEDW\x80c\xA2\x17\xFD\xDF\x14a\x05\x02W`\0\x80\xFD[\x80cxs\xCD\xE2\x14a\x03\xF9W\x80c~\xCE\xBE\0\x14a\x04-W\x80c\x91\xD1HT\x14a\x04pW`\0\x80\xFD[\x80c*Nbz\x11a\x01oW\x80cU5\xDB\xF6\x11a\x01>W\x80cU5\xDB\xF6\x14a\x03jW\x80cbLk\xE7\x14a\x03\x8AW\x80ch\x96\xEFK\x14a\x03\xA0W\x80cp\xA0\x821\x14a\x03\xB6W`\0\x80\xFD[\x80c*Nbz\x14a\x02\xFEW\x80c//\xF1]\x14a\x03\x14W\x80c0\xCB\x93\xB5\x14a\x034W\x80c6V\x8A\xBE\x14a\x03JW`\0\x80\xFD[\x80c\x1C\x9B(\xBF\x11a\x01\xABW\x80c\x1C\x9B(\xBF\x14a\x02IW\x80c$\x8A\x9C\xA3\x14a\x02mW\x80c'\xE25\xE3\x14a\x02\x9DW\x80c'\xFFb#\x14a\x02\xCAW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xDDW\x80c\x059\x1E\x8F\x14a\x02\x12W\x80c\x05\xF3xS\x14a\x024W`\0\x80\xFD[6a\x01\xD8W\0[`\0\x80\xFD[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01\xFDa\x01\xF86`\x04a\x17\xB6V[a\x06\x03V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1EW`\0\x80\xFD[Pa\x022a\x02-6`\x04a\x18!V[a\x06\x9CV[\0[4\x80\x15a\x02@W`\0\x80\xFD[Pa\x022a\x08\x81V[4\x80\x15a\x02UW`\0\x80\xFD[Pa\x02_`\x04T\x81V[`@Q\x90\x81R` \x01a\x02\tV[4\x80\x15a\x02yW`\0\x80\xFD[Pa\x02_a\x02\x886`\x04a\x18\x8CV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x02\xA9W`\0\x80\xFD[Pa\x02_a\x02\xB86`\x04a\x18\xA5V[`\n` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02\xE5f\x03\x8D~\xA4\xC6\x80\0\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\tV[4\x80\x15a\x03\nW`\0\x80\xFD[Pa\x02_`\x03T\x81V[4\x80\x15a\x03 W`\0\x80\xFD[Pa\x022a\x03/6`\x04a\x18\xC2V[a\t#V[4\x80\x15a\x03@W`\0\x80\xFD[Pa\x02_`\x07T\x81V[4\x80\x15a\x03VW`\0\x80\xFD[Pa\x022a\x03e6`\x04a\x18\xC2V[a\tNV[4\x80\x15a\x03vW`\0\x80\xFD[Pa\x02_a\x03\x856`\x04a\x19pV[a\t\xACV[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x02_`\x0BT\x81V[4\x80\x15a\x03\xACW`\0\x80\xFD[Pa\x02_`\x02T\x81V[4\x80\x15a\x03\xC2W`\0\x80\xFD[Pa\x02_a\x03\xD16`\x04a\x18\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\n` R`@\x90 T\x90V[4\x80\x15a\x04\x05W`\0\x80\xFD[Pa\x02_\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x049W`\0\x80\xFD[Pa\x02_a\x04H6`\x04a\x18\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[4\x80\x15a\x04|W`\0\x80\xFD[Pa\x01\xFDa\x04\x8B6`\x04a\x18\xC2V[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x04\xCDW`\0\x80\xFD[Pa\x02_`\x08T\x81V[4\x80\x15a\x04\xE3W`\0\x80\xFD[Pa\x02_`\x05T\x81V[4\x80\x15a\x04\xF9W`\0\x80\xFD[P`\x03Ta\x02_V[4\x80\x15a\x05\x0EW`\0\x80\xFD[Pa\x02_`\0\x81V[4\x80\x15a\x05#W`\0\x80\xFD[Pa\x02_`\x06T\x81V[4\x80\x15a\x059W`\0\x80\xFD[Pa\x02_a\x05H6`\x04a\x1A\xC9V[a\n\0V[4\x80\x15a\x05YW`\0\x80\xFD[Pa\x022a\x05h6`\x04a\x18\xC2V[a\x0B\x81V[4\x80\x15a\x05yW`\0\x80\xFD[Pa\x022a\x05\x886`\x04a\x18\x8CV[a\x0B\xA6V[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x022a\x05\xA86`\x04a\x18\x8CV[a\x0CCV[4\x80\x15a\x05\xB9W`\0\x80\xFD[Pa\x02_`\tT\x81V[4\x80\x15a\x05\xCFW`\0\x80\xFD[Pa\x022a\x05\xDE6`\x04a\x1B\x06V[a\rpV[4\x80\x15a\x05\xEFW`\0\x80\xFD[Pa\x022a\x05\xFE6`\x04a\x1B2V[a\r\xD0V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x06\x96WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x82\x01\x90U\x87\x90`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x82\x01R\x90\x81\x01\x91\x90\x91R3``\x82\x01R\x90\x87\x16`\x80\x82\x01R`\xA0\x81\x01\x86\x90R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x072\x82\x86\x86\x86a\x0F\xA7V[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xCEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\xE1\x883g\x01cEx]\x8A\0\0a\x0F\xD5V[a\x07\xFD\x88\x88a\x07\xF8g\x01cEx]\x8A\0\0\x8Aa\x1C.V[a\x0F\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x88a\x08Hg\x01cEx]\x8A\0\0\x8Aa\x1C.V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\t\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[a\t!a\x11\xA2V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\t>\x81a\x12cV[a\tH\x83\x83a\x12pV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\t\x9DW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xA7\x82\x82a\x13lV[PPPV[`\x02T`@Q`\0\x91\x903\x90\x7F\xDBL\xFD\xC7\x95\xED\xA1d\x07\xC0\x8D\xAB&\xED\xC5\x83\xF4~-\x10\xE9\xE5l%\x0FE6\x12@\x93\xE8x\x90a\t\xE7\x90\x86\x90\x85\x90a\x1CAV[`@Q\x80\x91\x03\x90\xA2`\x03TC\x10a\x06\x96Wa\x06\x96a\x11\xA2V[`\0\x80\x80[\x83Q\x81\x10\x15a\nGW\x83\x81\x81Q\x81\x10a\n Wa\n a\x1C\xB4V[` \x02` \x01\x01Q\x82a\n3\x91\x90a\x1C\xE3V[\x91P\x80a\n?\x81a\x1D\x0BV[\x91PPa\n\x05V[Pf\x03\x8D~\xA4\xC6\x80\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a\n\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FSum of scores too big\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\x06Tf\x03\x8D~\xA4\xC6\x80\0\x90a\n\xE3\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x1DCV[\x11\x15a\x0BqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSum of scores including historic`D\x82\x01R\x7F too big\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0B\x9C\x81a\x12cV[a\tH\x83\x83a\x13lV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0C>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\tUV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0C\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\0\x81\x11a\rkW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNext round length must be positi`D\x82\x01R\x7Fve\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[`\x08UV[a\r{3\x83\x83a\x0F\xD5V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R3\x91\x7F'\x17\xEA\xD6\xB9 \r\xD25\xAA\xD4h\xC9\x80\x9E\xA4\0\xFE3\xACi\xB5\xBF\xAAm>\x90\xFC\x92+c\x98\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x0EhW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[\x80Q\x82Q\x14a\x0E\xF8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xC5V[`\x02T`\x05T\x14\x15\x80\x15a\x0F\rWP`\x05T\x83\x14[a\x0FsW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCan only score previous round\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\0a\x0F~\x82a\n\0V[\x90Pa\x0F\x8A\x83\x83a\x14'V[\x80`\x06`\0\x82\x82Ta\x0F\x9C\x91\x90a\x1DCV[\x90\x91UPPPPPPV[`\0\x80`\0\x80a\x0F\xB9\x88\x88\x88\x88a\x15.V[\x92P\x92P\x92Pa\x0F\xC9\x82\x82a\x16(V[P\x90\x96\x95PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x90 T\x81\x11\x15a\x10dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInsufficient balance\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x81 \x80T\x83\x92\x90a\x10\x99\x90\x84\x90a\x1C.V[\x92PP\x81\x90UP\x80`\x0B`\0\x82\x82Ta\x10\xB2\x91\x90a\x1C.V[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x81 T\x90\x03a\x11\nWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\n` R`@\x81 U[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa\t\xA7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FWithdrawal failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xC5V[`\0`\x0BTGa\x11\xB2\x91\x90a\x1C.V[\x90P`\0`\tT\x82\x10a\x11\xC7W`\tTa\x11\xC9V[\x81[\x90P\x80`\x0B`\0\x82\x82Ta\x11\xDD\x91\x90a\x1DCV[\x90\x91UPP`\x02T`\x05U`\0`\x06U`\x04T`\x07U`\x03T\x15a\x12\x0EW`\x02Ta\x12\t\x90`\x01a\x1DCV[a\x12\x11V[`\0[`\x02U`\x08Ta\x12!\x90Ca\x1DCV[`\x03U`\x04\x81\x90U`\x02T`@Q\x90\x81R\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4s\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[a\x12m\x813a\x170V[PV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x13dW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x13\x023\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x06\x96V[P`\0a\x06\x96V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x13dW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x06\x96V[`\0[\x82Q\x81\x10\x15a\t\xA7W`\0\x83\x82\x81Q\x81\x10a\x14GWa\x14Ga\x1C\xB4V[` \x02` \x01\x01Q\x90P`\0f\x03\x8D~\xA4\xC6\x80\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x07T\x85\x85\x81Q\x81\x10a\x14zWa\x14za\x1C\xB4V[` \x02` \x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x14\x96\x91\x90a\x1DVV[a\x14\xA0\x91\x90a\x1DmV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\xDE\xAD\x03a\x14\xDEW\x80`\x0B`\0\x82\x82Ta\x14\xD3\x91\x90a\x1C.V[\x90\x91UPa\x15\x19\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\n` R`@\x81 \x80T\x83\x92\x90a\x15\x13\x90\x84\x90a\x1DCV[\x90\x91UPP[PP\x80\x80a\x15&\x90a\x1D\x0BV[\x91PPa\x14*V[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x15iWP`\0\x91P`\x03\x90P\x82a\x16\x1EV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xBDW=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x16\x14WP`\0\x92P`\x01\x91P\x82\x90Pa\x16\x1EV[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`\0\x82`\x03\x81\x11\x15a\x16<Wa\x16<a\x1D\xA8V[\x03a\x16EWPPV[`\x01\x82`\x03\x81\x11\x15a\x16YWa\x16Ya\x1D\xA8V[\x03a\x16\x90W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16\xA4Wa\x16\xA4a\x1D\xA8V[\x03a\x16\xDEW`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xC5V[`\x03\x82`\x03\x81\x11\x15a\x16\xF2Wa\x16\xF2a\x1D\xA8V[\x03a\x17,W`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xC5V[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x17,W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x07\xC5V[`\0` \x82\x84\x03\x12\x15a\x17\xC8W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x17\xF8W`\0\x80\xFD[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12mW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18:W`\0\x80\xFD[\x865a\x18E\x81a\x17\xFFV[\x95P` \x87\x015a\x18U\x81a\x17\xFFV[\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a\x18rW`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0` \x82\x84\x03\x12\x15a\x18\x9EW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xB7W`\0\x80\xFD[\x815a\x17\xF8\x81a\x17\xFFV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\xD5W`\0\x80\xFD[\x825\x91P` \x83\x015a\x18\xE7\x81a\x17\xFFV[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19hWa\x19ha\x18\xF2V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x19\x83W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\x9BW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x19\xAFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x19\xC1Wa\x19\xC1a\x18\xF2V[a\x19\xF1\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x19!V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x1A\x07W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A=Wa\x1A=a\x18\xF2V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1AXW`\0\x80\xFD[\x815` a\x1Ama\x1Ah\x83a\x1A#V[a\x19!V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1A\x8CW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1A\xBEW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xB1W`\0\x80\x81\xFD[\x83R\x91\x83\x01\x91\x83\x01a\x1A\x90V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1A\xDBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xF2W`\0\x80\xFD[a\x1A\xFE\x84\x82\x85\x01a\x1AGV[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x19W`\0\x80\xFD[\x825a\x1B$\x81a\x17\xFFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BGW`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1BgW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1B{W`\0\x80\xFD[\x815a\x1B\x89a\x1Ah\x82a\x1A#V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a\x1B\xA8W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x1B\xCFW\x845a\x1B\xC0\x81a\x17\xFFV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x1B\xADV[\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a\x1B\xE7W`\0\x80\xFD[PPa\x1B\xF5\x86\x82\x87\x01a\x1AGV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\x96Wa\x06\x96a\x1B\xFFV[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x1CoW` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x1CRV[P`\0``\x82\x85\x01\x01R``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1D\x04Wa\x1D\x04a\x1B\xFFV[P\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1D<Wa\x1D<a\x1B\xFFV[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x06\x96Wa\x06\x96a\x1B\xFFV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\x96Wa\x06\x96a\x1B\xFFV[`\0\x82a\x1D\xA3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA6\xD4\x97\xBD\x03\xAE\x19\xC9\x91\x83n\xFE*\x04\x19\xF2S\xA0\x88&\x81\x0B\xD3$\x9B_\x90g\x90\x99v\xDEdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static IMPACTEVALUATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ImpactEvaluator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ImpactEvaluator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ImpactEvaluator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ImpactEvaluator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ImpactEvaluator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ImpactEvaluator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ImpactEvaluator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMPACTEVALUATOR_ABI.clone(),
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
                IMPACTEVALUATOR_ABI.clone(),
                IMPACTEVALUATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EVALUATE_ROLE` (0x7873cde2) function
        pub fn evaluate_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([120, 115, 205, 226], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_SCORE` (0x27ff6223) function
        pub fn max_score(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([39, 255, 98, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addMeasurements` (0x5535dbf6) function
        pub fn add_measurements(
            &self,
            cid: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 53, 219, 246], cid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminAdvanceRound` (0x05f37853) function
        pub fn admin_advance_round(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 243, 120, 83], ())
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
        ///Calls the contract's `currentRoundEnd` (0x9f7f9da2) function
        pub fn current_round_end(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 127, 157, 162], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentRoundEndBlockNumber` (0x2a4e627a) function
        pub fn current_round_end_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([42, 78, 98, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentRoundIndex` (0x6896ef4b) function
        pub fn current_round_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([104, 150, 239, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentRoundRoundReward` (0x1c9b28bf) function
        pub fn current_round_round_reward(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([28, 155, 40, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextRoundLength` (0x9384c885) function
        pub fn next_round_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 132, 200, 133], ())
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
        ///Calls the contract's `previousRoundIndex` (0x991b2208) function
        pub fn previous_round_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 27, 34, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previousRoundRoundReward` (0x30cb93b5) function
        pub fn previous_round_round_reward(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 203, 147, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previousRoundTotalScores` (0xb57d012e) function
        pub fn previous_round_total_scores(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([181, 125, 1, 46], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            caller_confirmation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, caller_confirmation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roundReward` (0xf199f56d) function
        pub fn round_reward(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 153, 245, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNextRoundLength` (0xf0932bd7) function
        pub fn set_next_round_length(
            &self,
            next_round_length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 147, 43, 215], next_round_length)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRoundReward` (0xda845ce0) function
        pub fn set_round_reward(
            &self,
            round_reward: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 132, 92, 224], round_reward)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setScores` (0xfd59a195) function
        pub fn set_scores(
            &self,
            round_index: ::ethers::core::types::U256,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            scores: ::std::vec::Vec<u64>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 89, 161, 149], (round_index, addresses, scores))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateScores` (0xbd998c96) function
        pub fn validate_scores(
            &self,
            scores: ::std::vec::Vec<u64>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 153, 140, 150], scores)
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
        ///Gets the contract's `MeasurementsAdded` event
        pub fn measurements_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MeasurementsAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoundStart` event
        pub fn round_start_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoundStartFilter,
        > {
            self.0.event()
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
            ImpactEvaluatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ImpactEvaluator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessControlBadConfirmation` with signature `AccessControlBadConfirmation()` and selector `0x6697b232`
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
        name = "AccessControlBadConfirmation",
        abi = "AccessControlBadConfirmation()"
    )]
    pub struct AccessControlBadConfirmation;
    ///Custom Error type `AccessControlUnauthorizedAccount` with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`
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
        name = "AccessControlUnauthorizedAccount",
        abi = "AccessControlUnauthorizedAccount(address,bytes32)"
    )]
    pub struct AccessControlUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
        pub needed_role: [u8; 32],
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
    pub enum ImpactEvaluatorErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        InvalidAccountNonce(InvalidAccountNonce),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ImpactEvaluatorErrors {
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
                = <AccessControlBadConfirmation as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AccessControlBadConfirmation(decoded));
            }
            if let Ok(decoded)
                = <AccessControlUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AccessControlUnauthorizedAccount(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ImpactEvaluatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::ethers::contract::ContractRevert for ImpactEvaluatorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessControlBadConfirmation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccessControlUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
    impl ::core::fmt::Display for ImpactEvaluatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlBadConfirmation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessControlUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<::std::string::String> for ImpactEvaluatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessControlBadConfirmation> for ImpactEvaluatorErrors {
        fn from(value: AccessControlBadConfirmation) -> Self {
            Self::AccessControlBadConfirmation(value)
        }
    }
    impl ::core::convert::From<AccessControlUnauthorizedAccount>
    for ImpactEvaluatorErrors {
        fn from(value: AccessControlUnauthorizedAccount) -> Self {
            Self::AccessControlUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for ImpactEvaluatorErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for ImpactEvaluatorErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for ImpactEvaluatorErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<InvalidAccountNonce> for ImpactEvaluatorErrors {
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
    #[ethevent(
        name = "MeasurementsAdded",
        abi = "MeasurementsAdded(string,uint256,address)"
    )]
    pub struct MeasurementsAddedFilter {
        pub cid: ::std::string::String,
        pub round_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoundStart", abi = "RoundStart(uint256)")]
    pub struct RoundStartFilter {
        pub round_index: ::ethers::core::types::U256,
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ImpactEvaluatorEvents {
        MeasurementsAddedFilter(MeasurementsAddedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        RoundStartFilter(RoundStartFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ::ethers::contract::EthLogDecode for ImpactEvaluatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MeasurementsAddedFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::MeasurementsAddedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = RoundStartFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::RoundStartFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::WithdrawalFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ImpactEvaluatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MeasurementsAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundStartFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MeasurementsAddedFilter> for ImpactEvaluatorEvents {
        fn from(value: MeasurementsAddedFilter) -> Self {
            Self::MeasurementsAddedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ImpactEvaluatorEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ImpactEvaluatorEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ImpactEvaluatorEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<RoundStartFilter> for ImpactEvaluatorEvents {
        fn from(value: RoundStartFilter) -> Self {
            Self::RoundStartFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalFilter> for ImpactEvaluatorEvents {
        fn from(value: WithdrawalFilter) -> Self {
            Self::WithdrawalFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `EVALUATE_ROLE` function with signature `EVALUATE_ROLE()` and selector `0x7873cde2`
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
    #[ethcall(name = "EVALUATE_ROLE", abi = "EVALUATE_ROLE()")]
    pub struct EvaluateRoleCall;
    ///Container type for all input parameters for the `MAX_SCORE` function with signature `MAX_SCORE()` and selector `0x27ff6223`
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
    #[ethcall(name = "MAX_SCORE", abi = "MAX_SCORE()")]
    pub struct MaxScoreCall;
    ///Container type for all input parameters for the `addMeasurements` function with signature `addMeasurements(string)` and selector `0x5535dbf6`
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
    #[ethcall(name = "addMeasurements", abi = "addMeasurements(string)")]
    pub struct AddMeasurementsCall {
        pub cid: ::std::string::String,
    }
    ///Container type for all input parameters for the `adminAdvanceRound` function with signature `adminAdvanceRound()` and selector `0x05f37853`
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
    #[ethcall(name = "adminAdvanceRound", abi = "adminAdvanceRound()")]
    pub struct AdminAdvanceRoundCall;
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
    ///Container type for all input parameters for the `currentRoundEnd` function with signature `currentRoundEnd()` and selector `0x9f7f9da2`
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
    #[ethcall(name = "currentRoundEnd", abi = "currentRoundEnd()")]
    pub struct CurrentRoundEndCall;
    ///Container type for all input parameters for the `currentRoundEndBlockNumber` function with signature `currentRoundEndBlockNumber()` and selector `0x2a4e627a`
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
    #[ethcall(name = "currentRoundEndBlockNumber", abi = "currentRoundEndBlockNumber()")]
    pub struct CurrentRoundEndBlockNumberCall;
    ///Container type for all input parameters for the `currentRoundIndex` function with signature `currentRoundIndex()` and selector `0x6896ef4b`
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
    #[ethcall(name = "currentRoundIndex", abi = "currentRoundIndex()")]
    pub struct CurrentRoundIndexCall;
    ///Container type for all input parameters for the `currentRoundRoundReward` function with signature `currentRoundRoundReward()` and selector `0x1c9b28bf`
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
    #[ethcall(name = "currentRoundRoundReward", abi = "currentRoundRoundReward()")]
    pub struct CurrentRoundRoundRewardCall;
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nextRoundLength` function with signature `nextRoundLength()` and selector `0x9384c885`
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
    #[ethcall(name = "nextRoundLength", abi = "nextRoundLength()")]
    pub struct NextRoundLengthCall;
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
    ///Container type for all input parameters for the `previousRoundIndex` function with signature `previousRoundIndex()` and selector `0x991b2208`
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
    #[ethcall(name = "previousRoundIndex", abi = "previousRoundIndex()")]
    pub struct PreviousRoundIndexCall;
    ///Container type for all input parameters for the `previousRoundRoundReward` function with signature `previousRoundRoundReward()` and selector `0x30cb93b5`
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
    #[ethcall(name = "previousRoundRoundReward", abi = "previousRoundRoundReward()")]
    pub struct PreviousRoundRoundRewardCall;
    ///Container type for all input parameters for the `previousRoundTotalScores` function with signature `previousRoundTotalScores()` and selector `0xb57d012e`
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
    #[ethcall(name = "previousRoundTotalScores", abi = "previousRoundTotalScores()")]
    pub struct PreviousRoundTotalScoresCall;
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub caller_confirmation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `roundReward` function with signature `roundReward()` and selector `0xf199f56d`
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
    #[ethcall(name = "roundReward", abi = "roundReward()")]
    pub struct RoundRewardCall;
    ///Container type for all input parameters for the `setNextRoundLength` function with signature `setNextRoundLength(uint256)` and selector `0xf0932bd7`
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
    #[ethcall(name = "setNextRoundLength", abi = "setNextRoundLength(uint256)")]
    pub struct SetNextRoundLengthCall {
        pub next_round_length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setRoundReward` function with signature `setRoundReward(uint256)` and selector `0xda845ce0`
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
    #[ethcall(name = "setRoundReward", abi = "setRoundReward(uint256)")]
    pub struct SetRoundRewardCall {
        pub round_reward: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setScores` function with signature `setScores(uint256,address[],uint64[])` and selector `0xfd59a195`
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
    #[ethcall(name = "setScores", abi = "setScores(uint256,address[],uint64[])")]
    pub struct SetScoresCall {
        pub round_index: ::ethers::core::types::U256,
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub scores: ::std::vec::Vec<u64>,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `validateScores` function with signature `validateScores(uint64[])` and selector `0xbd998c96`
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
    #[ethcall(name = "validateScores", abi = "validateScores(uint64[])")]
    pub struct ValidateScoresCall {
        pub scores: ::std::vec::Vec<u64>,
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
    pub enum ImpactEvaluatorCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EvaluateRole(EvaluateRoleCall),
        MaxScore(MaxScoreCall),
        AddMeasurements(AddMeasurementsCall),
        AdminAdvanceRound(AdminAdvanceRoundCall),
        BalanceHeld(BalanceHeldCall),
        BalanceOf(BalanceOfCall),
        Balances(BalancesCall),
        CurrentRoundEnd(CurrentRoundEndCall),
        CurrentRoundEndBlockNumber(CurrentRoundEndBlockNumberCall),
        CurrentRoundIndex(CurrentRoundIndexCall),
        CurrentRoundRoundReward(CurrentRoundRoundRewardCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        NextRoundLength(NextRoundLengthCall),
        Nonces(NoncesCall),
        PreviousRoundIndex(PreviousRoundIndexCall),
        PreviousRoundRoundReward(PreviousRoundRoundRewardCall),
        PreviousRoundTotalScores(PreviousRoundTotalScoresCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RoundReward(RoundRewardCall),
        SetNextRoundLength(SetNextRoundLengthCall),
        SetRoundReward(SetRoundRewardCall),
        SetScores(SetScoresCall),
        SupportsInterface(SupportsInterfaceCall),
        ValidateScores(ValidateScoresCall),
        Withdraw(WithdrawCall),
        WithdrawOnBehalf(WithdrawOnBehalfCall),
    }
    impl ::ethers::core::abi::AbiDecode for ImpactEvaluatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <EvaluateRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EvaluateRole(decoded));
            }
            if let Ok(decoded)
                = <MaxScoreCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxScore(decoded));
            }
            if let Ok(decoded)
                = <AddMeasurementsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddMeasurements(decoded));
            }
            if let Ok(decoded)
                = <AdminAdvanceRoundCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AdminAdvanceRound(decoded));
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
                = <CurrentRoundEndCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CurrentRoundEnd(decoded));
            }
            if let Ok(decoded)
                = <CurrentRoundEndBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentRoundEndBlockNumber(decoded));
            }
            if let Ok(decoded)
                = <CurrentRoundIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentRoundIndex(decoded));
            }
            if let Ok(decoded)
                = <CurrentRoundRoundRewardCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentRoundRoundReward(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <NextRoundLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NextRoundLength(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <PreviousRoundIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PreviousRoundIndex(decoded));
            }
            if let Ok(decoded)
                = <PreviousRoundRoundRewardCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PreviousRoundRoundReward(decoded));
            }
            if let Ok(decoded)
                = <PreviousRoundTotalScoresCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PreviousRoundTotalScores(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <RoundRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RoundReward(decoded));
            }
            if let Ok(decoded)
                = <SetNextRoundLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetNextRoundLength(decoded));
            }
            if let Ok(decoded)
                = <SetRoundRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRoundReward(decoded));
            }
            if let Ok(decoded)
                = <SetScoresCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetScores(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <ValidateScoresCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateScores(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ImpactEvaluatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EvaluateRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxScore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddMeasurements(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminAdvanceRound(element) => {
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
                Self::CurrentRoundEnd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentRoundEndBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentRoundIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentRoundRoundReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextRoundLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviousRoundIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviousRoundRoundReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviousRoundTotalScores(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoundReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNextRoundLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRoundReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetScores(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateScores(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ImpactEvaluatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::EvaluateRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxScore(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddMeasurements(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminAdvanceRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceHeld(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentRoundEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentRoundEndBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurrentRoundIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentRoundRoundReward(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRoundLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviousRoundIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PreviousRoundRoundReward(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PreviousRoundTotalScores(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNextRoundLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRoundReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetScores(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateScores(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawOnBehalf(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ImpactEvaluatorCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<EvaluateRoleCall> for ImpactEvaluatorCalls {
        fn from(value: EvaluateRoleCall) -> Self {
            Self::EvaluateRole(value)
        }
    }
    impl ::core::convert::From<MaxScoreCall> for ImpactEvaluatorCalls {
        fn from(value: MaxScoreCall) -> Self {
            Self::MaxScore(value)
        }
    }
    impl ::core::convert::From<AddMeasurementsCall> for ImpactEvaluatorCalls {
        fn from(value: AddMeasurementsCall) -> Self {
            Self::AddMeasurements(value)
        }
    }
    impl ::core::convert::From<AdminAdvanceRoundCall> for ImpactEvaluatorCalls {
        fn from(value: AdminAdvanceRoundCall) -> Self {
            Self::AdminAdvanceRound(value)
        }
    }
    impl ::core::convert::From<BalanceHeldCall> for ImpactEvaluatorCalls {
        fn from(value: BalanceHeldCall) -> Self {
            Self::BalanceHeld(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ImpactEvaluatorCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalancesCall> for ImpactEvaluatorCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<CurrentRoundEndCall> for ImpactEvaluatorCalls {
        fn from(value: CurrentRoundEndCall) -> Self {
            Self::CurrentRoundEnd(value)
        }
    }
    impl ::core::convert::From<CurrentRoundEndBlockNumberCall> for ImpactEvaluatorCalls {
        fn from(value: CurrentRoundEndBlockNumberCall) -> Self {
            Self::CurrentRoundEndBlockNumber(value)
        }
    }
    impl ::core::convert::From<CurrentRoundIndexCall> for ImpactEvaluatorCalls {
        fn from(value: CurrentRoundIndexCall) -> Self {
            Self::CurrentRoundIndex(value)
        }
    }
    impl ::core::convert::From<CurrentRoundRoundRewardCall> for ImpactEvaluatorCalls {
        fn from(value: CurrentRoundRoundRewardCall) -> Self {
            Self::CurrentRoundRoundReward(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ImpactEvaluatorCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ImpactEvaluatorCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<NextRoundLengthCall> for ImpactEvaluatorCalls {
        fn from(value: NextRoundLengthCall) -> Self {
            Self::NextRoundLength(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for ImpactEvaluatorCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PreviousRoundIndexCall> for ImpactEvaluatorCalls {
        fn from(value: PreviousRoundIndexCall) -> Self {
            Self::PreviousRoundIndex(value)
        }
    }
    impl ::core::convert::From<PreviousRoundRoundRewardCall> for ImpactEvaluatorCalls {
        fn from(value: PreviousRoundRoundRewardCall) -> Self {
            Self::PreviousRoundRoundReward(value)
        }
    }
    impl ::core::convert::From<PreviousRoundTotalScoresCall> for ImpactEvaluatorCalls {
        fn from(value: PreviousRoundTotalScoresCall) -> Self {
            Self::PreviousRoundTotalScores(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ImpactEvaluatorCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ImpactEvaluatorCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RoundRewardCall> for ImpactEvaluatorCalls {
        fn from(value: RoundRewardCall) -> Self {
            Self::RoundReward(value)
        }
    }
    impl ::core::convert::From<SetNextRoundLengthCall> for ImpactEvaluatorCalls {
        fn from(value: SetNextRoundLengthCall) -> Self {
            Self::SetNextRoundLength(value)
        }
    }
    impl ::core::convert::From<SetRoundRewardCall> for ImpactEvaluatorCalls {
        fn from(value: SetRoundRewardCall) -> Self {
            Self::SetRoundReward(value)
        }
    }
    impl ::core::convert::From<SetScoresCall> for ImpactEvaluatorCalls {
        fn from(value: SetScoresCall) -> Self {
            Self::SetScores(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ImpactEvaluatorCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<ValidateScoresCall> for ImpactEvaluatorCalls {
        fn from(value: ValidateScoresCall) -> Self {
            Self::ValidateScores(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for ImpactEvaluatorCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawOnBehalfCall> for ImpactEvaluatorCalls {
        fn from(value: WithdrawOnBehalfCall) -> Self {
            Self::WithdrawOnBehalf(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `EVALUATE_ROLE` function with signature `EVALUATE_ROLE()` and selector `0x7873cde2`
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
    pub struct EvaluateRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MAX_SCORE` function with signature `MAX_SCORE()` and selector `0x27ff6223`
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
    pub struct MaxScoreReturn(pub u64);
    ///Container type for all return fields from the `addMeasurements` function with signature `addMeasurements(string)` and selector `0x5535dbf6`
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
    pub struct AddMeasurementsReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `currentRoundEnd` function with signature `currentRoundEnd()` and selector `0x9f7f9da2`
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
    pub struct CurrentRoundEndReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `currentRoundEndBlockNumber` function with signature `currentRoundEndBlockNumber()` and selector `0x2a4e627a`
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
    pub struct CurrentRoundEndBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `currentRoundIndex` function with signature `currentRoundIndex()` and selector `0x6896ef4b`
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
    pub struct CurrentRoundIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `currentRoundRoundReward` function with signature `currentRoundRoundReward()` and selector `0x1c9b28bf`
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
    pub struct CurrentRoundRoundRewardReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `nextRoundLength` function with signature `nextRoundLength()` and selector `0x9384c885`
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
    pub struct NextRoundLengthReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `previousRoundIndex` function with signature `previousRoundIndex()` and selector `0x991b2208`
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
    pub struct PreviousRoundIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previousRoundRoundReward` function with signature `previousRoundRoundReward()` and selector `0x30cb93b5`
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
    pub struct PreviousRoundRoundRewardReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previousRoundTotalScores` function with signature `previousRoundTotalScores()` and selector `0xb57d012e`
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
    pub struct PreviousRoundTotalScoresReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `roundReward` function with signature `roundReward()` and selector `0xf199f56d`
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
    pub struct RoundRewardReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `validateScores` function with signature `validateScores(uint64[])` and selector `0xbd998c96`
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
    pub struct ValidateScoresReturn(pub ::ethers::core::types::U256);
}
