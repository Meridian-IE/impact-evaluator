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
                    ::std::borrow::ToOwned::to_owned("currentRoundMeasurementCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentRoundMeasurementCount",
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
                    ::std::borrow::ToOwned::to_owned("getParticipantScore"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getParticipantScore",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getRoundEnd"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoundEnd"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoundExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoundExists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoundMeasurementsCids"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRoundMeasurementsCids",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoundScoresSubmitted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRoundScoresSubmitted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoundSummaryText"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRoundSummaryText",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("maxStoredRounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxStoredRounds"),
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
                    ::std::borrow::ToOwned::to_owned("rounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rounds"),
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
                                    name: ::std::borrow::ToOwned::to_owned("end"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("scoresSubmitted"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("summaryText"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exists"),
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
                    ::std::borrow::ToOwned::to_owned("setMaxStoredRounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaxStoredRounds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxStoredRounds"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("summaryText"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                                    indexed: false,
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
    const __BYTECODE: &[u8] = b"`\x80`@R`\n`\x02Uh\x05k\xC7^-c\x10\0\0`\x03Ua\x03\xE8`\x04U4\x80\x15b\0\0)W`\0\x80\xFD[P`@Qb\0#\xBC8\x03\x80b\0#\xBC\x839\x81\x01`@\x81\x90Rb\0\0L\x91b\0\x03(V[b\0\0Y`\0\x82b\0\0\x98V[Pb\0\0\x86\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x82b\0\0\x98V[Pb\0\0\x91b\0\x01GV[Pb\0\x03\xEEV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16b\0\x01=W`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\xF43\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01b\0\x01AV[P`\0[\x92\x91PPV[`\x01\x80T\x81\x01\x80\x82U`\0\x82\x81R\x91\x90b\0\x01d\x90\x82\x90b\0\x03pV[\x81T\x81\x10b\0\x01wWb\0\x01wb\0\x03\x86V[\x90`\0R` `\0 \x90`\x06\x02\x01\x90P`\x02TCb\0\x01\x97\x91\x90b\0\x03\x9CV[\x81U`\x05\x81\x01\x80T`\xFF\x19\x16`\x01\x17\x90U\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sb\0\x01\xD3b\0\x02vV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x04T`\x01T\x11\x15b\0\x02sW`\x01\x80`\x04T`\x01\x80T\x90Pb\0\x02\t\x91\x90b\0\x03pV[b\0\x02\x15\x91\x90b\0\x03pV[\x81T\x81\x10b\0\x02(Wb\0\x02(b\0\x03\x86V[`\0\x91\x82R` \x82 `\x06\x90\x91\x02\x01\x81\x81U\x90b\0\x02J`\x01\x83\x01\x82b\0\x02\x8DV[`\x03\x82\x01\x80T`\xFF\x19\x16\x90Ub\0\x02f`\x04\x83\x01`\0b\0\x02\xADV[P`\x05\x01\x80T`\xFF\x19\x16\x90U[PV[`\x01\x80T`\0\x91b\0\x02\x88\x91b\0\x03pV[\x90P\x90V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90b\0\x02s\x91\x90b\0\x02\xECV[P\x80Tb\0\x02\xBB\x90b\0\x03\xB2V[`\0\x82U\x80`\x1F\x10b\0\x02\xCCWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0\x02s\x91\x90b\0\x03\x11V[\x80\x82\x11\x15b\0\x03\rW`\0b\0\x03\x03\x82\x82b\0\x02\xADV[P`\x01\x01b\0\x02\xECV[P\x90V[[\x80\x82\x11\x15b\0\x03\rW`\0\x81U`\x01\x01b\0\x03\x12V[`\0` \x82\x84\x03\x12\x15b\0\x03;W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03SW`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15b\0\x01AWb\0\x01Ab\0\x03ZV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15b\0\x01AWb\0\x01Ab\0\x03ZV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03\xC7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\xE8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[a\x1F\xBE\x80b\0\x03\xFE`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xA5W`\x005`\xE0\x1C\x80c\x8EX\xCE$\x11a\0\xE1W\x80c\xA6\xBC\x94\xA0\x11a\0\x8AW\x80c\xDA\x84\\\xE0\x11a\0dW\x80c\xDA\x84\\\xE0\x14a\x04\xD5W\x80c\xE4\xC7T\x92\x14a\x04\xF5W\x80c\xF0\x93+\xD7\x14a\x05\x15W\x80c\xF1\x99\xF5m\x14a\x055W`\0\x80\xFD[\x80c\xA6\xBC\x94\xA0\x14a\x04\x80W\x80c\xC6H\x85\xEF\x14a\x04\xA0W\x80c\xD5Gt\x1F\x14a\x04\xB5W`\0\x80\xFD[\x80c\x94\x92\xC9|\x11a\0\xBBW\x80c\x94\x92\xC9|\x14a\x04\x1EW\x80c\x97:\x89\xB5\x14a\x04>W\x80c\xA2\x17\xFD\xDF\x14a\x04kW`\0\x80\xFD[\x80c\x8EX\xCE$\x14a\x03\x8AW\x80c\x91\xD1HT\x14a\x03\xB7W\x80c\x93\x84\xC8\x85\x14a\x04\x08W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01NW\x80ck\xF4\";\x11a\x01(W\x80ck\xF4\";\x14a\x02\xF0W\x80cxs\xCD\xE2\x14a\x03\x10W\x80c\x83\0-\xBA\x14a\x03DW\x80c\x8Ce\xC8\x1F\x14a\x03ZW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x02\x9BW\x80cU5\xDB\xF6\x14a\x02\xBBW\x80ch\x96\xEFK\x14a\x02\xDBW`\0\x80\xFD[\x80c\x19\xEE\xFA\xAB\x11a\x01\x7FW\x80c\x19\xEE\xFA\xAB\x14a\x02\x1DW\x80c$\x8A\x9C\xA3\x14a\x02=W\x80c//\xF1]\x14a\x02{W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xB1W\x80c\x05\x80\t\x93\x14a\x01\xE6W\x80c\x05\xF3xS\x14a\x02\x08W`\0\x80\xFD[6a\x01\xACW\0[`\0\x80\xFD[4\x80\x15a\x01\xBDW`\0\x80\xFD[Pa\x01\xD1a\x01\xCC6`\x04a\x17\xC0V[a\x05KV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF2W`\0\x80\xFD[Pa\x02\x06a\x02\x016`\x04a\x18\tV[a\x05\xE4V[\0[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x02\x06a\x07QV[4\x80\x15a\x02)W`\0\x80\xFD[Pa\x01\xD1a\x0286`\x04a\x18\tV[a\x07\xF3V[4\x80\x15a\x02IW`\0\x80\xFD[Pa\x02ma\x02X6`\x04a\x18\tV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\xDDV[4\x80\x15a\x02\x87W`\0\x80\xFD[Pa\x02\x06a\x02\x966`\x04a\x18DV[a\x08%V[4\x80\x15a\x02\xA7W`\0\x80\xFD[Pa\x02\x06a\x02\xB66`\x04a\x18DV[a\x08PV[4\x80\x15a\x02\xC7W`\0\x80\xFD[Pa\x02ma\x02\xD66`\x04a\x19\x80V[a\x08\xAEV[4\x80\x15a\x02\xE7W`\0\x80\xFD[Pa\x02ma\tDV[4\x80\x15a\x02\xFCW`\0\x80\xFD[Pa\x02ma\x03\x0B6`\x04a\x18\tV[a\tYV[4\x80\x15a\x03\x1CW`\0\x80\xFD[Pa\x02m\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x02m`\x04T\x81V[4\x80\x15a\x03fW`\0\x80\xFD[Pa\x03za\x03u6`\x04a\x18\tV[a\t\x87V[`@Qa\x01\xDD\x94\x93\x92\x91\x90a\x1A!V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x03\xAAa\x03\xA56`\x04a\x18\tV[a\nSV[`@Qa\x01\xDD\x91\x90a\x1AUV[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x01\xD1a\x03\xD26`\x04a\x18DV[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x04\x14W`\0\x80\xFD[Pa\x02m`\x02T\x81V[4\x80\x15a\x04*W`\0\x80\xFD[Pa\x02\x06a\x0496`\x04a\x1B{V[a\x0BPV[4\x80\x15a\x04JW`\0\x80\xFD[Pa\x04^a\x04Y6`\x04a\x18\tV[a\x0E\x84V[`@Qa\x01\xDD\x91\x90a\x1CmV[4\x80\x15a\x04wW`\0\x80\xFD[Pa\x02m`\0\x81V[4\x80\x15a\x04\x8CW`\0\x80\xFD[Pa\x02ma\x04\x9B6`\x04a\x18DV[a\x0F:V[4\x80\x15a\x04\xACW`\0\x80\xFD[Pa\x02ma\x0F\x97V[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x02\x06a\x04\xD06`\x04a\x18DV[a\x0F\xCDV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x02\x06a\x04\xF06`\x04a\x18\tV[a\x0F\xF2V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x01\xD1a\x05\x106`\x04a\x18\tV[a\x10\x8FV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\x06a\x0506`\x04a\x18\tV[a\x10\xC1V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02m`\x03T\x81V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x05\xDEWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x06\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x04T\x81\x10\x15a\x07LW`\0[`\x01Ta\x06\x9C\x90\x83\x90a\x1C\xAFV[\x81\x10\x15a\x07JW`\x01\x81\x81T\x81\x10a\x06\xB6Wa\x06\xB6a\x1C\xC2V[`\0\x91\x82R` \x82 `\x06\x90\x91\x02\x01\x81\x81U\x90a\x06\xD6`\x01\x83\x01\x82a\x172V[`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90Ua\x07\x0E`\x04\x83\x01`\0a\x17PV[P`\x05\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U\x80a\x07B\x81a\x1C\xF1V[\x91PPa\x06\x8EV[P[`\x04UV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x07\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[a\x07\xF1a\x11^V[V[`\0`\x01\x82\x81T\x81\x10a\x08\x08Wa\x08\x08a\x1C\xC2V[`\0\x91\x82R` \x90\x91 `\x05`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08@\x81a\x12\xD2V[a\x08J\x83\x83a\x12\xDCV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x08\x9FW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xA9\x82\x82a\x13\xD8V[PPPV[`\0a\x08\xB8a\x14\x93V[`\0a\x08\xC2a\tDV[\x90P`\x01\x81\x81T\x81\x10a\x08\xD7Wa\x08\xD7a\x1C\xC2V[`\0\x91\x82R` \x80\x83 `\x01`\x06\x90\x93\x02\x01\x82\x01\x80T\x92\x83\x01\x81U\x83R\x90\x91 \x01a\t\x02\x84\x82a\x1D\xCAV[P\x7F\xDBL\xFD\xC7\x95\xED\xA1d\x07\xC0\x8D\xAB&\xED\xC5\x83\xF4~-\x10\xE9\xE5l%\x0FE6\x12@\x93\xE8x\x83\x823`@Qa\t6\x93\x92\x91\x90a\x1E\xE4V[`@Q\x80\x91\x03\x90\xA1\x92\x91PPV[`\x01\x80T`\0\x91a\tT\x91a\x1C\xAFV[\x90P\x90V[`\0`\x01\x82\x81T\x81\x10a\tnWa\tna\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P\x91\x90PV[`\x01\x81\x81T\x81\x10a\t\x97W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x03\x82\x01T`\x04\x83\x01\x80T\x92\x94P`\xFF\x90\x91\x16\x92\x91a\t\xC7\x90a\x1D)V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF3\x90a\x1D)V[\x80\x15a\n@W\x80`\x1F\x10a\n\x15Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n@V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n#W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x05\x90\x93\x01T\x91\x92PP`\xFF\x16\x84V[```\x01\x82\x81T\x81\x10a\nhWa\nha\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0BEW\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\xB8\x90a\x1D)V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xE4\x90a\x1D)V[\x80\x15a\x0B1W\x80`\x1F\x10a\x0B\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\x99V[PPPP\x90P\x91\x90PV[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x0B\xE8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[\x81Q\x83Q\x14a\x0CxW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06xV[`\0`\x01\x85\x81T\x81\x10a\x0C\x8DWa\x0C\x8Da\x1C\xC2V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01`\x05\x81\x01T\x90\x91P`\xFF\x16a\r\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FRound does not exist\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\x03\x81\x01T`\xFF\x16\x15a\r}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FScores already submitted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\0[\x84Q\x81\x10\x15a\x0E6W\x83\x81\x81Q\x81\x10a\r\x9BWa\r\x9Ba\x1C\xC2V[` \x02` \x01\x01Q\x82`\x02\x01`\0\x87\x84\x81Q\x81\x10a\r\xBBWa\r\xBBa\x1C\xC2V[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80a\x0E.\x90a\x1C\xF1V[\x91PPa\r\x80V[P`\x04\x81\x01a\x0EE\x83\x82a\x1D\xCAV[P`\x03\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x0E}\x84\x84a\x14\xD2V[PPPPPV[```\x01\x82\x81T\x81\x10a\x0E\x99Wa\x0E\x99a\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x04\x01\x80Ta\x0E\xB5\x90a\x1D)V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\xE1\x90a\x1D)V[\x80\x15a\x0F.W\x80`\x1F\x10a\x0F\x03Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F.V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\x11W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0`\x01\x83\x81T\x81\x10a\x0FOWa\x0FOa\x1C\xC2V[`\0\x91\x82R` \x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R`\x02`\x06\x90\x93\x02\x01\x91\x90\x91\x01\x90R`@\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x92\x91PPV[`\0`\x01a\x0F\xA3a\tDV[\x81T\x81\x10a\x0F\xB3Wa\x0F\xB3a\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x01\x01\x80T\x90P\x90P\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xE8\x81a\x12\xD2V[a\x08J\x83\x83a\x13\xD8V[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x10\x8AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\x03UV[`\0`\x01\x82\x81T\x81\x10a\x10\xA4Wa\x10\xA4a\x1C\xC2V[`\0\x91\x82R` \x90\x91 `\x03`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x11YW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\x02UV[`\x01\x80T\x81\x01\x80\x82U`\0\x82\x81R\x91\x90a\x11y\x90\x82\x90a\x1C\xAFV[\x81T\x81\x10a\x11\x89Wa\x11\x89a\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01\x90P`\x02TCa\x11\xA7\x91\x90a\x1F#V[\x81U`\x05\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sa\x11\xFFa\tDV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x04T`\x01T\x11\x15a\x12\xCFW`\x01\x80`\x04T`\x01\x80T\x90Pa\x122\x91\x90a\x1C\xAFV[a\x12<\x91\x90a\x1C\xAFV[\x81T\x81\x10a\x12LWa\x12La\x1C\xC2V[`\0\x91\x82R` \x82 `\x06\x90\x91\x02\x01\x81\x81U\x90a\x12l`\x01\x83\x01\x82a\x172V[`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90Ua\x12\xA4`\x04\x83\x01`\0a\x17PV[P`\x05\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U[PV[a\x12\xCF\x813a\x16\xA8V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x13\xD0W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x13n3\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x05\xDEV[P`\0a\x05\xDEV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x13\xD0W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x05\xDEV[`\0`\x01a\x14\x9Fa\tDV[\x81T\x81\x10a\x14\xAFWa\x14\xAFa\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P\x80C\x10a\x12\xCFWa\x12\xCFa\x11^V[`\x03TG\x10\x15a\x15>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot enough funds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\0[\x82Q\x81\x10\x15a\x08\xA9W`\0\x83\x82\x81Q\x81\x10a\x15^Wa\x15^a\x1C\xC2V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x15|Wa\x15|a\x1C\xC2V[` \x02` \x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0f\x03\x8D~\xA4\xC6\x80\0`\x03T\x83a\x15\xA8\x91\x90a\x1F6V[a\x15\xB2\x91\x90a\x1FMV[`@Q\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15a\x16AW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fi\xCA\x02\xDDN\xDD{\xF0\xA4\xAB\xB9\xED;z\xF3\xF1Gx\xDB]a\x92\x1C}\xC7\xCDTRf2m\xE2\x82`@Qa\x164\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x16\x92V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1CC\xB9v\x1B?\xBAS!\xCA\x82\x12\xBF\xC21\x94_f\x8C\xCC\x0CDo39\x99\xEE\xA9\xCE\x8F\xDA\x81\x82`@Qa\x16\x89\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PPP\x80\x80a\x16\xA0\x90a\x1C\xF1V[\x91PPa\x15AV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x17.W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x06xV[PPV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x12\xCF\x91\x90a\x17\x8AV[P\x80Ta\x17\\\x90a\x1D)V[`\0\x82U\x80`\x1F\x10a\x17lWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x12\xCF\x91\x90a\x17\xABV[\x80\x82\x11\x15a\x17\xA7W`\0a\x17\x9E\x82\x82a\x17PV[P`\x01\x01a\x17\x8AV[P\x90V[[\x80\x82\x11\x15a\x17\xA7W`\0\x81U`\x01\x01a\x17\xACV[`\0` \x82\x84\x03\x12\x15a\x17\xD2W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x18\x02W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x18\x1BW`\0\x80\xFD[P5\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xCFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x18WW`\0\x80\xFD[\x825\x91P` \x83\x015a\x18i\x81a\x18\"V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x18\xEAWa\x18\xEAa\x18tV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x19\x03W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\x1DWa\x19\x1Da\x18tV[a\x19N` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x18\xA3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19cW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19\x92W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xA9W`\0\x80\xFD[a\x19\xB5\x84\x82\x85\x01a\x18\xF2V[\x94\x93PPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x19\xE3W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x19\xC7V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x81R\x83\x15\x15` \x82\x01R`\x80`@\x82\x01R`\0a\x1AB`\x80\x83\x01\x85a\x19\xBDV[\x90P\x82\x15\x15``\x83\x01R\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1A\xC8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra\x1A\xB6\x85\x83Qa\x19\xBDV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1A|V[P\x92\x97\x96PPPPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A\xEFWa\x1A\xEFa\x18tV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1B\nW`\0\x80\xFD[\x815` a\x1B\x1Fa\x1B\x1A\x83a\x1A\xD5V[a\x18\xA3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1B>W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1BpW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1BcW`\0\x80\x81\xFD[\x83R\x91\x83\x01\x91\x83\x01a\x1BBV[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1B\x91W`\0\x80\xFD[\x845\x93P` \x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xB1W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1B\xC5W`\0\x80\xFD[\x815a\x1B\xD3a\x1B\x1A\x82a\x1A\xD5V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a\x1B\xF2W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x1C\x19W\x845a\x1C\n\x81a\x18\"V[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x1B\xF7V[\x97PPP`@\x88\x015\x92P\x80\x83\x11\x15a\x1C1W`\0\x80\xFD[a\x1C=\x89\x84\x8A\x01a\x1A\xF9V[\x94P``\x88\x015\x92P\x80\x83\x11\x15a\x1CSW`\0\x80\xFD[PPa\x1Ca\x87\x82\x88\x01a\x18\xF2V[\x91PP\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x18\x02` \x83\x01\x84a\x19\xBDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\xDEWa\x05\xDEa\x1C\x80V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1D\"Wa\x1D\"a\x1C\x80V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D=W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1DvW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x08\xA9W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1D\xA3WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1D\xC2W\x82\x81U`\x01\x01a\x1D\xAFV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xE4Wa\x1D\xE4a\x18tV[a\x1D\xF8\x81a\x1D\xF2\x84Ta\x1D)V[\x84a\x1D|V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1EKW`\0\x84\x15a\x1E\x15WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1D\xC2V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x1E\x98W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1EyV[P\x85\x82\x10\x15a\x1E\xD4W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a\x1E\xF7``\x83\x01\x86a\x19\xBDV[\x90P\x83` \x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\xDEWa\x05\xDEa\x1C\x80V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\xDEWa\x05\xDEa\x1C\x80V[`\0\x82a\x1F\x83W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xDC\xD9\xC6\x1B!\xD9\xCDy\xCE\x14xaB\xB4Os\xF6MF\x17\xF3\x05*\xB3\xAA\xA0\xA5;Uf\0\x12dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static IMPACTEVALUATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xA5W`\x005`\xE0\x1C\x80c\x8EX\xCE$\x11a\0\xE1W\x80c\xA6\xBC\x94\xA0\x11a\0\x8AW\x80c\xDA\x84\\\xE0\x11a\0dW\x80c\xDA\x84\\\xE0\x14a\x04\xD5W\x80c\xE4\xC7T\x92\x14a\x04\xF5W\x80c\xF0\x93+\xD7\x14a\x05\x15W\x80c\xF1\x99\xF5m\x14a\x055W`\0\x80\xFD[\x80c\xA6\xBC\x94\xA0\x14a\x04\x80W\x80c\xC6H\x85\xEF\x14a\x04\xA0W\x80c\xD5Gt\x1F\x14a\x04\xB5W`\0\x80\xFD[\x80c\x94\x92\xC9|\x11a\0\xBBW\x80c\x94\x92\xC9|\x14a\x04\x1EW\x80c\x97:\x89\xB5\x14a\x04>W\x80c\xA2\x17\xFD\xDF\x14a\x04kW`\0\x80\xFD[\x80c\x8EX\xCE$\x14a\x03\x8AW\x80c\x91\xD1HT\x14a\x03\xB7W\x80c\x93\x84\xC8\x85\x14a\x04\x08W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01NW\x80ck\xF4\";\x11a\x01(W\x80ck\xF4\";\x14a\x02\xF0W\x80cxs\xCD\xE2\x14a\x03\x10W\x80c\x83\0-\xBA\x14a\x03DW\x80c\x8Ce\xC8\x1F\x14a\x03ZW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x02\x9BW\x80cU5\xDB\xF6\x14a\x02\xBBW\x80ch\x96\xEFK\x14a\x02\xDBW`\0\x80\xFD[\x80c\x19\xEE\xFA\xAB\x11a\x01\x7FW\x80c\x19\xEE\xFA\xAB\x14a\x02\x1DW\x80c$\x8A\x9C\xA3\x14a\x02=W\x80c//\xF1]\x14a\x02{W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xB1W\x80c\x05\x80\t\x93\x14a\x01\xE6W\x80c\x05\xF3xS\x14a\x02\x08W`\0\x80\xFD[6a\x01\xACW\0[`\0\x80\xFD[4\x80\x15a\x01\xBDW`\0\x80\xFD[Pa\x01\xD1a\x01\xCC6`\x04a\x17\xC0V[a\x05KV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF2W`\0\x80\xFD[Pa\x02\x06a\x02\x016`\x04a\x18\tV[a\x05\xE4V[\0[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x02\x06a\x07QV[4\x80\x15a\x02)W`\0\x80\xFD[Pa\x01\xD1a\x0286`\x04a\x18\tV[a\x07\xF3V[4\x80\x15a\x02IW`\0\x80\xFD[Pa\x02ma\x02X6`\x04a\x18\tV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\xDDV[4\x80\x15a\x02\x87W`\0\x80\xFD[Pa\x02\x06a\x02\x966`\x04a\x18DV[a\x08%V[4\x80\x15a\x02\xA7W`\0\x80\xFD[Pa\x02\x06a\x02\xB66`\x04a\x18DV[a\x08PV[4\x80\x15a\x02\xC7W`\0\x80\xFD[Pa\x02ma\x02\xD66`\x04a\x19\x80V[a\x08\xAEV[4\x80\x15a\x02\xE7W`\0\x80\xFD[Pa\x02ma\tDV[4\x80\x15a\x02\xFCW`\0\x80\xFD[Pa\x02ma\x03\x0B6`\x04a\x18\tV[a\tYV[4\x80\x15a\x03\x1CW`\0\x80\xFD[Pa\x02m\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x02m`\x04T\x81V[4\x80\x15a\x03fW`\0\x80\xFD[Pa\x03za\x03u6`\x04a\x18\tV[a\t\x87V[`@Qa\x01\xDD\x94\x93\x92\x91\x90a\x1A!V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x03\xAAa\x03\xA56`\x04a\x18\tV[a\nSV[`@Qa\x01\xDD\x91\x90a\x1AUV[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x01\xD1a\x03\xD26`\x04a\x18DV[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x04\x14W`\0\x80\xFD[Pa\x02m`\x02T\x81V[4\x80\x15a\x04*W`\0\x80\xFD[Pa\x02\x06a\x0496`\x04a\x1B{V[a\x0BPV[4\x80\x15a\x04JW`\0\x80\xFD[Pa\x04^a\x04Y6`\x04a\x18\tV[a\x0E\x84V[`@Qa\x01\xDD\x91\x90a\x1CmV[4\x80\x15a\x04wW`\0\x80\xFD[Pa\x02m`\0\x81V[4\x80\x15a\x04\x8CW`\0\x80\xFD[Pa\x02ma\x04\x9B6`\x04a\x18DV[a\x0F:V[4\x80\x15a\x04\xACW`\0\x80\xFD[Pa\x02ma\x0F\x97V[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x02\x06a\x04\xD06`\x04a\x18DV[a\x0F\xCDV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x02\x06a\x04\xF06`\x04a\x18\tV[a\x0F\xF2V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x01\xD1a\x05\x106`\x04a\x18\tV[a\x10\x8FV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\x06a\x0506`\x04a\x18\tV[a\x10\xC1V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02m`\x03T\x81V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x05\xDEWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x06\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x04T\x81\x10\x15a\x07LW`\0[`\x01Ta\x06\x9C\x90\x83\x90a\x1C\xAFV[\x81\x10\x15a\x07JW`\x01\x81\x81T\x81\x10a\x06\xB6Wa\x06\xB6a\x1C\xC2V[`\0\x91\x82R` \x82 `\x06\x90\x91\x02\x01\x81\x81U\x90a\x06\xD6`\x01\x83\x01\x82a\x172V[`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90Ua\x07\x0E`\x04\x83\x01`\0a\x17PV[P`\x05\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U\x80a\x07B\x81a\x1C\xF1V[\x91PPa\x06\x8EV[P[`\x04UV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x07\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[a\x07\xF1a\x11^V[V[`\0`\x01\x82\x81T\x81\x10a\x08\x08Wa\x08\x08a\x1C\xC2V[`\0\x91\x82R` \x90\x91 `\x05`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08@\x81a\x12\xD2V[a\x08J\x83\x83a\x12\xDCV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x08\x9FW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xA9\x82\x82a\x13\xD8V[PPPV[`\0a\x08\xB8a\x14\x93V[`\0a\x08\xC2a\tDV[\x90P`\x01\x81\x81T\x81\x10a\x08\xD7Wa\x08\xD7a\x1C\xC2V[`\0\x91\x82R` \x80\x83 `\x01`\x06\x90\x93\x02\x01\x82\x01\x80T\x92\x83\x01\x81U\x83R\x90\x91 \x01a\t\x02\x84\x82a\x1D\xCAV[P\x7F\xDBL\xFD\xC7\x95\xED\xA1d\x07\xC0\x8D\xAB&\xED\xC5\x83\xF4~-\x10\xE9\xE5l%\x0FE6\x12@\x93\xE8x\x83\x823`@Qa\t6\x93\x92\x91\x90a\x1E\xE4V[`@Q\x80\x91\x03\x90\xA1\x92\x91PPV[`\x01\x80T`\0\x91a\tT\x91a\x1C\xAFV[\x90P\x90V[`\0`\x01\x82\x81T\x81\x10a\tnWa\tna\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P\x91\x90PV[`\x01\x81\x81T\x81\x10a\t\x97W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x03\x82\x01T`\x04\x83\x01\x80T\x92\x94P`\xFF\x90\x91\x16\x92\x91a\t\xC7\x90a\x1D)V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF3\x90a\x1D)V[\x80\x15a\n@W\x80`\x1F\x10a\n\x15Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n@V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n#W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x05\x90\x93\x01T\x91\x92PP`\xFF\x16\x84V[```\x01\x82\x81T\x81\x10a\nhWa\nha\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0BEW\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\xB8\x90a\x1D)V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xE4\x90a\x1D)V[\x80\x15a\x0B1W\x80`\x1F\x10a\x0B\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\x99V[PPPP\x90P\x91\x90PV[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x0B\xE8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[\x81Q\x83Q\x14a\x0CxW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06xV[`\0`\x01\x85\x81T\x81\x10a\x0C\x8DWa\x0C\x8Da\x1C\xC2V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01`\x05\x81\x01T\x90\x91P`\xFF\x16a\r\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FRound does not exist\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\x03\x81\x01T`\xFF\x16\x15a\r}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FScores already submitted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\0[\x84Q\x81\x10\x15a\x0E6W\x83\x81\x81Q\x81\x10a\r\x9BWa\r\x9Ba\x1C\xC2V[` \x02` \x01\x01Q\x82`\x02\x01`\0\x87\x84\x81Q\x81\x10a\r\xBBWa\r\xBBa\x1C\xC2V[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80a\x0E.\x90a\x1C\xF1V[\x91PPa\r\x80V[P`\x04\x81\x01a\x0EE\x83\x82a\x1D\xCAV[P`\x03\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x0E}\x84\x84a\x14\xD2V[PPPPPV[```\x01\x82\x81T\x81\x10a\x0E\x99Wa\x0E\x99a\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x04\x01\x80Ta\x0E\xB5\x90a\x1D)V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\xE1\x90a\x1D)V[\x80\x15a\x0F.W\x80`\x1F\x10a\x0F\x03Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F.V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\x11W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0`\x01\x83\x81T\x81\x10a\x0FOWa\x0FOa\x1C\xC2V[`\0\x91\x82R` \x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R`\x02`\x06\x90\x93\x02\x01\x91\x90\x91\x01\x90R`@\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x92\x91PPV[`\0`\x01a\x0F\xA3a\tDV[\x81T\x81\x10a\x0F\xB3Wa\x0F\xB3a\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x01\x01\x80T\x90P\x90P\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xE8\x81a\x12\xD2V[a\x08J\x83\x83a\x13\xD8V[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x10\x8AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\x03UV[`\0`\x01\x82\x81T\x81\x10a\x10\xA4Wa\x10\xA4a\x1C\xC2V[`\0\x91\x82R` \x90\x91 `\x03`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x11YW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\x02UV[`\x01\x80T\x81\x01\x80\x82U`\0\x82\x81R\x91\x90a\x11y\x90\x82\x90a\x1C\xAFV[\x81T\x81\x10a\x11\x89Wa\x11\x89a\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01\x90P`\x02TCa\x11\xA7\x91\x90a\x1F#V[\x81U`\x05\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sa\x11\xFFa\tDV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x04T`\x01T\x11\x15a\x12\xCFW`\x01\x80`\x04T`\x01\x80T\x90Pa\x122\x91\x90a\x1C\xAFV[a\x12<\x91\x90a\x1C\xAFV[\x81T\x81\x10a\x12LWa\x12La\x1C\xC2V[`\0\x91\x82R` \x82 `\x06\x90\x91\x02\x01\x81\x81U\x90a\x12l`\x01\x83\x01\x82a\x172V[`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90Ua\x12\xA4`\x04\x83\x01`\0a\x17PV[P`\x05\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U[PV[a\x12\xCF\x813a\x16\xA8V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x13\xD0W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x13n3\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x05\xDEV[P`\0a\x05\xDEV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x13\xD0W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x05\xDEV[`\0`\x01a\x14\x9Fa\tDV[\x81T\x81\x10a\x14\xAFWa\x14\xAFa\x1C\xC2V[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P\x80C\x10a\x12\xCFWa\x12\xCFa\x11^V[`\x03TG\x10\x15a\x15>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot enough funds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06xV[`\0[\x82Q\x81\x10\x15a\x08\xA9W`\0\x83\x82\x81Q\x81\x10a\x15^Wa\x15^a\x1C\xC2V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x15|Wa\x15|a\x1C\xC2V[` \x02` \x01\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0f\x03\x8D~\xA4\xC6\x80\0`\x03T\x83a\x15\xA8\x91\x90a\x1F6V[a\x15\xB2\x91\x90a\x1FMV[`@Q\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15a\x16AW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fi\xCA\x02\xDDN\xDD{\xF0\xA4\xAB\xB9\xED;z\xF3\xF1Gx\xDB]a\x92\x1C}\xC7\xCDTRf2m\xE2\x82`@Qa\x164\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x16\x92V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1CC\xB9v\x1B?\xBAS!\xCA\x82\x12\xBF\xC21\x94_f\x8C\xCC\x0CDo39\x99\xEE\xA9\xCE\x8F\xDA\x81\x82`@Qa\x16\x89\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PPP\x80\x80a\x16\xA0\x90a\x1C\xF1V[\x91PPa\x15AV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x17.W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x06xV[PPV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x12\xCF\x91\x90a\x17\x8AV[P\x80Ta\x17\\\x90a\x1D)V[`\0\x82U\x80`\x1F\x10a\x17lWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x12\xCF\x91\x90a\x17\xABV[\x80\x82\x11\x15a\x17\xA7W`\0a\x17\x9E\x82\x82a\x17PV[P`\x01\x01a\x17\x8AV[P\x90V[[\x80\x82\x11\x15a\x17\xA7W`\0\x81U`\x01\x01a\x17\xACV[`\0` \x82\x84\x03\x12\x15a\x17\xD2W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x18\x02W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x18\x1BW`\0\x80\xFD[P5\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xCFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x18WW`\0\x80\xFD[\x825\x91P` \x83\x015a\x18i\x81a\x18\"V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x18\xEAWa\x18\xEAa\x18tV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x19\x03W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\x1DWa\x19\x1Da\x18tV[a\x19N` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x18\xA3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19cW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19\x92W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xA9W`\0\x80\xFD[a\x19\xB5\x84\x82\x85\x01a\x18\xF2V[\x94\x93PPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x19\xE3W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x19\xC7V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x81R\x83\x15\x15` \x82\x01R`\x80`@\x82\x01R`\0a\x1AB`\x80\x83\x01\x85a\x19\xBDV[\x90P\x82\x15\x15``\x83\x01R\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1A\xC8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra\x1A\xB6\x85\x83Qa\x19\xBDV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1A|V[P\x92\x97\x96PPPPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A\xEFWa\x1A\xEFa\x18tV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1B\nW`\0\x80\xFD[\x815` a\x1B\x1Fa\x1B\x1A\x83a\x1A\xD5V[a\x18\xA3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1B>W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1BpW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1BcW`\0\x80\x81\xFD[\x83R\x91\x83\x01\x91\x83\x01a\x1BBV[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1B\x91W`\0\x80\xFD[\x845\x93P` \x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xB1W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1B\xC5W`\0\x80\xFD[\x815a\x1B\xD3a\x1B\x1A\x82a\x1A\xD5V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a\x1B\xF2W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x1C\x19W\x845a\x1C\n\x81a\x18\"V[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x1B\xF7V[\x97PPP`@\x88\x015\x92P\x80\x83\x11\x15a\x1C1W`\0\x80\xFD[a\x1C=\x89\x84\x8A\x01a\x1A\xF9V[\x94P``\x88\x015\x92P\x80\x83\x11\x15a\x1CSW`\0\x80\xFD[PPa\x1Ca\x87\x82\x88\x01a\x18\xF2V[\x91PP\x92\x95\x91\x94P\x92PV[` \x81R`\0a\x18\x02` \x83\x01\x84a\x19\xBDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\xDEWa\x05\xDEa\x1C\x80V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1D\"Wa\x1D\"a\x1C\x80V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D=W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1DvW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x08\xA9W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1D\xA3WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1D\xC2W\x82\x81U`\x01\x01a\x1D\xAFV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xE4Wa\x1D\xE4a\x18tV[a\x1D\xF8\x81a\x1D\xF2\x84Ta\x1D)V[\x84a\x1D|V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1EKW`\0\x84\x15a\x1E\x15WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1D\xC2V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x1E\x98W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1EyV[P\x85\x82\x10\x15a\x1E\xD4W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a\x1E\xF7``\x83\x01\x86a\x19\xBDV[\x90P\x83` \x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\xDEWa\x05\xDEa\x1C\x80V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\xDEWa\x05\xDEa\x1C\x80V[`\0\x82a\x1F\x83W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xDC\xD9\xC6\x1B!\xD9\xCDy\xCE\x14xaB\xB4Os\xF6MF\x17\xF3\x05*\xB3\xAA\xA0\xA5;Uf\0\x12dsolcC\0\x08\x15\x003";
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
        ///Calls the contract's `currentRoundIndex` (0x6896ef4b) function
        pub fn current_round_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([104, 150, 239, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentRoundMeasurementCount` (0xc64885ef) function
        pub fn current_round_measurement_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 72, 133, 239], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParticipantScore` (0xa6bc94a0) function
        pub fn get_participant_score(
            &self,
            round_index: ::ethers::core::types::U256,
            participant: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 188, 148, 160], (round_index, participant))
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
        ///Calls the contract's `getRoundEnd` (0x6bf4223b) function
        pub fn get_round_end(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 244, 34, 59], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundExists` (0x19eefaab) function
        pub fn get_round_exists(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([25, 238, 250, 171], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundMeasurementsCids` (0x8e58ce24) function
        pub fn get_round_measurements_cids(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([142, 88, 206, 36], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundScoresSubmitted` (0xe4c75492) function
        pub fn get_round_scores_submitted(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 199, 84, 146], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundSummaryText` (0x973a89b5) function
        pub fn get_round_summary_text(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([151, 58, 137, 181], index)
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
        ///Calls the contract's `maxStoredRounds` (0x83002dba) function
        pub fn max_stored_rounds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 0, 45, 186], ())
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
        ///Calls the contract's `rounds` (0x8c65c81f) function
        pub fn rounds(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool, ::std::string::String, bool),
        > {
            self.0
                .method_hash([140, 101, 200, 31], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxStoredRounds` (0x05800993) function
        pub fn set_max_stored_rounds(
            &self,
            max_stored_rounds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 128, 9, 147], max_stored_rounds)
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
        ///Calls the contract's `setScores` (0x9492c97c) function
        pub fn set_scores(
            &self,
            round_index: ::ethers::core::types::U256,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            scores: ::std::vec::Vec<u64>,
            summary_text: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [148, 146, 201, 124],
                    (round_index, addresses, scores, summary_text),
                )
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
        pub round_index: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `currentRoundMeasurementCount` function with signature `currentRoundMeasurementCount()` and selector `0xc64885ef`
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
        name = "currentRoundMeasurementCount",
        abi = "currentRoundMeasurementCount()"
    )]
    pub struct CurrentRoundMeasurementCountCall;
    ///Container type for all input parameters for the `getParticipantScore` function with signature `getParticipantScore(uint256,address)` and selector `0xa6bc94a0`
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
        name = "getParticipantScore",
        abi = "getParticipantScore(uint256,address)"
    )]
    pub struct GetParticipantScoreCall {
        pub round_index: ::ethers::core::types::U256,
        pub participant: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `getRoundEnd` function with signature `getRoundEnd(uint256)` and selector `0x6bf4223b`
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
    #[ethcall(name = "getRoundEnd", abi = "getRoundEnd(uint256)")]
    pub struct GetRoundEndCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoundExists` function with signature `getRoundExists(uint256)` and selector `0x19eefaab`
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
    #[ethcall(name = "getRoundExists", abi = "getRoundExists(uint256)")]
    pub struct GetRoundExistsCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoundMeasurementsCids` function with signature `getRoundMeasurementsCids(uint256)` and selector `0x8e58ce24`
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
        name = "getRoundMeasurementsCids",
        abi = "getRoundMeasurementsCids(uint256)"
    )]
    pub struct GetRoundMeasurementsCidsCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoundScoresSubmitted` function with signature `getRoundScoresSubmitted(uint256)` and selector `0xe4c75492`
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
        name = "getRoundScoresSubmitted",
        abi = "getRoundScoresSubmitted(uint256)"
    )]
    pub struct GetRoundScoresSubmittedCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoundSummaryText` function with signature `getRoundSummaryText(uint256)` and selector `0x973a89b5`
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
    #[ethcall(name = "getRoundSummaryText", abi = "getRoundSummaryText(uint256)")]
    pub struct GetRoundSummaryTextCall {
        pub index: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `maxStoredRounds` function with signature `maxStoredRounds()` and selector `0x83002dba`
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
    #[ethcall(name = "maxStoredRounds", abi = "maxStoredRounds()")]
    pub struct MaxStoredRoundsCall;
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
    ///Container type for all input parameters for the `rounds` function with signature `rounds(uint256)` and selector `0x8c65c81f`
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
    #[ethcall(name = "rounds", abi = "rounds(uint256)")]
    pub struct RoundsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `setMaxStoredRounds` function with signature `setMaxStoredRounds(uint256)` and selector `0x05800993`
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
    #[ethcall(name = "setMaxStoredRounds", abi = "setMaxStoredRounds(uint256)")]
    pub struct SetMaxStoredRoundsCall {
        pub max_stored_rounds: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setScores` function with signature `setScores(uint256,address[],uint64[],string)` and selector `0x9492c97c`
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
    #[ethcall(name = "setScores", abi = "setScores(uint256,address[],uint64[],string)")]
    pub struct SetScoresCall {
        pub round_index: ::ethers::core::types::U256,
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub scores: ::std::vec::Vec<u64>,
        pub summary_text: ::std::string::String,
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ImpactEvaluatorCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EvaluateRole(EvaluateRoleCall),
        AddMeasurements(AddMeasurementsCall),
        AdminAdvanceRound(AdminAdvanceRoundCall),
        CurrentRoundIndex(CurrentRoundIndexCall),
        CurrentRoundMeasurementCount(CurrentRoundMeasurementCountCall),
        GetParticipantScore(GetParticipantScoreCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoundEnd(GetRoundEndCall),
        GetRoundExists(GetRoundExistsCall),
        GetRoundMeasurementsCids(GetRoundMeasurementsCidsCall),
        GetRoundScoresSubmitted(GetRoundScoresSubmittedCall),
        GetRoundSummaryText(GetRoundSummaryTextCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        MaxStoredRounds(MaxStoredRoundsCall),
        NextRoundLength(NextRoundLengthCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RoundReward(RoundRewardCall),
        Rounds(RoundsCall),
        SetMaxStoredRounds(SetMaxStoredRoundsCall),
        SetNextRoundLength(SetNextRoundLengthCall),
        SetRoundReward(SetRoundRewardCall),
        SetScores(SetScoresCall),
        SupportsInterface(SupportsInterfaceCall),
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
                = <CurrentRoundIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentRoundIndex(decoded));
            }
            if let Ok(decoded)
                = <CurrentRoundMeasurementCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentRoundMeasurementCount(decoded));
            }
            if let Ok(decoded)
                = <GetParticipantScoreCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetParticipantScore(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GetRoundEndCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoundEnd(decoded));
            }
            if let Ok(decoded)
                = <GetRoundExistsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoundExists(decoded));
            }
            if let Ok(decoded)
                = <GetRoundMeasurementsCidsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRoundMeasurementsCids(decoded));
            }
            if let Ok(decoded)
                = <GetRoundScoresSubmittedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRoundScoresSubmitted(decoded));
            }
            if let Ok(decoded)
                = <GetRoundSummaryTextCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRoundSummaryText(decoded));
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
                = <MaxStoredRoundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxStoredRounds(decoded));
            }
            if let Ok(decoded)
                = <NextRoundLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NextRoundLength(decoded));
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
                = <RoundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rounds(decoded));
            }
            if let Ok(decoded)
                = <SetMaxStoredRoundsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMaxStoredRounds(decoded));
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
                Self::AddMeasurements(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminAdvanceRound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentRoundIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentRoundMeasurementCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetParticipantScore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundEnd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundMeasurementsCids(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundScoresSubmitted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundSummaryText(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxStoredRounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextRoundLength(element) => {
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
                Self::Rounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMaxStoredRounds(element) => {
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
            }
        }
    }
    impl ::core::fmt::Display for ImpactEvaluatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::EvaluateRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddMeasurements(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminAdvanceRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentRoundIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentRoundMeasurementCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetParticipantScore(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundMeasurementsCids(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoundScoresSubmitted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoundSummaryText(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxStoredRounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRoundLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaxStoredRounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetNextRoundLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRoundReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetScores(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<CurrentRoundIndexCall> for ImpactEvaluatorCalls {
        fn from(value: CurrentRoundIndexCall) -> Self {
            Self::CurrentRoundIndex(value)
        }
    }
    impl ::core::convert::From<CurrentRoundMeasurementCountCall>
    for ImpactEvaluatorCalls {
        fn from(value: CurrentRoundMeasurementCountCall) -> Self {
            Self::CurrentRoundMeasurementCount(value)
        }
    }
    impl ::core::convert::From<GetParticipantScoreCall> for ImpactEvaluatorCalls {
        fn from(value: GetParticipantScoreCall) -> Self {
            Self::GetParticipantScore(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoundEndCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoundEndCall) -> Self {
            Self::GetRoundEnd(value)
        }
    }
    impl ::core::convert::From<GetRoundExistsCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoundExistsCall) -> Self {
            Self::GetRoundExists(value)
        }
    }
    impl ::core::convert::From<GetRoundMeasurementsCidsCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoundMeasurementsCidsCall) -> Self {
            Self::GetRoundMeasurementsCids(value)
        }
    }
    impl ::core::convert::From<GetRoundScoresSubmittedCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoundScoresSubmittedCall) -> Self {
            Self::GetRoundScoresSubmitted(value)
        }
    }
    impl ::core::convert::From<GetRoundSummaryTextCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoundSummaryTextCall) -> Self {
            Self::GetRoundSummaryText(value)
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
    impl ::core::convert::From<MaxStoredRoundsCall> for ImpactEvaluatorCalls {
        fn from(value: MaxStoredRoundsCall) -> Self {
            Self::MaxStoredRounds(value)
        }
    }
    impl ::core::convert::From<NextRoundLengthCall> for ImpactEvaluatorCalls {
        fn from(value: NextRoundLengthCall) -> Self {
            Self::NextRoundLength(value)
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
    impl ::core::convert::From<RoundsCall> for ImpactEvaluatorCalls {
        fn from(value: RoundsCall) -> Self {
            Self::Rounds(value)
        }
    }
    impl ::core::convert::From<SetMaxStoredRoundsCall> for ImpactEvaluatorCalls {
        fn from(value: SetMaxStoredRoundsCall) -> Self {
            Self::SetMaxStoredRounds(value)
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
    ///Container type for all return fields from the `currentRoundMeasurementCount` function with signature `currentRoundMeasurementCount()` and selector `0xc64885ef`
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
    pub struct CurrentRoundMeasurementCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getParticipantScore` function with signature `getParticipantScore(uint256,address)` and selector `0xa6bc94a0`
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
    pub struct GetParticipantScoreReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `getRoundEnd` function with signature `getRoundEnd(uint256)` and selector `0x6bf4223b`
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
    pub struct GetRoundEndReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRoundExists` function with signature `getRoundExists(uint256)` and selector `0x19eefaab`
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
    pub struct GetRoundExistsReturn(pub bool);
    ///Container type for all return fields from the `getRoundMeasurementsCids` function with signature `getRoundMeasurementsCids(uint256)` and selector `0x8e58ce24`
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
    pub struct GetRoundMeasurementsCidsReturn(
        pub ::std::vec::Vec<::std::string::String>,
    );
    ///Container type for all return fields from the `getRoundScoresSubmitted` function with signature `getRoundScoresSubmitted(uint256)` and selector `0xe4c75492`
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
    pub struct GetRoundScoresSubmittedReturn(pub bool);
    ///Container type for all return fields from the `getRoundSummaryText` function with signature `getRoundSummaryText(uint256)` and selector `0x973a89b5`
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
    pub struct GetRoundSummaryTextReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `maxStoredRounds` function with signature `maxStoredRounds()` and selector `0x83002dba`
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
    pub struct MaxStoredRoundsReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `rounds` function with signature `rounds(uint256)` and selector `0x8c65c81f`
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
    pub struct RoundsReturn {
        pub end: ::ethers::core::types::U256,
        pub scores_submitted: bool,
        pub summary_text: ::std::string::String,
        pub exists: bool,
    }
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
