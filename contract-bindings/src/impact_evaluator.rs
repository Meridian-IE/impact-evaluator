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
                    ::std::borrow::ToOwned::to_owned("getRound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRound"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ImpactEvaluator.Round",
                                        ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R`\n`\x02U`d`\x03U4\x80\x15b\0\0\x1BW`\0\x80\xFD[P`@Qb\0&\xD18\x03\x80b\0&\xD1\x839\x81\x01`@\x81\x90Rb\0\0>\x91b\0\x04DV[b\0\0K`\0\x82b\0\0\x8AV[Pb\0\0x\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x82b\0\0\x8AV[Pb\0\0\x83b\0\x019V[Pb\0\x06)V[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16b\0\x01/W`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\xE63\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01b\0\x013V[P`\0[\x92\x91PPV[b\0\x01u`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[`\x02Tb\0\x01\x84\x90Cb\0\x04\x8CV[\x81R`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x81Q`\x06\x90\x91\x02\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x81\x01\x91\x82U` \x80\x84\x01Q\x80Q\x85\x94\x93b\0\x01\xFF\x93\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x90\x91\x01\x92\x01\x90b\0\x02\xC9V[P`@\x82\x01Q\x80Qb\0\x02\x1D\x91`\x02\x84\x01\x91` \x90\x91\x01\x90b\0\x03&V[P``\x82\x01Q\x80Qb\0\x02;\x91`\x03\x84\x01\x91` \x90\x91\x01\x90b\0\x03\x8CV[P`\x80\x82\x01Q`\x04\x82\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90b\0\x02j\x90\x82b\0\x05GV[PPP\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sb\0\x02\x9Eb\0\x02\xB2` \x1B` \x1CV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01\x80T`\0\x91b\0\x02\xC4\x91b\0\x06\x13V[\x90P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x03\x14W\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x03\x14W\x82Q\x82\x90b\0\x03\x03\x90\x82b\0\x05GV[P\x91` \x01\x91\x90`\x01\x01\x90b\0\x02\xEAV[Pb\0\x03\"\x92\x91Pb\0\x03\xCAV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x03~W\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x03~W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\x03GV[Pb\0\x03\"\x92\x91Pb\0\x03\xEBV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x03~W\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x03~W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x03\xADV[\x80\x82\x11\x15b\0\x03\"W`\0b\0\x03\xE1\x82\x82b\0\x04\x02V[P`\x01\x01b\0\x03\xCAV[[\x80\x82\x11\x15b\0\x03\"W`\0\x81U`\x01\x01b\0\x03\xECV[P\x80Tb\0\x04\x10\x90b\0\x04\xB8V[`\0\x82U\x80`\x1F\x10b\0\x04!WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0\x04A\x91\x90b\0\x03\xEBV[PV[`\0` \x82\x84\x03\x12\x15b\0\x04WW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x04oW`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15b\0\x013Wb\0\x013b\0\x04vV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x04\xCDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x04\xEEWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x05BW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x05\x1DWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x05>W\x82\x81U`\x01\x01b\0\x05)V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x05cWb\0\x05cb\0\x04\xA2V[b\0\x05{\x81b\0\x05t\x84Tb\0\x04\xB8V[\x84b\0\x04\xF4V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x05\xB3W`\0\x84\x15b\0\x05\x9AWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x05>V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x05\xE4W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x05\xC3V[P\x85\x82\x10\x15b\0\x06\x03W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15b\0\x013Wb\0\x013b\0\x04vV[a \x98\x80b\0\x069`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01HW`\x005`\xE0\x1C\x80c\x8Ce\xC8\x1F\x11a\0\xC0W\x80c\xC6H\x85\xEF\x11a\0tW\x80c\xDA\x84\\\xE0\x11a\0YW\x80c\xDA\x84\\\xE0\x14a\x03\xB4W\x80c\xF0\x93+\xD7\x14a\x03\xD4W\x80c\xF1\x99\xF5m\x14a\x03\xF4W`\0\x80\xFD[\x80c\xC6H\x85\xEF\x14a\x03\x7FW\x80c\xD5Gt\x1F\x14a\x03\x94W`\0\x80\xFD[\x80c\x91\xD1HT\x11a\0\xA5W\x80c\x91\xD1HT\x14a\x03\x03W\x80c\x93\x84\xC8\x85\x14a\x03TW\x80c\xA2\x17\xFD\xDF\x14a\x03jW`\0\x80\xFD[\x80c\x8Ce\xC8\x1F\x14a\x02\xA7W\x80c\x8F\x13'\xC0\x14a\x02\xD6W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01\x17W\x80ch\x96\xEFK\x11a\0\xFCW\x80ch\x96\xEFK\x14a\x02>W\x80cv\x99\xEA\x11\x14a\x02SW\x80cxs\xCD\xE2\x14a\x02sW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x01\xFEW\x80cU5\xDB\xF6\x14a\x02\x1EW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01TW\x80c\x05\xF3xS\x14a\x01\x89W\x80c$\x8A\x9C\xA3\x14a\x01\xA0W\x80c//\xF1]\x14a\x01\xDEW`\0\x80\xFD[6a\x01OW\0[`\0\x80\xFD[4\x80\x15a\x01`W`\0\x80\xFD[Pa\x01ta\x01o6`\x04a\x17\xD4V[a\x04\nV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x95W`\0\x80\xFD[Pa\x01\x9Ea\x04\xA3V[\0[4\x80\x15a\x01\xACW`\0\x80\xFD[Pa\x01\xD0a\x01\xBB6`\x04a\x18\x1DV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x80V[4\x80\x15a\x01\xEAW`\0\x80\xFD[Pa\x01\x9Ea\x01\xF96`\x04a\x18XV[a\x05JV[4\x80\x15a\x02\nW`\0\x80\xFD[Pa\x01\x9Ea\x02\x196`\x04a\x18XV[a\x05uV[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x01\xD0a\x0296`\x04a\x19\x94V[a\x05\xD3V[4\x80\x15a\x02JW`\0\x80\xFD[Pa\x01\xD0a\x06`V[4\x80\x15a\x02_W`\0\x80\xFD[Pa\x01\x9Ea\x02n6`\x04a\x1A`V[a\x06uV[4\x80\x15a\x02\x7FW`\0\x80\xFD[Pa\x01\xD0\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x02\xB3W`\0\x80\xFD[Pa\x02\xC7a\x02\xC26`\x04a\x18\x1DV[a\x0B\xFFV[`@Qa\x01\x80\x93\x92\x91\x90a\x1B\xB6V[4\x80\x15a\x02\xE2W`\0\x80\xFD[Pa\x02\xF6a\x02\xF16`\x04a\x18\x1DV[a\x0C\xC2V[`@Qa\x01\x80\x91\x90a\x1CaV[4\x80\x15a\x03\x0FW`\0\x80\xFD[Pa\x01ta\x03\x1E6`\x04a\x18XV[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x01\xD0`\x02T\x81V[4\x80\x15a\x03vW`\0\x80\xFD[Pa\x01\xD0`\0\x81V[4\x80\x15a\x03\x8BW`\0\x80\xFD[Pa\x01\xD0a\x0F\x7FV[4\x80\x15a\x03\xA0W`\0\x80\xFD[Pa\x01\x9Ea\x03\xAF6`\x04a\x18XV[a\x0F\xB5V[4\x80\x15a\x03\xC0W`\0\x80\xFD[Pa\x01\x9Ea\x03\xCF6`\x04a\x18\x1DV[a\x0F\xDAV[4\x80\x15a\x03\xE0W`\0\x80\xFD[Pa\x01\x9Ea\x03\xEF6`\x04a\x18\x1DV[a\x10wV[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x01\xD0`\x03T\x81V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x04\x9DWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x05@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x05Ha\x11\x14V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05e\x81a\x12\x98V[a\x05o\x83\x83a\x12\xA5V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x05\xC4W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xCE\x82\x82a\x13\xA1V[PPPV[`\0\x80a\x05\xDEa\x06`V[\x90P`\x01\x81\x81T\x81\x10a\x05\xF3Wa\x05\xF3a\x1DwV[`\0\x91\x82R` \x80\x83 `\x01`\x06\x90\x93\x02\x01\x82\x01\x80T\x92\x83\x01\x81U\x83R\x90\x91 \x01a\x06\x1E\x84\x82a\x1EGV[P\x7F\x18J\xF4\xCB\x002d\xF0\xF2:It/\xB8\x14)\xDD8\xBA\x83I*\xDB\xC7\x82\tGpQ\xB8+\xCB\x83\x82`@Qa\x06P\x92\x91\x90a\x1FaV[`@Q\x80\x91\x03\x90\xA1a\x04\x9Da\x14\\V[`\x01\x80T`\0\x91a\x06p\x91a\x1F\xB2V[\x90P\x90V[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x07\rW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\x01Ta\x07\x1C\x90`\x02\x90a\x1F\xB2V[\x84\x11\x15a\x07\x85W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FWrong round\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[\x81Q\x83Q\x14a\x08\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x057V[`\0`\x01\x85\x81T\x81\x10a\x08*Wa\x08*a\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\x1EW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x08\x91\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xBD\x90a\x1D\xA6V[\x80\x15a\t\nW\x80`\x1F\x10a\x08\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08rV[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x8CW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\taW[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\xE4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t\xD0W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\n\x10\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n<\x90a\x1D\xA6V[\x80\x15a\n\x89W\x80`\x1F\x10a\n^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nlW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`\x80\x01Q\x15a\x0B\0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FScores already submitted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\x01`\x80\x82\x01\x81\x90R\x80T\x82\x91\x90\x87\x90\x81\x10a\x0B3Wa\x0B3a\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x0Bg\x92\x91\x90a\x16QV[P`@\x82\x01Q\x80Qa\x0B\x83\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x16\xA7V[P``\x82\x01Q\x80Qa\x0B\x9F\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x17-V[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\x0B\xEA\x90\x82a\x1EGV[P\x90PPa\x0B\xF8\x84\x84a\x14\x9BV[PPPPPV[`\x01\x81\x81T\x81\x10a\x0C\x0FW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x04\x82\x01T`\x05\x83\x01\x80T\x92\x94P`\xFF\x90\x91\x16\x92\x91a\x0C?\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Ck\x90a\x1D\xA6V[\x80\x15a\x0C\xB8W\x80`\x1F\x10a\x0C\x8DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xB8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x9BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[a\x0C\xFD`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[`\x01\x82\x81T\x81\x10a\r\x10Wa\r\x10a\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0E\x04W\x83\x82\x90`\0R` `\0 \x01\x80Ta\rw\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xA3\x90a\x1D\xA6V[\x80\x15a\r\xF0W\x80`\x1F\x10a\r\xC5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xF0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xD3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\rXV[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0ErW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0EGW[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xCAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\xB6W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\x0E\xF6\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\"\x90a\x1D\xA6V[\x80\x15a\x0FoW\x80`\x1F\x10a\x0FDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FoV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FRW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0`\x01a\x0F\x8Ba\x06`V[\x81T\x81\x10a\x0F\x9BWa\x0F\x9Ba\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`\x01\x01\x80T\x90P\x90P\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xD0\x81a\x12\x98V[a\x05o\x83\x83a\x13\xA1V[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x10rW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\x03UV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x11\x0FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\x02UV[a\x11O`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[`\x02Ta\x11\\\x90Ca\x1F\xC5V[\x81R`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x81Q`\x06\x90\x91\x02\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x81\x01\x91\x82U` \x80\x84\x01Q\x80Q\x85\x94\x93a\x11\xD5\x93\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x90\x91\x01\x92\x01\x90a\x16QV[P`@\x82\x01Q\x80Qa\x11\xF1\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x16\xA7V[P``\x82\x01Q\x80Qa\x12\r\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x17-V[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\x12X\x90\x82a\x1EGV[PPP\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sa\x12\x84a\x06`V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x12\xA2\x813a\x15\xC7V[PV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x13\x99W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x1373\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x04\x9DV[P`\0a\x04\x9DV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x13\x99W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x04\x9DV[`\0`\x01a\x14ha\x06`V[\x81T\x81\x10a\x14xWa\x14xa\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P\x80C\x10a\x12\xA2Wa\x12\xA2a\x11\x14V[`\x03TG\x10\x15a\x15\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot enough funds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\0[\x82Q\x81\x10\x15a\x05\xCEW`\0\x83\x82\x81Q\x81\x10a\x15'Wa\x15'a\x1DwV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x15EWa\x15Ea\x1DwV[` \x02` \x01\x01Q\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xFC`\x03Tf\x03\x8D~\xA4\xC6\x80\0\x84a\x15\x7F\x91\x90a\x1F\xD8V[a\x15\x89\x91\x90a \x13V[`@Q\x81\x15\x90\x92\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x15\xB1W=`\0\x80>=`\0\xFD[PPP\x80\x80a\x15\xBF\x90a *V[\x91PPa\x15\nV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x16MW`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x057V[PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x16\x97W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x16\x97W\x82Q\x82\x90a\x16\x87\x90\x82a\x1EGV[P\x91` \x01\x91\x90`\x01\x01\x90a\x16qV[Pa\x16\xA3\x92\x91Pa\x17hV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x17!W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x17!W\x82Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x16\xC7V[Pa\x16\xA3\x92\x91Pa\x17\x85V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x17!W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x17!W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x17MV[\x80\x82\x11\x15a\x16\xA3W`\0a\x17|\x82\x82a\x17\x9AV[P`\x01\x01a\x17hV[[\x80\x82\x11\x15a\x16\xA3W`\0\x81U`\x01\x01a\x17\x86V[P\x80Ta\x17\xA6\x90a\x1D\xA6V[`\0\x82U\x80`\x1F\x10a\x17\xB6WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x12\xA2\x91\x90a\x17\x85V[`\0` \x82\x84\x03\x12\x15a\x17\xE6W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x18\x16W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x18/W`\0\x80\xFD[P5\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xA2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x18kW`\0\x80\xFD[\x825\x91P` \x83\x015a\x18}\x81a\x186V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x18\xFEWa\x18\xFEa\x18\x88V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x19\x17W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x191Wa\x191a\x18\x88V[a\x19b` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x18\xB7V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19wW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19\xA6W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xBDW`\0\x80\xFD[a\x19\xC9\x84\x82\x85\x01a\x19\x06V[\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\xEBWa\x19\xEBa\x18\x88V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1A\x06W`\0\x80\xFD[\x815` a\x1A\x1Ba\x1A\x16\x83a\x19\xD1V[a\x18\xB7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1A:W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1AUW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1A>V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1AvW`\0\x80\xFD[\x845\x93P` \x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\x96W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1A\xAAW`\0\x80\xFD[\x815a\x1A\xB8a\x1A\x16\x82a\x19\xD1V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a\x1A\xD7W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x1A\xFEW\x845a\x1A\xEF\x81a\x186V[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x1A\xDCV[\x97PPP`@\x88\x015\x92P\x80\x83\x11\x15a\x1B\x16W`\0\x80\xFD[a\x1B\"\x89\x84\x8A\x01a\x19\xF5V[\x94P``\x88\x015\x92P\x80\x83\x11\x15a\x1B8W`\0\x80\xFD[PPa\x1BF\x87\x82\x88\x01a\x19\x06V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1BxW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1B\\V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a\x1B\xD7``\x83\x01\x84a\x1BRV[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x1C&W\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x1B\xF4V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x1C&W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x1CEV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`\xC0`@\x84\x01R\x80Q`\xE0\x84\x01\x81\x90R`\0\x92\x91a\x01\0`\x05\x83\x90\x1B\x86\x01\x81\x01\x92\x91\x84\x01\x91\x90\x86\x01\x90\x85[\x81\x81\x10\x15a\x1C\xE7W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x88\x86\x03\x01\x83Ra\x1C\xD5\x85\x85Qa\x1BRV[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01a\x1C\x9BV[PPPP`@\x85\x01Q\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x83\x03\x01``\x86\x01Ra\x1D'\x82\x84a\x1B\xE0V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01Ra\x1DD\x83\x83a\x1C1V[\x92P`\x80\x86\x01Q\x91Pa\x1D[`\xA0\x86\x01\x83\x15\x15\x90RV[`\xA0\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x1B\xD7\x82\x82a\x1BRV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D\xBAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1D\xF3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\xCEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1E WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1E?W\x82\x81U`\x01\x01a\x1E,V[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EaWa\x1Eaa\x18\x88V[a\x1Eu\x81a\x1Eo\x84Ta\x1D\xA6V[\x84a\x1D\xF9V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1E\xC8W`\0\x84\x15a\x1E\x92WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1E?V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x1F\x15W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1E\xF6V[P\x85\x82\x10\x15a\x1FQW\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R`\0a\x1Ft`@\x83\x01\x85a\x1BRV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x9DWa\x04\x9Da\x1F\x83V[\x80\x82\x01\x80\x82\x11\x15a\x04\x9DWa\x04\x9Da\x1F\x83V[`\0\x82a \x0EW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x9DWa\x04\x9Da\x1F\x83V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a [Wa [a\x1F\x83V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x07\xB9\xAD-\xEE+\xA7Z\xCDp\x9EV\x04/X\xC0_\x02\x063\xC6\x95\xD9\x96g\xBD\xAC\xB6-9H4dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static IMPACTEVALUATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01HW`\x005`\xE0\x1C\x80c\x8Ce\xC8\x1F\x11a\0\xC0W\x80c\xC6H\x85\xEF\x11a\0tW\x80c\xDA\x84\\\xE0\x11a\0YW\x80c\xDA\x84\\\xE0\x14a\x03\xB4W\x80c\xF0\x93+\xD7\x14a\x03\xD4W\x80c\xF1\x99\xF5m\x14a\x03\xF4W`\0\x80\xFD[\x80c\xC6H\x85\xEF\x14a\x03\x7FW\x80c\xD5Gt\x1F\x14a\x03\x94W`\0\x80\xFD[\x80c\x91\xD1HT\x11a\0\xA5W\x80c\x91\xD1HT\x14a\x03\x03W\x80c\x93\x84\xC8\x85\x14a\x03TW\x80c\xA2\x17\xFD\xDF\x14a\x03jW`\0\x80\xFD[\x80c\x8Ce\xC8\x1F\x14a\x02\xA7W\x80c\x8F\x13'\xC0\x14a\x02\xD6W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01\x17W\x80ch\x96\xEFK\x11a\0\xFCW\x80ch\x96\xEFK\x14a\x02>W\x80cv\x99\xEA\x11\x14a\x02SW\x80cxs\xCD\xE2\x14a\x02sW`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x01\xFEW\x80cU5\xDB\xF6\x14a\x02\x1EW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01TW\x80c\x05\xF3xS\x14a\x01\x89W\x80c$\x8A\x9C\xA3\x14a\x01\xA0W\x80c//\xF1]\x14a\x01\xDEW`\0\x80\xFD[6a\x01OW\0[`\0\x80\xFD[4\x80\x15a\x01`W`\0\x80\xFD[Pa\x01ta\x01o6`\x04a\x17\xD4V[a\x04\nV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x95W`\0\x80\xFD[Pa\x01\x9Ea\x04\xA3V[\0[4\x80\x15a\x01\xACW`\0\x80\xFD[Pa\x01\xD0a\x01\xBB6`\x04a\x18\x1DV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x80V[4\x80\x15a\x01\xEAW`\0\x80\xFD[Pa\x01\x9Ea\x01\xF96`\x04a\x18XV[a\x05JV[4\x80\x15a\x02\nW`\0\x80\xFD[Pa\x01\x9Ea\x02\x196`\x04a\x18XV[a\x05uV[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x01\xD0a\x0296`\x04a\x19\x94V[a\x05\xD3V[4\x80\x15a\x02JW`\0\x80\xFD[Pa\x01\xD0a\x06`V[4\x80\x15a\x02_W`\0\x80\xFD[Pa\x01\x9Ea\x02n6`\x04a\x1A`V[a\x06uV[4\x80\x15a\x02\x7FW`\0\x80\xFD[Pa\x01\xD0\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[4\x80\x15a\x02\xB3W`\0\x80\xFD[Pa\x02\xC7a\x02\xC26`\x04a\x18\x1DV[a\x0B\xFFV[`@Qa\x01\x80\x93\x92\x91\x90a\x1B\xB6V[4\x80\x15a\x02\xE2W`\0\x80\xFD[Pa\x02\xF6a\x02\xF16`\x04a\x18\x1DV[a\x0C\xC2V[`@Qa\x01\x80\x91\x90a\x1CaV[4\x80\x15a\x03\x0FW`\0\x80\xFD[Pa\x01ta\x03\x1E6`\x04a\x18XV[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x01\xD0`\x02T\x81V[4\x80\x15a\x03vW`\0\x80\xFD[Pa\x01\xD0`\0\x81V[4\x80\x15a\x03\x8BW`\0\x80\xFD[Pa\x01\xD0a\x0F\x7FV[4\x80\x15a\x03\xA0W`\0\x80\xFD[Pa\x01\x9Ea\x03\xAF6`\x04a\x18XV[a\x0F\xB5V[4\x80\x15a\x03\xC0W`\0\x80\xFD[Pa\x01\x9Ea\x03\xCF6`\x04a\x18\x1DV[a\x0F\xDAV[4\x80\x15a\x03\xE0W`\0\x80\xFD[Pa\x01\x9Ea\x03\xEF6`\x04a\x18\x1DV[a\x10wV[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x01\xD0`\x03T\x81V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x04\x9DWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x05@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x05Ha\x11\x14V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05e\x81a\x12\x98V[a\x05o\x83\x83a\x12\xA5V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x05\xC4W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xCE\x82\x82a\x13\xA1V[PPPV[`\0\x80a\x05\xDEa\x06`V[\x90P`\x01\x81\x81T\x81\x10a\x05\xF3Wa\x05\xF3a\x1DwV[`\0\x91\x82R` \x80\x83 `\x01`\x06\x90\x93\x02\x01\x82\x01\x80T\x92\x83\x01\x81U\x83R\x90\x91 \x01a\x06\x1E\x84\x82a\x1EGV[P\x7F\x18J\xF4\xCB\x002d\xF0\xF2:It/\xB8\x14)\xDD8\xBA\x83I*\xDB\xC7\x82\tGpQ\xB8+\xCB\x83\x82`@Qa\x06P\x92\x91\x90a\x1FaV[`@Q\x80\x91\x03\x90\xA1a\x04\x9Da\x14\\V[`\x01\x80T`\0\x91a\x06p\x91a\x1F\xB2V[\x90P\x90V[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x07\rW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\x01Ta\x07\x1C\x90`\x02\x90a\x1F\xB2V[\x84\x11\x15a\x07\x85W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FWrong round\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[\x81Q\x83Q\x14a\x08\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x057V[`\0`\x01\x85\x81T\x81\x10a\x08*Wa\x08*a\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\x1EW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x08\x91\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xBD\x90a\x1D\xA6V[\x80\x15a\t\nW\x80`\x1F\x10a\x08\xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08rV[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x8CW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\taW[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\xE4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t\xD0W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\n\x10\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n<\x90a\x1D\xA6V[\x80\x15a\n\x89W\x80`\x1F\x10a\n^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nlW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`\x80\x01Q\x15a\x0B\0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FScores already submitted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\x01`\x80\x82\x01\x81\x90R\x80T\x82\x91\x90\x87\x90\x81\x10a\x0B3Wa\x0B3a\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x0Bg\x92\x91\x90a\x16QV[P`@\x82\x01Q\x80Qa\x0B\x83\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x16\xA7V[P``\x82\x01Q\x80Qa\x0B\x9F\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x17-V[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\x0B\xEA\x90\x82a\x1EGV[P\x90PPa\x0B\xF8\x84\x84a\x14\x9BV[PPPPPV[`\x01\x81\x81T\x81\x10a\x0C\x0FW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x04\x82\x01T`\x05\x83\x01\x80T\x92\x94P`\xFF\x90\x91\x16\x92\x91a\x0C?\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Ck\x90a\x1D\xA6V[\x80\x15a\x0C\xB8W\x80`\x1F\x10a\x0C\x8DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xB8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x9BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[a\x0C\xFD`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[`\x01\x82\x81T\x81\x10a\r\x10Wa\r\x10a\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0E\x04W\x83\x82\x90`\0R` `\0 \x01\x80Ta\rw\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xA3\x90a\x1D\xA6V[\x80\x15a\r\xF0W\x80`\x1F\x10a\r\xC5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xF0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xD3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\rXV[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0ErW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0EGW[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xCAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\xB6W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\x0E\xF6\x90a\x1D\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\"\x90a\x1D\xA6V[\x80\x15a\x0FoW\x80`\x1F\x10a\x0FDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FoV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FRW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0`\x01a\x0F\x8Ba\x06`V[\x81T\x81\x10a\x0F\x9BWa\x0F\x9Ba\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`\x01\x01\x80T\x90P\x90P\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xD0\x81a\x12\x98V[a\x05o\x83\x83a\x13\xA1V[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x10rW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\x03UV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x11\x0FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\x02UV[a\x11O`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[`\x02Ta\x11\\\x90Ca\x1F\xC5V[\x81R`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x81Q`\x06\x90\x91\x02\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x81\x01\x91\x82U` \x80\x84\x01Q\x80Q\x85\x94\x93a\x11\xD5\x93\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x90\x91\x01\x92\x01\x90a\x16QV[P`@\x82\x01Q\x80Qa\x11\xF1\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x16\xA7V[P``\x82\x01Q\x80Qa\x12\r\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x17-V[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\x12X\x90\x82a\x1EGV[PPP\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sa\x12\x84a\x06`V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x12\xA2\x813a\x15\xC7V[PV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x13\x99W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x1373\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x04\x9DV[P`\0a\x04\x9DV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x13\x99W`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x04\x9DV[`\0`\x01a\x14ha\x06`V[\x81T\x81\x10a\x14xWa\x14xa\x1DwV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P\x80C\x10a\x12\xA2Wa\x12\xA2a\x11\x14V[`\x03TG\x10\x15a\x15\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot enough funds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x057V[`\0[\x82Q\x81\x10\x15a\x05\xCEW`\0\x83\x82\x81Q\x81\x10a\x15'Wa\x15'a\x1DwV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x15EWa\x15Ea\x1DwV[` \x02` \x01\x01Q\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xFC`\x03Tf\x03\x8D~\xA4\xC6\x80\0\x84a\x15\x7F\x91\x90a\x1F\xD8V[a\x15\x89\x91\x90a \x13V[`@Q\x81\x15\x90\x92\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x15\xB1W=`\0\x80>=`\0\xFD[PPP\x80\x80a\x15\xBF\x90a *V[\x91PPa\x15\nV[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x16MW`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x057V[PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x16\x97W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x16\x97W\x82Q\x82\x90a\x16\x87\x90\x82a\x1EGV[P\x91` \x01\x91\x90`\x01\x01\x90a\x16qV[Pa\x16\xA3\x92\x91Pa\x17hV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x17!W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x17!W\x82Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x16\xC7V[Pa\x16\xA3\x92\x91Pa\x17\x85V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x17!W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x17!W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x17MV[\x80\x82\x11\x15a\x16\xA3W`\0a\x17|\x82\x82a\x17\x9AV[P`\x01\x01a\x17hV[[\x80\x82\x11\x15a\x16\xA3W`\0\x81U`\x01\x01a\x17\x86V[P\x80Ta\x17\xA6\x90a\x1D\xA6V[`\0\x82U\x80`\x1F\x10a\x17\xB6WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x12\xA2\x91\x90a\x17\x85V[`\0` \x82\x84\x03\x12\x15a\x17\xE6W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x18\x16W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x18/W`\0\x80\xFD[P5\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xA2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x18kW`\0\x80\xFD[\x825\x91P` \x83\x015a\x18}\x81a\x186V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x18\xFEWa\x18\xFEa\x18\x88V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x19\x17W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x191Wa\x191a\x18\x88V[a\x19b` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x18\xB7V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19wW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19\xA6W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xBDW`\0\x80\xFD[a\x19\xC9\x84\x82\x85\x01a\x19\x06V[\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\xEBWa\x19\xEBa\x18\x88V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1A\x06W`\0\x80\xFD[\x815` a\x1A\x1Ba\x1A\x16\x83a\x19\xD1V[a\x18\xB7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1A:W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1AUW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1A>V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1AvW`\0\x80\xFD[\x845\x93P` \x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\x96W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1A\xAAW`\0\x80\xFD[\x815a\x1A\xB8a\x1A\x16\x82a\x19\xD1V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a\x1A\xD7W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x1A\xFEW\x845a\x1A\xEF\x81a\x186V[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x1A\xDCV[\x97PPP`@\x88\x015\x92P\x80\x83\x11\x15a\x1B\x16W`\0\x80\xFD[a\x1B\"\x89\x84\x8A\x01a\x19\xF5V[\x94P``\x88\x015\x92P\x80\x83\x11\x15a\x1B8W`\0\x80\xFD[PPa\x1BF\x87\x82\x88\x01a\x19\x06V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1BxW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1B\\V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a\x1B\xD7``\x83\x01\x84a\x1BRV[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x1C&W\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x1B\xF4V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x1C&W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x1CEV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`\xC0`@\x84\x01R\x80Q`\xE0\x84\x01\x81\x90R`\0\x92\x91a\x01\0`\x05\x83\x90\x1B\x86\x01\x81\x01\x92\x91\x84\x01\x91\x90\x86\x01\x90\x85[\x81\x81\x10\x15a\x1C\xE7W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x88\x86\x03\x01\x83Ra\x1C\xD5\x85\x85Qa\x1BRV[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01a\x1C\x9BV[PPPP`@\x85\x01Q\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x83\x03\x01``\x86\x01Ra\x1D'\x82\x84a\x1B\xE0V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01Ra\x1DD\x83\x83a\x1C1V[\x92P`\x80\x86\x01Q\x91Pa\x1D[`\xA0\x86\x01\x83\x15\x15\x90RV[`\xA0\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x1B\xD7\x82\x82a\x1BRV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D\xBAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1D\xF3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\xCEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1E WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1E?W\x82\x81U`\x01\x01a\x1E,V[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EaWa\x1Eaa\x18\x88V[a\x1Eu\x81a\x1Eo\x84Ta\x1D\xA6V[\x84a\x1D\xF9V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1E\xC8W`\0\x84\x15a\x1E\x92WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1E?V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x1F\x15W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1E\xF6V[P\x85\x82\x10\x15a\x1FQW\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R`\0a\x1Ft`@\x83\x01\x85a\x1BRV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x9DWa\x04\x9Da\x1F\x83V[\x80\x82\x01\x80\x82\x11\x15a\x04\x9DWa\x04\x9Da\x1F\x83V[`\0\x82a \x0EW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x9DWa\x04\x9Da\x1F\x83V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a [Wa [a\x1F\x83V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x07\xB9\xAD-\xEE+\xA7Z\xCDp\x9EV\x04/X\xC0_\x02\x063\xC6\x95\xD9\x96g\xBD\xAC\xB6-9H4dsolcC\0\x08\x15\x003";
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
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRound` (0x8f1327c0) function
        pub fn get_round(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Round> {
            self.0
                .method_hash([143, 19, 39, 192], index)
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
            (::ethers::core::types::U256, bool, ::std::string::String),
        > {
            self.0
                .method_hash([140, 101, 200, 31], p0)
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
        ///Calls the contract's `setScores` (0x7699ea11) function
        pub fn set_scores(
            &self,
            round_index: ::ethers::core::types::U256,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            scores: ::std::vec::Vec<::ethers::core::types::U256>,
            summary_text: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [118, 153, 234, 17],
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
    #[ethevent(name = "MeasurementsAdded", abi = "MeasurementsAdded(string,uint256)")]
    pub struct MeasurementsAddedFilter {
        pub cid: ::std::string::String,
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ImpactEvaluatorEvents {
        MeasurementsAddedFilter(MeasurementsAddedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        RoundStartFilter(RoundStartFilter),
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
    ///Container type for all input parameters for the `getRound` function with signature `getRound(uint256)` and selector `0x8f1327c0`
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
    #[ethcall(name = "getRound", abi = "getRound(uint256)")]
    pub struct GetRoundCall {
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
    ///Container type for all input parameters for the `setScores` function with signature `setScores(uint256,address[],uint256[],string)` and selector `0x7699ea11`
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
    #[ethcall(name = "setScores", abi = "setScores(uint256,address[],uint256[],string)")]
    pub struct SetScoresCall {
        pub round_index: ::ethers::core::types::U256,
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub scores: ::std::vec::Vec<::ethers::core::types::U256>,
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
        GetRoleAdmin(GetRoleAdminCall),
        GetRound(GetRoundCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        NextRoundLength(NextRoundLengthCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RoundReward(RoundRewardCall),
        Rounds(RoundsCall),
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
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GetRoundCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRound(decoded));
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
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRoundLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rounds(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetRoleAdminCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoundCall> for ImpactEvaluatorCalls {
        fn from(value: GetRoundCall) -> Self {
            Self::GetRound(value)
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
    ///Container type for all return fields from the `getRound` function with signature `getRound(uint256)` and selector `0x8f1327c0`
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
    pub struct GetRoundReturn(pub Round);
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
    ///`Round(uint256,string[],address[],uint256[],bool,string)`
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
    pub struct Round {
        pub end: ::ethers::core::types::U256,
        pub measurements_cids: ::std::vec::Vec<::std::string::String>,
        pub participant_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub participant_scores: ::std::vec::Vec<::ethers::core::types::U256>,
        pub scores_submitted: bool,
        pub summary_text: ::std::string::String,
    }
}
