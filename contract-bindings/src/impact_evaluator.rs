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
                    ::std::borrow::ToOwned::to_owned("maxTransfersPerTx"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxTransfersPerTx"),
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
                    ::std::borrow::ToOwned::to_owned("minBalanceForTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minBalanceForTransfer",
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
                    ::std::borrow::ToOwned::to_owned("participantCountReadyForTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "participantCountReadyForTransfer",
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
                    ::std::borrow::ToOwned::to_owned(
                        "participantCountScheduledForTransfer",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "participantCountScheduledForTransfer",
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
                    ::std::borrow::ToOwned::to_owned("readyForTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readyForTransfer"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("releaseRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("releaseRewards"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("rewardsScheduledFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rewardsScheduledFor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("participant"),
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
                    ::std::borrow::ToOwned::to_owned("scheduledForTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "scheduledForTransfer",
                            ),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMaxTransfersPerTx"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setMaxTransfersPerTx",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_maxTransfersPerTx",
                                    ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("tick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tick"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("TransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TransferFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R`\n`\x04U`\n`\x0BUh\x05k\xC7^-c\x10\0\0`\x0CU`\0`\rU4\x80\x15b\0\0-W`\0\x80\xFD[P`@Qb\0\x1E\x068\x03\x80b\0\x1E\x06\x839\x81\x01`@\x81\x90Rb\0\0P\x91b\0\x02[V[b\0\0]`\0\x82b\0\0\x9CV[Pb\0\0\x8A\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x82b\0\0\x9CV[Pb\0\0\x95b\0\x01KV[Pb\0\x03\x0CV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16b\0\x01AW`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\xF83\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01b\0\x01EV[P`\0[\x92\x91PPV[`\0f\x03\x8D~\xA4\xC6\x80\0`\tTf\x03\x8D~\xA4\xC6\x80\0b\0\x01l\x91\x90b\0\x02\xA3V[`\nTb\0\x01{\x91\x90b\0\x02\xB9V[b\0\x01\x87\x91\x90b\0\x02\xD3V[\x90P`\0`\x07T\x82`\rTGb\0\x01\x9F\x91\x90b\0\x02\xA3V[b\0\x01\xAB\x91\x90b\0\x02\xA3V[b\0\x01\xB7\x91\x90b\0\x02\xA3V[\x90P`\0`\x0CT\x82\x10b\0\x01\xCEW`\x0CTb\0\x01\xD0V[\x81[`\x05T`\x08U`\0`\tU`\x07T`\nU`\x06T\x90\x91P\x15b\0\x02\x03W`\x05Tb\0\x01\xFD\x90`\x01b\0\x02\xF6V[b\0\x02\x06V[`\0[`\x05U`\x0BTb\0\x02\x18\x90Cb\0\x02\xF6V[`\x06U`\x07\x81\x90U`\x05T`@Q\x90\x81R\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4s\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0` \x82\x84\x03\x12\x15b\0\x02nW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\x86W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15b\0\x01EWb\0\x01Eb\0\x02\x8DV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17b\0\x01EWb\0\x01Eb\0\x02\x8DV[`\0\x82b\0\x02\xF1WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15b\0\x01EWb\0\x01Eb\0\x02\x8DV[a\x1A\xEA\x80b\0\x03\x1C`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x1DW`\x005`\xE0\x1C\x80cbLk\xE7\x11a\x01\x1DW\x80c\xA2\x17\xFD\xDF\x11a\0\xB0W\x80c\xD3\x12\x03R\x11a\0\x7FW\x80c\xDA\x84\\\xE0\x11a\0dW\x80c\xDA\x84\\\xE0\x14a\x06/W\x80c\xF0\x93+\xD7\x14a\x06OW\x80c\xF1\x99\xF5m\x14a\x06oW`\0\x80\xFD[\x80c\xD3\x12\x03R\x14a\x05\xFAW\x80c\xD5Gt\x1F\x14a\x06\x0FW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x05\x9AW\x80c\xB5}\x01.\x14a\x05\xAFW\x80c\xCA\xD5V\\\x14a\x05\xC5W\x80c\xD0\xFA\xB3\xCA\x14a\x05\xE5W`\0\x80\xFD[\x80c\x91\xD1HT\x11a\0\xECW\x80c\x91\xD1HT\x14a\x04\xD8W\x80c\x93\x84\xC8\x85\x14a\x05)W\x80c\x93\xFE\xDDa\x14a\x05?W\x80c\x99\x1B\"\x08\x14a\x05\x84W`\0\x80\xFD[\x80cbLk\xE7\x14a\x04bW\x80ceS}\xE3\x14a\x04xW\x80ch\x96\xEFK\x14a\x04\x8EW\x80cxs\xCD\xE2\x14a\x04\xA4W`\0\x80\xFD[\x80c*Nbz\x11a\x01\xB0W\x80c6V\x8A\xBE\x11a\x01\x7FW\x80cAs9;\x11a\x01dW\x80cAs9;\x14a\x04\x06W\x80cU5\xDB\xF6\x14a\x04\"W\x80c`1\x19\xB1\x14a\x04BW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x03\xD1W\x80c>\xAF]\x9F\x14a\x03\xF1W`\0\x80\xFD[\x80c*Nbz\x14a\x03BW\x80c//\xF1]\x14a\x03XW\x80c0\xCB\x93\xB5\x14a\x03xW\x80c1s\xC2\x88\x14a\x03\x8EW`\0\x80\xFD[\x80c\x1D,\x93\xD8\x11a\x01\xECW\x80c\x1D,\x93\xD8\x14a\x02\xAAW\x80c$\x8A\x9C\xA3\x14a\x02\xCAW\x80c'\xE25\xE3\x14a\x02\xFAW\x80c'\xFFb#\x14a\x03'W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02)W\x80c\x05\xF3xS\x14a\x02^W\x80c\x0F\x88\x13&\x14a\x02uW\x80c\x1C\x9B(\xBF\x14a\x02\x94W`\0\x80\xFD[6a\x02$W\0[`\0\x80\xFD[4\x80\x15a\x025W`\0\x80\xFD[Pa\x02Ia\x02D6`\x04a\x17(V[a\x06\x85V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW`\0\x80\xFD[Pa\x02sa\x07\x1EV[\0[4\x80\x15a\x02\x81W`\0\x80\xFD[P`\x03T[`@Q\x90\x81R` \x01a\x02UV[4\x80\x15a\x02\xA0W`\0\x80\xFD[Pa\x02\x86`\x07T\x81V[4\x80\x15a\x02\xB6W`\0\x80\xFD[Pa\x02sa\x02\xC56`\x04a\x17jV[a\x07\xC5V[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02\x86a\x02\xE56`\x04a\x17jV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x03\x06W`\0\x80\xFD[Pa\x02\x86a\x03\x156`\x04a\x17\xA5V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x033W`\0\x80\xFD[Pa\x02\x86f\x03\x8D~\xA4\xC6\x80\0\x81V[4\x80\x15a\x03NW`\0\x80\xFD[Pa\x02\x86`\x06T\x81V[4\x80\x15a\x03dW`\0\x80\xFD[Pa\x02sa\x03s6`\x04a\x17\xC2V[a\x08iV[4\x80\x15a\x03\x84W`\0\x80\xFD[Pa\x02\x86`\nT\x81V[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x02\x86a\x03\xA96`\x04a\x17\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[4\x80\x15a\x03\xDDW`\0\x80\xFD[Pa\x02sa\x03\xEC6`\x04a\x17\xC2V[a\x08\x94V[4\x80\x15a\x03\xFDW`\0\x80\xFD[Pa\x02sa\x08\xF2V[4\x80\x15a\x04\x12W`\0\x80\xFD[Pa\x02\x86g\x06\xF0[Y\xD3\xB2\0\0\x81V[4\x80\x15a\x04.W`\0\x80\xFD[Pa\x02\x86a\x04=6`\x04a\x17\xF2V[a\t\x0BV[4\x80\x15a\x04NW`\0\x80\xFD[Pa\x02sa\x04]6`\x04a\x18\xB0V[a\tsV[4\x80\x15a\x04nW`\0\x80\xFD[Pa\x02\x86`\rT\x81V[4\x80\x15a\x04\x84W`\0\x80\xFD[Pa\x02\x86`\x04T\x81V[4\x80\x15a\x04\x9AW`\0\x80\xFD[Pa\x02\x86`\x05T\x81V[4\x80\x15a\x04\xB0W`\0\x80\xFD[Pa\x02\x86\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x04\xE4W`\0\x80\xFD[Pa\x02Ia\x04\xF36`\x04a\x17\xC2V[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x055W`\0\x80\xFD[Pa\x02\x86`\x0BT\x81V[4\x80\x15a\x05KW`\0\x80\xFD[Pa\x05_a\x05Z6`\x04a\x17jV[a\x0C\xC4V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x05\x90W`\0\x80\xFD[Pa\x02\x86`\x08T\x81V[4\x80\x15a\x05\xA6W`\0\x80\xFD[Pa\x02\x86`\0\x81V[4\x80\x15a\x05\xBBW`\0\x80\xFD[Pa\x02\x86`\tT\x81V[4\x80\x15a\x05\xD1W`\0\x80\xFD[Pa\x05_a\x05\xE06`\x04a\x17jV[a\x0C\xFBV[4\x80\x15a\x05\xF1W`\0\x80\xFD[Pa\x02sa\r\x0BV[4\x80\x15a\x06\x06W`\0\x80\xFD[P`\x02Ta\x02\x86V[4\x80\x15a\x06\x1BW`\0\x80\xFD[Pa\x02sa\x06*6`\x04a\x17\xC2V[a\r\xCBV[4\x80\x15a\x06;W`\0\x80\xFD[Pa\x02sa\x06J6`\x04a\x17jV[a\r\xF0V[4\x80\x15a\x06[W`\0\x80\xFD[Pa\x02sa\x06j6`\x04a\x17jV[a\x0E\x8DV[4\x80\x15a\x06{W`\0\x80\xFD[Pa\x02\x86`\x0CT\x81V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x07\x18WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x07\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\xC3a\x0F\xBAV[V[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x08]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[a\x08f\x81`\x04UV[PV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x84\x81a\x10\xB6V[a\x08\x8E\x83\x83a\x10\xC0V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x08\xE3W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xED\x82\x82a\x11\xBCV[PPPV[`\x06TC\x10a\t\x03Wa\t\x03a\x0F\xBAV[a\x08fa\x12wV[`\0\x80`\x05T\x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xDBL\xFD\xC7\x95\xED\xA1d\x07\xC0\x8D\xAB&\xED\xC5\x83\xF4~-\x10\xE9\xE5l%\x0FE6\x12@\x93\xE8x\x86\x86`@Qa\t\\\x92\x91\x90a\x19*V[`@Q\x80\x91\x03\x90\xA3a\tla\x08\xF2V[\x93\x92PPPV[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\n\x0BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[\x82\x81\x14a\n\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[`\x05T`\x08T\x14\x15\x80\x15a\n\xAEWP`\x08T\x85\x14[a\x0B\x14W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCan only score previous round\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[`\0\x80`\0[\x85\x81\x10\x15a\x0B\xE6W`\0\x85\x85\x83\x81\x81\x10a\x0B6Wa\x0B6a\x19wV[\x90P` \x02\x015\x90P`\0\x88\x88\x84\x81\x81\x10a\x0BSWa\x0BSa\x19wV[\x90P` \x02\x01` \x81\x01\x90a\x0Bh\x91\x90a\x17\xA5V[\x90Pa\x0Bt\x82\x86a\x19\xD5V[\x94P`\0f\x03\x8D~\xA4\xC6\x80\0`\nT\x84a\x0B\x8E\x91\x90a\x19\xE8V[a\x0B\x98\x91\x90a\x19\xFFV[\x90Pa\xDE\xADs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x0B\xD0Wa\x0B\xC3\x82\x82a\x14{V[a\x0B\xCD\x81\x86a\x19\xD5V[\x94P[PPP\x80\x80a\x0B\xDE\x90a\x1A:V[\x91PPa\x0B\x1AV[Pf\x03\x8D~\xA4\xC6\x80\0`\tT\x83a\x0B\xFD\x91\x90a\x19\xD5V[\x11\x15a\x0C\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSum of scores including historic`D\x82\x01R\x7F too big\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[\x81`\t`\0\x82\x82Ta\x0C\x9D\x91\x90a\x19\xD5V[\x92PP\x81\x90UP\x80`\r`\0\x82\x82Ta\x0C\xB6\x91\x90a\x19\xD5V[\x90\x91UPPPPPPPPPV[`\x03\x81\x81T\x81\x10a\x0C\xD4W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[`\x02\x81\x81T\x81\x10a\x0C\xD4W`\0\x80\xFD[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\r\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[a\r\xABa\x15rV[a\r\xB3a\x12wV[`\r`\0\x82\x82Ta\r\xC4\x91\x90a\x1ArV[\x90\x91UPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\r\xE6\x81a\x10\xB6V[a\x08\x8E\x83\x83a\x11\xBCV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0E\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[`\x0CUV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0F%W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[`\0\x81\x11a\x0F\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNext round length must be positi`D\x82\x01R\x7Fve\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[`\x0BUV[`\0f\x03\x8D~\xA4\xC6\x80\0`\tTf\x03\x8D~\xA4\xC6\x80\0a\x0F\xD9\x91\x90a\x1ArV[`\nTa\x0F\xE6\x91\x90a\x19\xE8V[a\x0F\xF0\x91\x90a\x19\xFFV[\x90P`\0`\x07T\x82`\rTGa\x10\x06\x91\x90a\x1ArV[a\x10\x10\x91\x90a\x1ArV[a\x10\x1A\x91\x90a\x1ArV[\x90P`\0`\x0CT\x82\x10a\x10/W`\x0CTa\x101V[\x81[`\x05T`\x08U`\0`\tU`\x07T`\nU`\x06T\x90\x91P\x15a\x10`W`\x05Ta\x10[\x90`\x01a\x19\xD5V[a\x10cV[`\0[`\x05U`\x0BTa\x10s\x90Ca\x19\xD5V[`\x06U`\x07\x81\x90U`\x05T`@Q\x90\x81R\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4s\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[a\x08f\x813a\x16\x1FV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x11\xB4W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x11R3\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x07\x18V[P`\0a\x07\x18V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x11\xB4W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x07\x18V[`\x03T`\0\x90\x81\x90\x81[\x81\x81\x10\x80\x15a\x12\x91WP`\x04T\x81\x10[\x15a\x14sW`\x03\x80T`\0\x91\x90a\x12\xAA\x90`\x01\x90a\x1ArV[\x81T\x81\x10a\x12\xBAWa\x12\xBAa\x19wV[`\0\x91\x82R` \x90\x91 \x01T`\x03\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92P\x90\x80a\x12\xF2Wa\x12\xF2a\x1A\x85V[`\0\x82\x81R` \x80\x82 \x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x90\x92\x01\x90\x92Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x82R`\x01\x90R`@\x81 \x80T\x91\x90Ua\x13~\x81\x86a\x19\xD5V[`@Q\x90\x95Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15a\x14\rW\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fi\xCA\x02\xDDN\xDD{\xF0\xA4\xAB\xB9\xED;z\xF3\xF1Gx\xDB]a\x92\x1C}\xC7\xCDTRf2m\xE2\x82`@Qa\x14\0\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x14^V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1CC\xB9v\x1B?\xBAS!\xCA\x82\x12\xBF\xC21\x94_f\x8C\xCC\x0CDo39\x99\xEE\xA9\xCE\x8F\xDA\x81\x82`@Qa\x14U\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PP\x80\x80a\x14k\x90a\x1A:V[\x91PPa\x12\x81V[P\x90\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x90a\x14\xAC\x83\x83a\x19\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 \x81\x90U\x90Pg\x06\xF0[Y\xD3\xB2\0\0\x82\x11\x80\x15\x90a\x14\xF5WPg\x06\xF0[Y\xD3\xB2\0\0\x81\x11[\x15a\x08\x8EW`\x02\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPPPV[`\x03T\x15a\x16\x02W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FScheduled transfers still pendin`D\x82\x01R\x7Fg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[`\x02\x80Ta\x16\x12\x91`\x03\x91a\x16\xA9V[Pa\x07\xC3`\x02`\0a\x16\xF9V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x16\xA5W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x07\xB2V[PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x16\xE9W`\0R` `\0 \x91\x82\x01[\x82\x81\x11\x15a\x16\xE9W\x82T\x82U\x91`\x01\x01\x91\x90`\x01\x01\x90a\x16\xCEV[Pa\x16\xF5\x92\x91Pa\x17\x13V[P\x90V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x08f\x91\x90[[\x80\x82\x11\x15a\x16\xF5W`\0\x81U`\x01\x01a\x17\x14V[`\0` \x82\x84\x03\x12\x15a\x17:W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\tlW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17|W`\0\x80\xFD[P5\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08fW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17\xB7W`\0\x80\xFD[\x815a\tl\x81a\x17\x83V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xD5W`\0\x80\xFD[\x825\x91P` \x83\x015a\x17\xE7\x81a\x17\x83V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x18\x05W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\x1DW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x181W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18@W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x18RW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x18vW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x8EW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x18\xA9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x18\xC8W`\0\x80\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xE7W`\0\x80\xFD[a\x18\xF3\x89\x83\x8A\x01a\x18dV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x19\x0CW`\0\x80\xFD[Pa\x19\x19\x88\x82\x89\x01a\x18dV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07\x18Wa\x07\x18a\x19\xA6V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07\x18Wa\x07\x18a\x19\xA6V[`\0\x82a\x1A5W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1AkWa\x1Aka\x19\xA6V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x07\x18Wa\x07\x18a\x19\xA6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xF9v?\x9A5\xAE\xF6\x80\x96\xF4X\xABA\xE5\x19\xCCO\x01\xFFN\xDF\x02o\xEFy\x01\x1F9\xEAw\x87_dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static IMPACTEVALUATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02\x1DW`\x005`\xE0\x1C\x80cbLk\xE7\x11a\x01\x1DW\x80c\xA2\x17\xFD\xDF\x11a\0\xB0W\x80c\xD3\x12\x03R\x11a\0\x7FW\x80c\xDA\x84\\\xE0\x11a\0dW\x80c\xDA\x84\\\xE0\x14a\x06/W\x80c\xF0\x93+\xD7\x14a\x06OW\x80c\xF1\x99\xF5m\x14a\x06oW`\0\x80\xFD[\x80c\xD3\x12\x03R\x14a\x05\xFAW\x80c\xD5Gt\x1F\x14a\x06\x0FW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x05\x9AW\x80c\xB5}\x01.\x14a\x05\xAFW\x80c\xCA\xD5V\\\x14a\x05\xC5W\x80c\xD0\xFA\xB3\xCA\x14a\x05\xE5W`\0\x80\xFD[\x80c\x91\xD1HT\x11a\0\xECW\x80c\x91\xD1HT\x14a\x04\xD8W\x80c\x93\x84\xC8\x85\x14a\x05)W\x80c\x93\xFE\xDDa\x14a\x05?W\x80c\x99\x1B\"\x08\x14a\x05\x84W`\0\x80\xFD[\x80cbLk\xE7\x14a\x04bW\x80ceS}\xE3\x14a\x04xW\x80ch\x96\xEFK\x14a\x04\x8EW\x80cxs\xCD\xE2\x14a\x04\xA4W`\0\x80\xFD[\x80c*Nbz\x11a\x01\xB0W\x80c6V\x8A\xBE\x11a\x01\x7FW\x80cAs9;\x11a\x01dW\x80cAs9;\x14a\x04\x06W\x80cU5\xDB\xF6\x14a\x04\"W\x80c`1\x19\xB1\x14a\x04BW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x03\xD1W\x80c>\xAF]\x9F\x14a\x03\xF1W`\0\x80\xFD[\x80c*Nbz\x14a\x03BW\x80c//\xF1]\x14a\x03XW\x80c0\xCB\x93\xB5\x14a\x03xW\x80c1s\xC2\x88\x14a\x03\x8EW`\0\x80\xFD[\x80c\x1D,\x93\xD8\x11a\x01\xECW\x80c\x1D,\x93\xD8\x14a\x02\xAAW\x80c$\x8A\x9C\xA3\x14a\x02\xCAW\x80c'\xE25\xE3\x14a\x02\xFAW\x80c'\xFFb#\x14a\x03'W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02)W\x80c\x05\xF3xS\x14a\x02^W\x80c\x0F\x88\x13&\x14a\x02uW\x80c\x1C\x9B(\xBF\x14a\x02\x94W`\0\x80\xFD[6a\x02$W\0[`\0\x80\xFD[4\x80\x15a\x025W`\0\x80\xFD[Pa\x02Ia\x02D6`\x04a\x17(V[a\x06\x85V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW`\0\x80\xFD[Pa\x02sa\x07\x1EV[\0[4\x80\x15a\x02\x81W`\0\x80\xFD[P`\x03T[`@Q\x90\x81R` \x01a\x02UV[4\x80\x15a\x02\xA0W`\0\x80\xFD[Pa\x02\x86`\x07T\x81V[4\x80\x15a\x02\xB6W`\0\x80\xFD[Pa\x02sa\x02\xC56`\x04a\x17jV[a\x07\xC5V[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02\x86a\x02\xE56`\x04a\x17jV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x03\x06W`\0\x80\xFD[Pa\x02\x86a\x03\x156`\x04a\x17\xA5V[`\x01` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x033W`\0\x80\xFD[Pa\x02\x86f\x03\x8D~\xA4\xC6\x80\0\x81V[4\x80\x15a\x03NW`\0\x80\xFD[Pa\x02\x86`\x06T\x81V[4\x80\x15a\x03dW`\0\x80\xFD[Pa\x02sa\x03s6`\x04a\x17\xC2V[a\x08iV[4\x80\x15a\x03\x84W`\0\x80\xFD[Pa\x02\x86`\nT\x81V[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x02\x86a\x03\xA96`\x04a\x17\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[4\x80\x15a\x03\xDDW`\0\x80\xFD[Pa\x02sa\x03\xEC6`\x04a\x17\xC2V[a\x08\x94V[4\x80\x15a\x03\xFDW`\0\x80\xFD[Pa\x02sa\x08\xF2V[4\x80\x15a\x04\x12W`\0\x80\xFD[Pa\x02\x86g\x06\xF0[Y\xD3\xB2\0\0\x81V[4\x80\x15a\x04.W`\0\x80\xFD[Pa\x02\x86a\x04=6`\x04a\x17\xF2V[a\t\x0BV[4\x80\x15a\x04NW`\0\x80\xFD[Pa\x02sa\x04]6`\x04a\x18\xB0V[a\tsV[4\x80\x15a\x04nW`\0\x80\xFD[Pa\x02\x86`\rT\x81V[4\x80\x15a\x04\x84W`\0\x80\xFD[Pa\x02\x86`\x04T\x81V[4\x80\x15a\x04\x9AW`\0\x80\xFD[Pa\x02\x86`\x05T\x81V[4\x80\x15a\x04\xB0W`\0\x80\xFD[Pa\x02\x86\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x04\xE4W`\0\x80\xFD[Pa\x02Ia\x04\xF36`\x04a\x17\xC2V[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x055W`\0\x80\xFD[Pa\x02\x86`\x0BT\x81V[4\x80\x15a\x05KW`\0\x80\xFD[Pa\x05_a\x05Z6`\x04a\x17jV[a\x0C\xC4V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x05\x90W`\0\x80\xFD[Pa\x02\x86`\x08T\x81V[4\x80\x15a\x05\xA6W`\0\x80\xFD[Pa\x02\x86`\0\x81V[4\x80\x15a\x05\xBBW`\0\x80\xFD[Pa\x02\x86`\tT\x81V[4\x80\x15a\x05\xD1W`\0\x80\xFD[Pa\x05_a\x05\xE06`\x04a\x17jV[a\x0C\xFBV[4\x80\x15a\x05\xF1W`\0\x80\xFD[Pa\x02sa\r\x0BV[4\x80\x15a\x06\x06W`\0\x80\xFD[P`\x02Ta\x02\x86V[4\x80\x15a\x06\x1BW`\0\x80\xFD[Pa\x02sa\x06*6`\x04a\x17\xC2V[a\r\xCBV[4\x80\x15a\x06;W`\0\x80\xFD[Pa\x02sa\x06J6`\x04a\x17jV[a\r\xF0V[4\x80\x15a\x06[W`\0\x80\xFD[Pa\x02sa\x06j6`\x04a\x17jV[a\x0E\x8DV[4\x80\x15a\x06{W`\0\x80\xFD[Pa\x02\x86`\x0CT\x81V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x07\x18WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x07\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\xC3a\x0F\xBAV[V[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x08]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[a\x08f\x81`\x04UV[PV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x84\x81a\x10\xB6V[a\x08\x8E\x83\x83a\x10\xC0V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x08\xE3W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xED\x82\x82a\x11\xBCV[PPPV[`\x06TC\x10a\t\x03Wa\t\x03a\x0F\xBAV[a\x08fa\x12wV[`\0\x80`\x05T\x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xDBL\xFD\xC7\x95\xED\xA1d\x07\xC0\x8D\xAB&\xED\xC5\x83\xF4~-\x10\xE9\xE5l%\x0FE6\x12@\x93\xE8x\x86\x86`@Qa\t\\\x92\x91\x90a\x19*V[`@Q\x80\x91\x03\x90\xA3a\tla\x08\xF2V[\x93\x92PPPV[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\n\x0BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[\x82\x81\x14a\n\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[`\x05T`\x08T\x14\x15\x80\x15a\n\xAEWP`\x08T\x85\x14[a\x0B\x14W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FCan only score previous round\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[`\0\x80`\0[\x85\x81\x10\x15a\x0B\xE6W`\0\x85\x85\x83\x81\x81\x10a\x0B6Wa\x0B6a\x19wV[\x90P` \x02\x015\x90P`\0\x88\x88\x84\x81\x81\x10a\x0BSWa\x0BSa\x19wV[\x90P` \x02\x01` \x81\x01\x90a\x0Bh\x91\x90a\x17\xA5V[\x90Pa\x0Bt\x82\x86a\x19\xD5V[\x94P`\0f\x03\x8D~\xA4\xC6\x80\0`\nT\x84a\x0B\x8E\x91\x90a\x19\xE8V[a\x0B\x98\x91\x90a\x19\xFFV[\x90Pa\xDE\xADs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x0B\xD0Wa\x0B\xC3\x82\x82a\x14{V[a\x0B\xCD\x81\x86a\x19\xD5V[\x94P[PPP\x80\x80a\x0B\xDE\x90a\x1A:V[\x91PPa\x0B\x1AV[Pf\x03\x8D~\xA4\xC6\x80\0`\tT\x83a\x0B\xFD\x91\x90a\x19\xD5V[\x11\x15a\x0C\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSum of scores including historic`D\x82\x01R\x7F too big\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[\x81`\t`\0\x82\x82Ta\x0C\x9D\x91\x90a\x19\xD5V[\x92PP\x81\x90UP\x80`\r`\0\x82\x82Ta\x0C\xB6\x91\x90a\x19\xD5V[\x90\x91UPPPPPPPPPV[`\x03\x81\x81T\x81\x10a\x0C\xD4W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[`\x02\x81\x81T\x81\x10a\x0C\xD4W`\0\x80\xFD[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\r\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[a\r\xABa\x15rV[a\r\xB3a\x12wV[`\r`\0\x82\x82Ta\r\xC4\x91\x90a\x1ArV[\x90\x91UPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\r\xE6\x81a\x10\xB6V[a\x08\x8E\x83\x83a\x11\xBCV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0E\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[`\x0CUV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x0F%W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xB2V[`\0\x81\x11a\x0F\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FNext round length must be positi`D\x82\x01R\x7Fve\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[`\x0BUV[`\0f\x03\x8D~\xA4\xC6\x80\0`\tTf\x03\x8D~\xA4\xC6\x80\0a\x0F\xD9\x91\x90a\x1ArV[`\nTa\x0F\xE6\x91\x90a\x19\xE8V[a\x0F\xF0\x91\x90a\x19\xFFV[\x90P`\0`\x07T\x82`\rTGa\x10\x06\x91\x90a\x1ArV[a\x10\x10\x91\x90a\x1ArV[a\x10\x1A\x91\x90a\x1ArV[\x90P`\0`\x0CT\x82\x10a\x10/W`\x0CTa\x101V[\x81[`\x05T`\x08U`\0`\tU`\x07T`\nU`\x06T\x90\x91P\x15a\x10`W`\x05Ta\x10[\x90`\x01a\x19\xD5V[a\x10cV[`\0[`\x05U`\x0BTa\x10s\x90Ca\x19\xD5V[`\x06U`\x07\x81\x90U`\x05T`@Q\x90\x81R\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4s\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[a\x08f\x813a\x16\x1FV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x11\xB4W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x11R3\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x07\x18V[P`\0a\x07\x18V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x11\xB4W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x07\x18V[`\x03T`\0\x90\x81\x90\x81[\x81\x81\x10\x80\x15a\x12\x91WP`\x04T\x81\x10[\x15a\x14sW`\x03\x80T`\0\x91\x90a\x12\xAA\x90`\x01\x90a\x1ArV[\x81T\x81\x10a\x12\xBAWa\x12\xBAa\x19wV[`\0\x91\x82R` \x90\x91 \x01T`\x03\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92P\x90\x80a\x12\xF2Wa\x12\xF2a\x1A\x85V[`\0\x82\x81R` \x80\x82 \x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x90\x92\x01\x90\x92Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x82R`\x01\x90R`@\x81 \x80T\x91\x90Ua\x13~\x81\x86a\x19\xD5V[`@Q\x90\x95Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15a\x14\rW\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fi\xCA\x02\xDDN\xDD{\xF0\xA4\xAB\xB9\xED;z\xF3\xF1Gx\xDB]a\x92\x1C}\xC7\xCDTRf2m\xE2\x82`@Qa\x14\0\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x14^V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1CC\xB9v\x1B?\xBAS!\xCA\x82\x12\xBF\xC21\x94_f\x8C\xCC\x0CDo39\x99\xEE\xA9\xCE\x8F\xDA\x81\x82`@Qa\x14U\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PP\x80\x80a\x14k\x90a\x1A:V[\x91PPa\x12\x81V[P\x90\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x90a\x14\xAC\x83\x83a\x19\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 \x81\x90U\x90Pg\x06\xF0[Y\xD3\xB2\0\0\x82\x11\x80\x15\x90a\x14\xF5WPg\x06\xF0[Y\xD3\xB2\0\0\x81\x11[\x15a\x08\x8EW`\x02\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPPPV[`\x03T\x15a\x16\x02W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FScheduled transfers still pendin`D\x82\x01R\x7Fg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xB2V[`\x02\x80Ta\x16\x12\x91`\x03\x91a\x16\xA9V[Pa\x07\xC3`\x02`\0a\x16\xF9V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x16\xA5W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x07\xB2V[PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x16\xE9W`\0R` `\0 \x91\x82\x01[\x82\x81\x11\x15a\x16\xE9W\x82T\x82U\x91`\x01\x01\x91\x90`\x01\x01\x90a\x16\xCEV[Pa\x16\xF5\x92\x91Pa\x17\x13V[P\x90V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x08f\x91\x90[[\x80\x82\x11\x15a\x16\xF5W`\0\x81U`\x01\x01a\x17\x14V[`\0` \x82\x84\x03\x12\x15a\x17:W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\tlW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17|W`\0\x80\xFD[P5\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08fW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17\xB7W`\0\x80\xFD[\x815a\tl\x81a\x17\x83V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xD5W`\0\x80\xFD[\x825\x91P` \x83\x015a\x17\xE7\x81a\x17\x83V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x18\x05W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\x1DW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x181W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18@W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x18RW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x18vW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x8EW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x18\xA9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x18\xC8W`\0\x80\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xE7W`\0\x80\xFD[a\x18\xF3\x89\x83\x8A\x01a\x18dV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x19\x0CW`\0\x80\xFD[Pa\x19\x19\x88\x82\x89\x01a\x18dV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07\x18Wa\x07\x18a\x19\xA6V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07\x18Wa\x07\x18a\x19\xA6V[`\0\x82a\x1A5W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1AkWa\x1Aka\x19\xA6V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x07\x18Wa\x07\x18a\x19\xA6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xF9v?\x9A5\xAE\xF6\x80\x96\xF4X\xABA\xE5\x19\xCCO\x01\xFFN\xDF\x02o\xEFy\x01\x1F9\xEAw\x87_dsolcC\0\x08\x15\x003";
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
        pub fn max_score(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
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
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], p0)
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
        ///Calls the contract's `maxTransfersPerTx` (0x65537de3) function
        pub fn max_transfers_per_tx(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([101, 83, 125, 227], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minBalanceForTransfer` (0x4173393b) function
        pub fn min_balance_for_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 115, 57, 59], ())
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
        ///Calls the contract's `participantCountReadyForTransfer` (0xd3120352) function
        pub fn participant_count_ready_for_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([211, 18, 3, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `participantCountScheduledForTransfer` (0x0f881326) function
        pub fn participant_count_scheduled_for_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 136, 19, 38], ())
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
        ///Calls the contract's `readyForTransfer` (0xcad5565c) function
        pub fn ready_for_transfer(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([202, 213, 86, 92], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `releaseRewards` (0xd0fab3ca) function
        pub fn release_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 250, 179, 202], ())
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
        ///Calls the contract's `rewardsScheduledFor` (0x3173c288) function
        pub fn rewards_scheduled_for(
            &self,
            participant: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 115, 194, 136], participant)
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
        ///Calls the contract's `scheduledForTransfer` (0x93fedd61) function
        pub fn scheduled_for_transfer(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([147, 254, 221, 97], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxTransfersPerTx` (0x1d2c93d8) function
        pub fn set_max_transfers_per_tx(
            &self,
            max_transfers_per_tx: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 44, 147, 216], max_transfers_per_tx)
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
        ///Calls the contract's `setScores` (0x603119b1) function
        pub fn set_scores(
            &self,
            round_index: ::ethers::core::types::U256,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            scores: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 49, 25, 177], (round_index, addresses, scores))
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
        ///Calls the contract's `tick` (0x3eaf5d9f) function
        pub fn tick(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 175, 93, 159], ())
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
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TransferFailed` event
        pub fn transfer_failed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFailedFilter,
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ImpactEvaluatorErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
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
        #[ethevent(indexed)]
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "TransferFailed", abi = "TransferFailed(address,uint256)")]
    pub struct TransferFailedFilter {
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ImpactEvaluatorEvents {
        MeasurementsAddedFilter(MeasurementsAddedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        RoundStartFilter(RoundStartFilter),
        TransferFilter(TransferFilter),
        TransferFailedFilter(TransferFailedFilter),
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
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TransferFailedFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::TransferFailedFilter(decoded));
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
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFailedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<TransferFilter> for ImpactEvaluatorEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<TransferFailedFilter> for ImpactEvaluatorEvents {
        fn from(value: TransferFailedFilter) -> Self {
            Self::TransferFailedFilter(value)
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
    ///Container type for all input parameters for the `maxTransfersPerTx` function with signature `maxTransfersPerTx()` and selector `0x65537de3`
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
    #[ethcall(name = "maxTransfersPerTx", abi = "maxTransfersPerTx()")]
    pub struct MaxTransfersPerTxCall;
    ///Container type for all input parameters for the `minBalanceForTransfer` function with signature `minBalanceForTransfer()` and selector `0x4173393b`
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
    #[ethcall(name = "minBalanceForTransfer", abi = "minBalanceForTransfer()")]
    pub struct MinBalanceForTransferCall;
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
    ///Container type for all input parameters for the `participantCountReadyForTransfer` function with signature `participantCountReadyForTransfer()` and selector `0xd3120352`
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
        name = "participantCountReadyForTransfer",
        abi = "participantCountReadyForTransfer()"
    )]
    pub struct ParticipantCountReadyForTransferCall;
    ///Container type for all input parameters for the `participantCountScheduledForTransfer` function with signature `participantCountScheduledForTransfer()` and selector `0x0f881326`
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
        name = "participantCountScheduledForTransfer",
        abi = "participantCountScheduledForTransfer()"
    )]
    pub struct ParticipantCountScheduledForTransferCall;
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
    ///Container type for all input parameters for the `readyForTransfer` function with signature `readyForTransfer(uint256)` and selector `0xcad5565c`
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
    #[ethcall(name = "readyForTransfer", abi = "readyForTransfer(uint256)")]
    pub struct ReadyForTransferCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `releaseRewards` function with signature `releaseRewards()` and selector `0xd0fab3ca`
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
    #[ethcall(name = "releaseRewards", abi = "releaseRewards()")]
    pub struct ReleaseRewardsCall;
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
    ///Container type for all input parameters for the `rewardsScheduledFor` function with signature `rewardsScheduledFor(address)` and selector `0x3173c288`
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
    #[ethcall(name = "rewardsScheduledFor", abi = "rewardsScheduledFor(address)")]
    pub struct RewardsScheduledForCall {
        pub participant: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `scheduledForTransfer` function with signature `scheduledForTransfer(uint256)` and selector `0x93fedd61`
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
    #[ethcall(name = "scheduledForTransfer", abi = "scheduledForTransfer(uint256)")]
    pub struct ScheduledForTransferCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `setMaxTransfersPerTx` function with signature `setMaxTransfersPerTx(uint256)` and selector `0x1d2c93d8`
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
    #[ethcall(name = "setMaxTransfersPerTx", abi = "setMaxTransfersPerTx(uint256)")]
    pub struct SetMaxTransfersPerTxCall {
        pub max_transfers_per_tx: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `setScores` function with signature `setScores(uint256,address[],uint256[])` and selector `0x603119b1`
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
    #[ethcall(name = "setScores", abi = "setScores(uint256,address[],uint256[])")]
    pub struct SetScoresCall {
        pub round_index: ::ethers::core::types::U256,
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub scores: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all input parameters for the `tick` function with signature `tick()` and selector `0x3eaf5d9f`
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
    #[ethcall(name = "tick", abi = "tick()")]
    pub struct TickCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ImpactEvaluatorCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EvaluateRole(EvaluateRoleCall),
        MaxScore(MaxScoreCall),
        AddMeasurements(AddMeasurementsCall),
        AdminAdvanceRound(AdminAdvanceRoundCall),
        BalanceHeld(BalanceHeldCall),
        Balances(BalancesCall),
        CurrentRoundEndBlockNumber(CurrentRoundEndBlockNumberCall),
        CurrentRoundIndex(CurrentRoundIndexCall),
        CurrentRoundRoundReward(CurrentRoundRoundRewardCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        MaxTransfersPerTx(MaxTransfersPerTxCall),
        MinBalanceForTransfer(MinBalanceForTransferCall),
        NextRoundLength(NextRoundLengthCall),
        ParticipantCountReadyForTransfer(ParticipantCountReadyForTransferCall),
        ParticipantCountScheduledForTransfer(ParticipantCountScheduledForTransferCall),
        PreviousRoundIndex(PreviousRoundIndexCall),
        PreviousRoundRoundReward(PreviousRoundRoundRewardCall),
        PreviousRoundTotalScores(PreviousRoundTotalScoresCall),
        ReadyForTransfer(ReadyForTransferCall),
        ReleaseRewards(ReleaseRewardsCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RewardsScheduledFor(RewardsScheduledForCall),
        RoundReward(RoundRewardCall),
        ScheduledForTransfer(ScheduledForTransferCall),
        SetMaxTransfersPerTx(SetMaxTransfersPerTxCall),
        SetNextRoundLength(SetNextRoundLengthCall),
        SetRoundReward(SetRoundRewardCall),
        SetScores(SetScoresCall),
        SupportsInterface(SupportsInterfaceCall),
        Tick(TickCall),
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
                = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
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
                = <MaxTransfersPerTxCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxTransfersPerTx(decoded));
            }
            if let Ok(decoded)
                = <MinBalanceForTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MinBalanceForTransfer(decoded));
            }
            if let Ok(decoded)
                = <NextRoundLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NextRoundLength(decoded));
            }
            if let Ok(decoded)
                = <ParticipantCountReadyForTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ParticipantCountReadyForTransfer(decoded));
            }
            if let Ok(decoded)
                = <ParticipantCountScheduledForTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ParticipantCountScheduledForTransfer(decoded));
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
                = <ReadyForTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReadyForTransfer(decoded));
            }
            if let Ok(decoded)
                = <ReleaseRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReleaseRewards(decoded));
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
                = <RewardsScheduledForCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RewardsScheduledFor(decoded));
            }
            if let Ok(decoded)
                = <RoundRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RoundReward(decoded));
            }
            if let Ok(decoded)
                = <ScheduledForTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ScheduledForTransfer(decoded));
            }
            if let Ok(decoded)
                = <SetMaxTransfersPerTxCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMaxTransfersPerTx(decoded));
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
                = <TickCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Tick(decoded));
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
                Self::Balances(element) => {
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
                Self::MaxTransfersPerTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinBalanceForTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextRoundLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParticipantCountReadyForTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParticipantCountScheduledForTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviousRoundIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviousRoundRoundReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviousRoundTotalScores(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadyForTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardsScheduledFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoundReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ScheduledForTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxTransfersPerTx(element) => {
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
                Self::Tick(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::MaxTransfersPerTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinBalanceForTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NextRoundLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParticipantCountReadyForTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParticipantCountScheduledForTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PreviousRoundIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PreviousRoundRoundReward(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PreviousRoundTotalScores(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReadyForTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardsScheduledFor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoundReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::ScheduledForTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxTransfersPerTx(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetNextRoundLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRoundReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetScores(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tick(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<BalancesCall> for ImpactEvaluatorCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
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
    impl ::core::convert::From<MaxTransfersPerTxCall> for ImpactEvaluatorCalls {
        fn from(value: MaxTransfersPerTxCall) -> Self {
            Self::MaxTransfersPerTx(value)
        }
    }
    impl ::core::convert::From<MinBalanceForTransferCall> for ImpactEvaluatorCalls {
        fn from(value: MinBalanceForTransferCall) -> Self {
            Self::MinBalanceForTransfer(value)
        }
    }
    impl ::core::convert::From<NextRoundLengthCall> for ImpactEvaluatorCalls {
        fn from(value: NextRoundLengthCall) -> Self {
            Self::NextRoundLength(value)
        }
    }
    impl ::core::convert::From<ParticipantCountReadyForTransferCall>
    for ImpactEvaluatorCalls {
        fn from(value: ParticipantCountReadyForTransferCall) -> Self {
            Self::ParticipantCountReadyForTransfer(value)
        }
    }
    impl ::core::convert::From<ParticipantCountScheduledForTransferCall>
    for ImpactEvaluatorCalls {
        fn from(value: ParticipantCountScheduledForTransferCall) -> Self {
            Self::ParticipantCountScheduledForTransfer(value)
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
    impl ::core::convert::From<ReadyForTransferCall> for ImpactEvaluatorCalls {
        fn from(value: ReadyForTransferCall) -> Self {
            Self::ReadyForTransfer(value)
        }
    }
    impl ::core::convert::From<ReleaseRewardsCall> for ImpactEvaluatorCalls {
        fn from(value: ReleaseRewardsCall) -> Self {
            Self::ReleaseRewards(value)
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
    impl ::core::convert::From<RewardsScheduledForCall> for ImpactEvaluatorCalls {
        fn from(value: RewardsScheduledForCall) -> Self {
            Self::RewardsScheduledFor(value)
        }
    }
    impl ::core::convert::From<RoundRewardCall> for ImpactEvaluatorCalls {
        fn from(value: RoundRewardCall) -> Self {
            Self::RoundReward(value)
        }
    }
    impl ::core::convert::From<ScheduledForTransferCall> for ImpactEvaluatorCalls {
        fn from(value: ScheduledForTransferCall) -> Self {
            Self::ScheduledForTransfer(value)
        }
    }
    impl ::core::convert::From<SetMaxTransfersPerTxCall> for ImpactEvaluatorCalls {
        fn from(value: SetMaxTransfersPerTxCall) -> Self {
            Self::SetMaxTransfersPerTx(value)
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
    impl ::core::convert::From<TickCall> for ImpactEvaluatorCalls {
        fn from(value: TickCall) -> Self {
            Self::Tick(value)
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
    pub struct MaxScoreReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `maxTransfersPerTx` function with signature `maxTransfersPerTx()` and selector `0x65537de3`
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
    pub struct MaxTransfersPerTxReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minBalanceForTransfer` function with signature `minBalanceForTransfer()` and selector `0x4173393b`
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
    pub struct MinBalanceForTransferReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `participantCountReadyForTransfer` function with signature `participantCountReadyForTransfer()` and selector `0xd3120352`
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
    pub struct ParticipantCountReadyForTransferReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `participantCountScheduledForTransfer` function with signature `participantCountScheduledForTransfer()` and selector `0x0f881326`
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
    pub struct ParticipantCountScheduledForTransferReturn(
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `readyForTransfer` function with signature `readyForTransfer(uint256)` and selector `0xcad5565c`
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
    pub struct ReadyForTransferReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewardsScheduledFor` function with signature `rewardsScheduledFor(address)` and selector `0x3173c288`
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
    pub struct RewardsScheduledForReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `scheduledForTransfer` function with signature `scheduledForTransfer(uint256)` and selector `0x93fedd61`
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
    pub struct ScheduledForTransferReturn(pub ::ethers::core::types::Address);
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
}
