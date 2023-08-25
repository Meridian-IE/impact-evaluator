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
                    ::std::borrow::ToOwned::to_owned("addMeasurement"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addMeasurement"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
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
                                    name: ::std::borrow::ToOwned::to_owned("start"),
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
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("MeasurementAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MeasurementAdded"),
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IMPACTEVALUATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\n`\x02U4\x80\x15b\0\0\x16W`\0\x80\xFD[P`@Qb\0!\xB88\x03\x80b\0!\xB8\x839\x81\x01`@\x81\x90Rb\0\09\x91b\0\x041V[b\0\0F`\0\x82b\0\0\x85V[Pb\0\0s\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x82b\0\0\x85V[Pb\0\0~b\0\x014V[Pb\0\x05\xF6V[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16b\0\x01*W`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\xE13\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01b\0\x01.V[P`\0[\x92\x91PPV[b\0\x01p`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[C\x81R`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x81Q`\x06\x90\x91\x02\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x81\x01\x91\x82U` \x80\x84\x01Q\x80Q\x85\x94\x93b\0\x01\xEC\x93\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x90\x91\x01\x92\x01\x90b\0\x02\xB6V[P`@\x82\x01Q\x80Qb\0\x02\n\x91`\x02\x84\x01\x91` \x90\x91\x01\x90b\0\x03\x13V[P``\x82\x01Q\x80Qb\0\x02(\x91`\x03\x84\x01\x91` \x90\x91\x01\x90b\0\x03yV[P`\x80\x82\x01Q`\x04\x82\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90b\0\x02W\x90\x82b\0\x05\x08V[PPP\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sb\0\x02\x8Bb\0\x02\x9F` \x1B` \x1CV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01\x80T`\0\x91b\0\x02\xB1\x91b\0\x05\xD4V[\x90P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x03\x01W\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x03\x01W\x82Q\x82\x90b\0\x02\xF0\x90\x82b\0\x05\x08V[P\x91` \x01\x91\x90`\x01\x01\x90b\0\x02\xD7V[Pb\0\x03\x0F\x92\x91Pb\0\x03\xB7V[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x03kW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x03kW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\x034V[Pb\0\x03\x0F\x92\x91Pb\0\x03\xD8V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x03kW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x03kW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x03\x9AV[\x80\x82\x11\x15b\0\x03\x0FW`\0b\0\x03\xCE\x82\x82b\0\x03\xEFV[P`\x01\x01b\0\x03\xB7V[[\x80\x82\x11\x15b\0\x03\x0FW`\0\x81U`\x01\x01b\0\x03\xD9V[P\x80Tb\0\x03\xFD\x90b\0\x04yV[`\0\x82U\x80`\x1F\x10b\0\x04\x0EWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0\x04.\x91\x90b\0\x03\xD8V[PV[`\0` \x82\x84\x03\x12\x15b\0\x04DW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x04\\W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x04\x8EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x04\xAFWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x05\x03W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x04\xDEWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x04\xFFW\x82\x81U`\x01\x01b\0\x04\xEAV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x05$Wb\0\x05$b\0\x04cV[b\0\x05<\x81b\0\x055\x84Tb\0\x04yV[\x84b\0\x04\xB5V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x05tW`\0\x84\x15b\0\x05[WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x04\xFFV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x05\xA5W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x05\x84V[P\x85\x82\x10\x15b\0\x05\xC4W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15b\0\x01.WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[a\x1B\xB2\x80b\0\x06\x06`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80cxs\xCD\xE2\x11a\0\x8CW\x80c\x91\xD1HT\x11a\0fW\x80c\x91\xD1HT\x14a\x01\xFCW\x80c\xA2\x17\xFD\xDF\x14a\x02@W\x80c\xD5Gt\x1F\x14a\x02HW\x80c\xFEev\xA9\x14a\x02[W`\0\x80\xFD[\x80cxs\xCD\xE2\x14a\x01\x93W\x80c\x8Ce\xC8\x1F\x14a\x01\xBAW\x80c\x8F\x13'\xC0\x14a\x01\xDCW`\0\x80\xFD[\x80c//\xF1]\x11a\0\xC8W\x80c//\xF1]\x14a\x01RW\x80c6V\x8A\xBE\x14a\x01eW\x80ch\x96\xEFK\x14a\x01xW\x80cv\x99\xEA\x11\x14a\x01\x80W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xEFW\x80c\x05\xF3xS\x14a\x01\x17W\x80c$\x8A\x9C\xA3\x14a\x01!W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a\x13\x92V[a\x02nV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Fa\x03\x07V[\0[a\x01Da\x01/6`\x04a\x13\xDBV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x0EV[a\x01\x1Fa\x01`6`\x04a\x14\x1DV[a\x03\xAEV[a\x01\x1Fa\x01s6`\x04a\x14\x1DV[a\x03\xD9V[a\x01Da\x047V[a\x01\x1Fa\x01\x8E6`\x04a\x15\xE4V[a\x04LV[a\x01D\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[a\x01\xCDa\x01\xC86`\x04a\x13\xDBV[a\t\xCAV[`@Qa\x01\x0E\x93\x92\x91\x90a\x178V[a\x01\xEFa\x01\xEA6`\x04a\x13\xDBV[a\n\x8DV[`@Qa\x01\x0E\x91\x90a\x17\xE3V[a\x01\x02a\x02\n6`\x04a\x14\x1DV[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x01D`\0\x81V[a\x01\x1Fa\x02V6`\x04a\x14\x1DV[a\rJV[a\x01\x1Fa\x02i6`\x04a\x18\xF9V[a\roV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x03\x01WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x03\xA4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xACa\x0E\0V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03\xC9\x81a\x0FxV[a\x03\xD3\x83\x83a\x0F\x82V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x04(W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x042\x82\x82a\x10~V[PPPV[`\x01\x80T`\0\x91a\x04G\x91a\x196V[\x90P\x90V[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x04\xE4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x9BV[`\x01Ta\x04\xF3\x90`\x02\x90a\x196V[\x84\x14a\x05[W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FWrong round\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x9BV[\x81Q\x83Q\x14a\x05\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x9BV[`\0`\x01\x85\x81T\x81\x10a\x06\0Wa\x06\0a\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xF4W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06g\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x93\x90a\x19\x9FV[\x80\x15a\x06\xE0W\x80`\x1F\x10a\x06\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06HV[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07bW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x077W[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xBAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07\xA6W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\x07\xE6\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\x12\x90a\x19\x9FV[\x80\x15a\x08_W\x80`\x1F\x10a\x084Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08_V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`\x80\x01Q\x15a\x08\xD6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FScores already submitted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x9BV[`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\x01`\x80\x82\x01\x81\x90R\x80T\x82\x91\x90\x87\x90\x81\x10a\t\tWa\t\ta\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\t=\x92\x91\x90a\x12\x0FV[P`@\x82\x01Q\x80Qa\tY\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x12eV[P``\x82\x01Q\x80Qa\tu\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x12\xEBV[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\t\xC0\x90\x82a\x1A@V[PPPPPPPPV[`\x01\x81\x81T\x81\x10a\t\xDAW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x04\x82\x01T`\x05\x83\x01\x80T\x92\x94P`\xFF\x90\x91\x16\x92\x91a\n\n\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n6\x90a\x19\x9FV[\x80\x15a\n\x83W\x80`\x1F\x10a\nXWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x83V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nfW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[a\n\xC8`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[`\x01\x82\x81T\x81\x10a\n\xDBWa\n\xDBa\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xCFW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0BB\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bn\x90a\x19\x9FV[\x80\x15a\x0B\xBBW\x80`\x1F\x10a\x0B\x90Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xBBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x9EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0B#V[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0C=W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0C\x12W[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0C\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0C\x81W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\x0C\xC1\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\xED\x90a\x19\x9FV[\x80\x15a\r:W\x80`\x1F\x10a\r\x0FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r:V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\re\x81a\x0FxV[a\x03\xD3\x83\x83a\x10~V[`\x01a\rya\x047V[\x81T\x81\x10a\r\x89Wa\r\x89a\x19pV[`\0\x91\x82R` \x80\x83 `\x01`\x06\x90\x93\x02\x01\x82\x01\x80T\x92\x83\x01\x81U\x83R\x90\x91 \x01a\r\xB4\x82\x82a\x1A@V[P\x7F\xF6\x98\x15\xDD\xD5\xB3\xC0]#\xFE\xEB]q=\xB87[\x9C\x1F\x10\xD9\xE8\xB0\xEC=\x80\xEF\0i\xDE(\xC4\x81a\r\xDFa\x047V[`@Qa\r\xED\x92\x91\x90a\x1BZV[`@Q\x80\x91\x03\x90\xA1a\r\xFDa\x11=V[PV[a\x0E;`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[C\x81R`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x81Q`\x06\x90\x91\x02\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x81\x01\x91\x82U` \x80\x84\x01Q\x80Q\x85\x94\x93a\x0E\xB5\x93\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x90\x91\x01\x92\x01\x90a\x12\x0FV[P`@\x82\x01Q\x80Qa\x0E\xD1\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x12eV[P``\x82\x01Q\x80Qa\x0E\xED\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x12\xEBV[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\x0F8\x90\x82a\x1A@V[PPP\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sa\x0Fda\x047V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[a\r\xFD\x813a\x11\x89V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x10vW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x10\x143\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x03\x01V[P`\0a\x03\x01V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x10vW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x03\x01V[PPV[`\0`\x01a\x11Ia\x047V[\x81T\x81\x10a\x11YWa\x11Ya\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P`\x02T\x81Ca\x11|\x91\x90a\x196V[\x10a\r\xFDWa\r\xFDa\x0E\0V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x119W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x03\x9BV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12UW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12UW\x82Q\x82\x90a\x12E\x90\x82a\x1A@V[P\x91` \x01\x91\x90`\x01\x01\x90a\x12/V[Pa\x12a\x92\x91Pa\x13&V[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12\xDFW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12\xDFW\x82Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x12\x85V[Pa\x12a\x92\x91Pa\x13CV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12\xDFW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12\xDFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x13\x0BV[\x80\x82\x11\x15a\x12aW`\0a\x13:\x82\x82a\x13XV[P`\x01\x01a\x13&V[[\x80\x82\x11\x15a\x12aW`\0\x81U`\x01\x01a\x13DV[P\x80Ta\x13d\x90a\x19\x9FV[`\0\x82U\x80`\x1F\x10a\x13tWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\r\xFD\x91\x90a\x13CV[`\0` \x82\x84\x03\x12\x15a\x13\xA4W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x13\xD4W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13\xEDW`\0\x80\xFD[P5\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\x18W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x140W`\0\x80\xFD[\x825\x91Pa\x14@` \x84\x01a\x13\xF4V[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xBFWa\x14\xBFa\x14IV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\xE1Wa\x14\xE1a\x14IV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x14\xFCW`\0\x80\xFD[\x815` a\x15\x11a\x15\x0C\x83a\x14\xC7V[a\x14xV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x150W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x15KW\x805\x83R\x91\x83\x01\x91\x83\x01a\x154V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x15gW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x81Wa\x15\x81a\x14IV[a\x15\xB2` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x14xV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x15\xC7W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\xFAW`\0\x80\xFD[\x845\x93P` \x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\x1AW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x16.W`\0\x80\xFD[\x815a\x16<a\x15\x0C\x82a\x14\xC7V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a\x16[W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x16\x80Wa\x16q\x85a\x13\xF4V[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x16`V[\x97PPP`@\x88\x015\x92P\x80\x83\x11\x15a\x16\x98W`\0\x80\xFD[a\x16\xA4\x89\x84\x8A\x01a\x14\xEBV[\x94P``\x88\x015\x92P\x80\x83\x11\x15a\x16\xBAW`\0\x80\xFD[PPa\x16\xC8\x87\x82\x88\x01a\x15VV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x16\xFAW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x16\xDEV[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a\x17Y``\x83\x01\x84a\x16\xD4V[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x17\xA8W\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x17vV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x17\xA8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x17\xC7V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`\xC0`@\x84\x01R\x80Q`\xE0\x84\x01\x81\x90R`\0\x92\x91a\x01\0`\x05\x83\x90\x1B\x86\x01\x81\x01\x92\x91\x84\x01\x91\x90\x86\x01\x90\x85[\x81\x81\x10\x15a\x18iW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x88\x86\x03\x01\x83Ra\x18W\x85\x85Qa\x16\xD4V[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01a\x18\x1DV[PPPP`@\x85\x01Q\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x83\x03\x01``\x86\x01Ra\x18\xA9\x82\x84a\x17bV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01Ra\x18\xC6\x83\x83a\x17\xB3V[\x92P`\x80\x86\x01Q\x91Pa\x18\xDD`\xA0\x86\x01\x83\x15\x15\x90RV[`\xA0\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x17Y\x82\x82a\x16\xD4V[`\0` \x82\x84\x03\x12\x15a\x19\x0BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\"W`\0\x80\xFD[a\x19.\x84\x82\x85\x01a\x15VV[\x94\x93PPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03\x01W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x19\xECW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x042W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1A\x19WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1A8W\x82\x81U`\x01\x01a\x1A%V[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AZWa\x1AZa\x14IV[a\x1An\x81a\x1Ah\x84Ta\x19\x9FV[\x84a\x19\xF2V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1A\xC1W`\0\x84\x15a\x1A\x8BWP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1A8V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x1B\x0EW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1A\xEFV[P\x85\x82\x10\x15a\x1BJW\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R`\0a\x1Bm`@\x83\x01\x85a\x16\xD4V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 W\xB7-\xBBm(\x01\x86)\xF2[\xE1U\xBF\x15^\xB2\x01\x07\xB4\xEB\x9F\xBE\xD3m\"!\x91\x18\xF3\xF9\x8AdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static IMPACTEVALUATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80cxs\xCD\xE2\x11a\0\x8CW\x80c\x91\xD1HT\x11a\0fW\x80c\x91\xD1HT\x14a\x01\xFCW\x80c\xA2\x17\xFD\xDF\x14a\x02@W\x80c\xD5Gt\x1F\x14a\x02HW\x80c\xFEev\xA9\x14a\x02[W`\0\x80\xFD[\x80cxs\xCD\xE2\x14a\x01\x93W\x80c\x8Ce\xC8\x1F\x14a\x01\xBAW\x80c\x8F\x13'\xC0\x14a\x01\xDCW`\0\x80\xFD[\x80c//\xF1]\x11a\0\xC8W\x80c//\xF1]\x14a\x01RW\x80c6V\x8A\xBE\x14a\x01eW\x80ch\x96\xEFK\x14a\x01xW\x80cv\x99\xEA\x11\x14a\x01\x80W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xEFW\x80c\x05\xF3xS\x14a\x01\x17W\x80c$\x8A\x9C\xA3\x14a\x01!W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a\x13\x92V[a\x02nV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Fa\x03\x07V[\0[a\x01Da\x01/6`\x04a\x13\xDBV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01\x0EV[a\x01\x1Fa\x01`6`\x04a\x14\x1DV[a\x03\xAEV[a\x01\x1Fa\x01s6`\x04a\x14\x1DV[a\x03\xD9V[a\x01Da\x047V[a\x01\x1Fa\x01\x8E6`\x04a\x15\xE4V[a\x04LV[a\x01D\x7F\x94\x98\xF1\x0E';\xD1\xC89j\r\x1C\xC9\x11\x17\xE2\xF1\xA5\xA5\xF3\xB4\x11,vS\x182\xA9\x11\xB4Di\x81V[a\x01\xCDa\x01\xC86`\x04a\x13\xDBV[a\t\xCAV[`@Qa\x01\x0E\x93\x92\x91\x90a\x178V[a\x01\xEFa\x01\xEA6`\x04a\x13\xDBV[a\n\x8DV[`@Qa\x01\x0E\x91\x90a\x17\xE3V[a\x01\x02a\x02\n6`\x04a\x14\x1DV[`\0\x91\x82R` \x82\x81R`@\x80\x84 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x01D`\0\x81V[a\x01\x1Fa\x02V6`\x04a\x14\x1DV[a\rJV[a\x01\x1Fa\x02i6`\x04a\x18\xF9V[a\roV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x03\x01WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x90 T`\xFF\x16a\x03\xA4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xACa\x0E\0V[V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03\xC9\x81a\x0FxV[a\x03\xD3\x83\x83a\x0F\x82V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x04(W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x042\x82\x82a\x10~V[PPPV[`\x01\x80T`\0\x91a\x04G\x91a\x196V[\x90P\x90V[3`\0\x90\x81R\x7Fi\xDFD`\xCF\x80l\xEB\xFA(O{&yQL\x9E\xCE]UMU\xBE\x17=\x9D\xC2\xEA\xD8\x1A\x92\xB4` R`@\x90 T`\xFF\x16a\x04\xE4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNot an evaluator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x9BV[`\x01Ta\x04\xF3\x90`\x02\x90a\x196V[\x84\x14a\x05[W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FWrong round\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x9BV[\x81Q\x83Q\x14a\x05\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FAddresses and scores length mism`D\x82\x01R\x7Fatch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x9BV[`\0`\x01\x85\x81T\x81\x10a\x06\0Wa\x06\0a\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xF4W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06g\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x93\x90a\x19\x9FV[\x80\x15a\x06\xE0W\x80`\x1F\x10a\x06\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06HV[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07bW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x077W[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xBAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07\xA6W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\x07\xE6\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\x12\x90a\x19\x9FV[\x80\x15a\x08_W\x80`\x1F\x10a\x084Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08_V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`\x80\x01Q\x15a\x08\xD6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FScores already submitted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x9BV[`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\x01`\x80\x82\x01\x81\x90R\x80T\x82\x91\x90\x87\x90\x81\x10a\t\tWa\t\ta\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\t=\x92\x91\x90a\x12\x0FV[P`@\x82\x01Q\x80Qa\tY\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x12eV[P``\x82\x01Q\x80Qa\tu\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x12\xEBV[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\t\xC0\x90\x82a\x1A@V[PPPPPPPPV[`\x01\x81\x81T\x81\x10a\t\xDAW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x04\x82\x01T`\x05\x83\x01\x80T\x92\x94P`\xFF\x90\x91\x16\x92\x91a\n\n\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n6\x90a\x19\x9FV[\x80\x15a\n\x83W\x80`\x1F\x10a\nXWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x83V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nfW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[a\n\xC8`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[`\x01\x82\x81T\x81\x10a\n\xDBWa\n\xDBa\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`@Q\x80`\xC0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xCFW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0BB\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bn\x90a\x19\x9FV[\x80\x15a\x0B\xBBW\x80`\x1F\x10a\x0B\x90Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xBBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x9EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0B#V[PPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0C=W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0C\x12W[PPPPP\x81R` \x01`\x03\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0C\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0C\x81W[PPP\x91\x83RPP`\x04\x82\x01T`\xFF\x16\x15\x15` \x82\x01R`\x05\x82\x01\x80T`@\x90\x92\x01\x91a\x0C\xC1\x90a\x19\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\xED\x90a\x19\x9FV[\x80\x15a\r:W\x80`\x1F\x10a\r\x0FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r:V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\re\x81a\x0FxV[a\x03\xD3\x83\x83a\x10~V[`\x01a\rya\x047V[\x81T\x81\x10a\r\x89Wa\r\x89a\x19pV[`\0\x91\x82R` \x80\x83 `\x01`\x06\x90\x93\x02\x01\x82\x01\x80T\x92\x83\x01\x81U\x83R\x90\x91 \x01a\r\xB4\x82\x82a\x1A@V[P\x7F\xF6\x98\x15\xDD\xD5\xB3\xC0]#\xFE\xEB]q=\xB87[\x9C\x1F\x10\xD9\xE8\xB0\xEC=\x80\xEF\0i\xDE(\xC4\x81a\r\xDFa\x047V[`@Qa\r\xED\x92\x91\x90a\x1BZV[`@Q\x80\x91\x03\x90\xA1a\r\xFDa\x11=V[PV[a\x0E;`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81R` \x01``\x81RP\x90V[C\x81R`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x81Q`\x06\x90\x91\x02\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x81\x01\x91\x82U` \x80\x84\x01Q\x80Q\x85\x94\x93a\x0E\xB5\x93\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x90\x91\x01\x92\x01\x90a\x12\x0FV[P`@\x82\x01Q\x80Qa\x0E\xD1\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a\x12eV[P``\x82\x01Q\x80Qa\x0E\xED\x91`\x03\x84\x01\x91` \x90\x91\x01\x90a\x12\xEBV[P`\x80\x82\x01Q`\x04\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\xA0\x82\x01Q`\x05\x82\x01\x90a\x0F8\x90\x82a\x1A@V[PPP\x7F.\x843\x906\xB9\xCA\xEFm\xA05e\xDD7\xA4-\x04\x1D\x8A\xF7Y\xCC\xFD\xDC\x01bXV\x14l\xE4sa\x0Fda\x047V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[a\r\xFD\x813a\x11\x89V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x10vW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x10\x143\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x03\x01V[P`\0a\x03\x01V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16\x15a\x10vW`\0\x83\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x03\x01V[PPV[`\0`\x01a\x11Ia\x047V[\x81T\x81\x10a\x11YWa\x11Ya\x19pV[\x90`\0R` `\0 \x90`\x06\x02\x01`\0\x01T\x90P`\x02T\x81Ca\x11|\x91\x90a\x196V[\x10a\r\xFDWa\r\xFDa\x0E\0V[`\0\x82\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x119W`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\x03\x9BV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12UW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12UW\x82Q\x82\x90a\x12E\x90\x82a\x1A@V[P\x91` \x01\x91\x90`\x01\x01\x90a\x12/V[Pa\x12a\x92\x91Pa\x13&V[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12\xDFW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12\xDFW\x82Q\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x12\x85V[Pa\x12a\x92\x91Pa\x13CV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x12\xDFW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x12\xDFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x13\x0BV[\x80\x82\x11\x15a\x12aW`\0a\x13:\x82\x82a\x13XV[P`\x01\x01a\x13&V[[\x80\x82\x11\x15a\x12aW`\0\x81U`\x01\x01a\x13DV[P\x80Ta\x13d\x90a\x19\x9FV[`\0\x82U\x80`\x1F\x10a\x13tWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\r\xFD\x91\x90a\x13CV[`\0` \x82\x84\x03\x12\x15a\x13\xA4W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x13\xD4W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13\xEDW`\0\x80\xFD[P5\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\x18W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x140W`\0\x80\xFD[\x825\x91Pa\x14@` \x84\x01a\x13\xF4V[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xBFWa\x14\xBFa\x14IV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\xE1Wa\x14\xE1a\x14IV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x14\xFCW`\0\x80\xFD[\x815` a\x15\x11a\x15\x0C\x83a\x14\xC7V[a\x14xV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x150W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x15KW\x805\x83R\x91\x83\x01\x91\x83\x01a\x154V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x15gW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x81Wa\x15\x81a\x14IV[a\x15\xB2` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x14xV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x15\xC7W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\xFAW`\0\x80\xFD[\x845\x93P` \x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\x1AW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x16.W`\0\x80\xFD[\x815a\x16<a\x15\x0C\x82a\x14\xC7V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a\x16[W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x16\x80Wa\x16q\x85a\x13\xF4V[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x16`V[\x97PPP`@\x88\x015\x92P\x80\x83\x11\x15a\x16\x98W`\0\x80\xFD[a\x16\xA4\x89\x84\x8A\x01a\x14\xEBV[\x94P``\x88\x015\x92P\x80\x83\x11\x15a\x16\xBAW`\0\x80\xFD[PPa\x16\xC8\x87\x82\x88\x01a\x15VV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x16\xFAW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x16\xDEV[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a\x17Y``\x83\x01\x84a\x16\xD4V[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x17\xA8W\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x17vV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x17\xA8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x17\xC7V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`\xC0`@\x84\x01R\x80Q`\xE0\x84\x01\x81\x90R`\0\x92\x91a\x01\0`\x05\x83\x90\x1B\x86\x01\x81\x01\x92\x91\x84\x01\x91\x90\x86\x01\x90\x85[\x81\x81\x10\x15a\x18iW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x88\x86\x03\x01\x83Ra\x18W\x85\x85Qa\x16\xD4V[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01a\x18\x1DV[PPPP`@\x85\x01Q\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x83\x03\x01``\x86\x01Ra\x18\xA9\x82\x84a\x17bV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01Ra\x18\xC6\x83\x83a\x17\xB3V[\x92P`\x80\x86\x01Q\x91Pa\x18\xDD`\xA0\x86\x01\x83\x15\x15\x90RV[`\xA0\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x17Y\x82\x82a\x16\xD4V[`\0` \x82\x84\x03\x12\x15a\x19\x0BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\"W`\0\x80\xFD[a\x19.\x84\x82\x85\x01a\x15VV[\x94\x93PPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03\x01W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x19\xECW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x042W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1A\x19WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1A8W\x82\x81U`\x01\x01a\x1A%V[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AZWa\x1AZa\x14IV[a\x1An\x81a\x1Ah\x84Ta\x19\x9FV[\x84a\x19\xF2V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1A\xC1W`\0\x84\x15a\x1A\x8BWP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1A8V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x1B\x0EW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1A\xEFV[P\x85\x82\x10\x15a\x1BJW\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R`\0a\x1Bm`@\x83\x01\x85a\x16\xD4V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 W\xB7-\xBBm(\x01\x86)\xF2[\xE1U\xBF\x15^\xB2\x01\x07\xB4\xEB\x9F\xBE\xD3m\"!\x91\x18\xF3\xF9\x8AdsolcC\0\x08\x15\x003";
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
        ///Calls the contract's `addMeasurement` (0xfe6576a9) function
        pub fn add_measurement(
            &self,
            cid: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 101, 118, 169], cid)
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
        ///Gets the contract's `MeasurementAdded` event
        pub fn measurement_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MeasurementAddedFilter,
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
    #[ethevent(name = "MeasurementAdded", abi = "MeasurementAdded(string,uint256)")]
    pub struct MeasurementAddedFilter {
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
        MeasurementAddedFilter(MeasurementAddedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        RoundStartFilter(RoundStartFilter),
    }
    impl ::ethers::contract::EthLogDecode for ImpactEvaluatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MeasurementAddedFilter::decode_log(log) {
                return Ok(ImpactEvaluatorEvents::MeasurementAddedFilter(decoded));
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
                Self::MeasurementAddedFilter(element) => {
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
    impl ::core::convert::From<MeasurementAddedFilter> for ImpactEvaluatorEvents {
        fn from(value: MeasurementAddedFilter) -> Self {
            Self::MeasurementAddedFilter(value)
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
    ///Container type for all input parameters for the `addMeasurement` function with signature `addMeasurement(string)` and selector `0xfe6576a9`
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
    #[ethcall(name = "addMeasurement", abi = "addMeasurement(string)")]
    pub struct AddMeasurementCall {
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
        AddMeasurement(AddMeasurementCall),
        AdminAdvanceRound(AdminAdvanceRoundCall),
        CurrentRoundIndex(CurrentRoundIndexCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRound(GetRoundCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        Rounds(RoundsCall),
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
                = <AddMeasurementCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddMeasurement(decoded));
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
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <RoundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rounds(decoded));
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
                Self::AddMeasurement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminAdvanceRound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentRoundIndex(element) => {
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
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::AddMeasurement(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminAdvanceRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentRoundIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rounds(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AddMeasurementCall> for ImpactEvaluatorCalls {
        fn from(value: AddMeasurementCall) -> Self {
            Self::AddMeasurement(value)
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
    impl ::core::convert::From<RoundsCall> for ImpactEvaluatorCalls {
        fn from(value: RoundsCall) -> Self {
            Self::Rounds(value)
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
        pub start: ::ethers::core::types::U256,
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
        pub start: ::ethers::core::types::U256,
        pub measurement_cids: ::std::vec::Vec<::std::string::String>,
        pub participant_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub participant_scores: ::std::vec::Vec<::ethers::core::types::U256>,
        pub scores_submitted: bool,
        pub summary_text: ::std::string::String,
    }
}
