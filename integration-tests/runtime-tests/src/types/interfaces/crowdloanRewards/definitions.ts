import { DefinitionRpc } from "@polkadot/types/types";

export default {
  rpc: {
    amountAvailableToClaimFor: {
      description: "The unclaimed amount",
      params: [
        {
          name: "accountId",
          type: "AccountId"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        }
      ],
      type: "Balance"
    },
  },
  types: {
    ComposableTraitsAssetsXcmAssetLocation: "Null",
    PalletCrowdloanRewardsModelsReward: "Null",
    PalletCrowdloanRewardsModelsRemoteAccount: {
      _enum: {
        RelayChain: 'AccountId32',
        Ethereum: 'EthereumAccountId'
      }
    },
    ComposableTraitsCallFilterCallFilterEntry: "Null",
    PalletAssetsRegistryCandidateStatus: "Null",
    SpConsensusAuraSr25519AppSr25519Public: "Null",
    ComposableTraitsBondedFinanceBondOffer: "Null",
    PalletCollatorSelectionCandidateInfo: "Null",
    PalletCrowdloanRewardsReward: "Null",
    PalletDemocracyVoteThreshold: "Null",
    PalletDemocracyPreimageStatus: "Null",
    PalletDemocracyReferendumInfo: "Null",
    PalletPreimageRequestStatus: "Null",
    PalletDemocracyReleases: "Null",
    PalletDemocracyVoteVoting: "Null",
    CumulusPalletDmpQueueConfigData: "Null",
    PalletDutchAuctionSellOrder: "Null",
    ComposableTraitsVestingVestingSchedule: "Null",
    CumulusPalletDmpQueuePageIndexData: "Null",
    PalletDutchAuctionTakeOrder: "Null",
    ComposableTraitsGovernanceSignedRawOrigin: "Null",
    PalletIdentityRegistration: "Null",
    PalletIdentityRegistrarInfo: "Null",
    PolkadotPrimitivesV1AbridgedHostConfiguration: "Null",
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: "Null",
    PolkadotPrimitivesV1PersistedValidationData: "PersistedValidationData",
    PalletSchedulerScheduledV2: "Null",
    PalletSchedulerReleases: "Null",
    PalletSchedulerScheduledV3: "Null",
    DaliRuntimeOpaqueSessionKeys: "Null",
    OrmlTokensAccountData: "Null",
    OrmlTokensBalanceLock: "Null",
    PalletTreasuryProposal: "Null",
    PalletVaultModelsStrategyOverview: "Null",
    PalletVaultModelsVaultInfo: "Null",
    CumulusPalletXcmpQueueInboundStatus: "Null",
    CumulusPalletXcmpQueueInboundChannelDetails: "Null",
    PolkadotParachainPrimitivesXcmpMessageFormat: "Null",
    CumulusPalletXcmpQueueOutboundStatus: "Null",
    CumulusPalletXcmpQueueQueueConfigData: "Null",
    CumulusPalletXcmpQueueOutboundChannelDetails: "Null",
    PalletCrowdloanRewardsModelsProof: {
      _enum: {
        RelayChain: '(AccountId32, MultiSignature)',
        Ethereum: 'EcdsaSignature'
      }
    },
    PalletDemocracyConviction: "Null",
    PalletDemocracyVoteAccountVote: "Null",
    ComposableTraitsDefiSell: "Null",
    ComposableTraitsAuctionAuctionStepFunction: "Null",
    ComposableTraitsDefiTake: "Null",
    ComposableTraitsTimeTimeReleaseFunction: {
      _enum: {
        LinearDecrease: "ComposableTraitsTimeLinearDecrease",
        StairstepExponentialDecrease: "ComposableTraitsTimeStairstepExponentialDecrease"
      }
    },
    ComposableTraitsTimeLinearDecrease: {
      total: "u64"
    },
    ComposableTraitsTimeStairstepExponentialDecrease: {
      step: "u64",
      cut: "Permill"
    },
    PalletIdentityJudgement: "Null",
    PalletIdentityBitFlags: "Null",
    PalletIdentityIdentityInfo: "Null",
    CumulusPrimitivesParachainInherentParachainInherentData: 'ParachainInherentData',
    DaliRuntimeOriginCaller: "Null",
    ComposableTraitsVaultVaultConfig: "Null",
    XcmVersionedMultiAsset: "Null",
    PalletMosaicNetworkInfo: {
      enabled: "bool",
      maxTransferSize: "Balance"
    },
    PalletMosaicDecayBudgetPenaltyDecayer: "Null",
    PalletAssetsRegistryForeignMetadata: "Null",
    PalletMosaicAssetInfo: "Null",
    PalletMosaicRelayerStaleRelayer: "Null",
    FrameSupportScheduleMaybeHashed: "Null",
    FrameSupportScheduleLookupError: "Null",
    PalletCurrencyFactoryRanges: "Null",
    PalletCurrencyFactoryRangesRange: "Null",
    ComposableTraitsLendingCreateInput: {
      updatable: "ComposableTraitsLendingUpdateInput",
      reservedFactor: "Perquintill"
    },
    ComposableTraitsLendingUpdateInput: {
      collateralFactor: "u128",
      underCollaterizedWarnPercent: "Percent",
      liquidators: "Vec<u32>",
      interestRateModel: "ComposableTraitslendingMathInterestRateModel",
      currencyPair: "ComposableTraitsDefiCurrencyPair",
    },
    ComposableTraitsLendingMarketConfig: "Null",
    ComposableTraitsDefiCurrencyPair: {
      base: "u128",
      quote: "u128"
    },
    ComposableTraitslendingMathInterestRateModel: {
      _enum: {
        Curve: "ComposableTraitsLendingMathCurveModel",
        Jump: "ComposableTraitsLendingMathJumpModel",
        DynamicPIDController: "ComposableTraitsLendingMathDynamicPIDControllerModel",
        DoubleExponent: "ComposableTraitsLendingMathDoubleExponentModel"
      }
    },
    ComposableTraitsLendingMathJumpModel: {
      baseRate: "u128",
      jumpRate: "u128",
      fullRate: "u128",
      targetUtilization: "Percent"
    },
    ComposableTraitsLendingMathCurveModel: {
      baseRate: "u128"
    },
    ComposableTraitsLendingMathDynamicPIDControllerModel: {
      proportionalParameter: "i128",
      integralParameter: "i128",
      derivativeParameter: "i128",
      previousErrorValue: "i128",
      previousIntegralTerm: "i128",
      previousInterestRate: "u128",
      targetUtilization: "u128"
    },
    ComposableTraitsLendingMathDoubleExponentModel: {
      coefficients: "[u8;16]"
    },
    PalletLiquidationsLiquidationStrategyConfiguration: {
      _enum: {
        DutchAuction: "ComposableTraitsTimeTimeReleaseFunction",
        UniswapV2: "Null",
        XcmDex: "Null"
      }
    },
    PalletOracleAssetInfo: "Null",
    PalletOracleWithdraw: {
      stake: 'u128',
      unlockBlock: 'u32'
    },
    PalletOraclePrePrice: "Null",
    PalletOraclePrice: "Null",
    ComposableTraitsOraclePrice: "Null"
  },
};
