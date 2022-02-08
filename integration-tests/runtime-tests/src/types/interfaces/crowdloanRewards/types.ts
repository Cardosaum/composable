// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { BTreeMap, Enum, Null, Struct, U8aFixed, Vec, bool, i128, u128, u32, u64 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { EthereumAccountId } from '@polkadot/types/interfaces/eth';
import type { EcdsaSignature, MultiSignature } from '@polkadot/types/interfaces/extrinsics';
import type { ParachainInherentData, PersistedValidationData } from '@polkadot/types/interfaces/parachains';
import type { AccountId32, Balance, Percent, Permill, Perquintill } from '@polkadot/types/interfaces/runtime';

/** @name ComposableSupportValidationValidated */
export interface ComposableSupportValidationValidated extends Struct {
  readonly value: ComposableTraitsLendingCreateInput;
  readonly reservedFactor: Perquintill;
}

/** @name ComposableTraitsAssetsXcmAssetLocation */
export interface ComposableTraitsAssetsXcmAssetLocation extends Null {}

/** @name ComposableTraitsAuctionAuctionStepFunction */
export interface ComposableTraitsAuctionAuctionStepFunction extends Null {}

/** @name ComposableTraitsBondedFinanceBondOffer */
export interface ComposableTraitsBondedFinanceBondOffer extends Null {}

/** @name ComposableTraitsCallFilterCallFilterEntry */
export interface ComposableTraitsCallFilterCallFilterEntry extends Null {}

/** @name ComposableTraitsDefiCurrencyPair */
export interface ComposableTraitsDefiCurrencyPair extends Struct {
  readonly base: u128;
  readonly quote: u128;
}

/** @name ComposableTraitsDefiSell */
export interface ComposableTraitsDefiSell extends Null {}

/** @name ComposableTraitsDefiTake */
export interface ComposableTraitsDefiTake extends Null {}

/** @name ComposableTraitsGovernanceSignedRawOrigin */
export interface ComposableTraitsGovernanceSignedRawOrigin extends Null {}

/** @name ComposableTraitsLendingCreateInput */
export interface ComposableTraitsLendingCreateInput extends Struct {
  readonly updatable: ComposableTraitsLendingUpdateInput;
  readonly currencyPair: ComposableTraitsDefiCurrencyPair;
}

/** @name ComposableTraitsLendingMarketConfig */
export interface ComposableTraitsLendingMarketConfig extends Null {}

/** @name ComposableTraitsLendingMathCurveModel */
export interface ComposableTraitsLendingMathCurveModel extends Struct {
  readonly baseRate: u128;
}

/** @name ComposableTraitsLendingMathDoubleExponentModel */
export interface ComposableTraitsLendingMathDoubleExponentModel extends Struct {
  readonly coefficients: U8aFixed;
}

/** @name ComposableTraitsLendingMathDynamicPIDControllerModel */
export interface ComposableTraitsLendingMathDynamicPIDControllerModel extends Struct {
  readonly proportionalParameter: i128;
  readonly integralParameter: i128;
  readonly derivativeParameter: i128;
  readonly previousErrorValue: i128;
  readonly previousIntegralTerm: i128;
  readonly previousInterestRate: u128;
  readonly targetUtilization: u128;
}

/** @name ComposableTraitslendingMathInterestRateModel */
export interface ComposableTraitslendingMathInterestRateModel extends Enum {
  readonly isCurve: boolean;
  readonly asCurve: ComposableTraitsLendingMathCurveModel;
  readonly isJump: boolean;
  readonly asJump: ComposableTraitsLendingMathJumpModel;
  readonly isDynamicPIDController: boolean;
  readonly asDynamicPIDController: ComposableTraitsLendingMathDynamicPIDControllerModel;
  readonly isDoubleExponent: boolean;
  readonly asDoubleExponent: ComposableTraitsLendingMathDoubleExponentModel;
  readonly type: 'Curve' | 'Jump' | 'DynamicPIDController' | 'DoubleExponent';
}

/** @name ComposableTraitsLendingMathJumpModel */
export interface ComposableTraitsLendingMathJumpModel extends Struct {
  readonly baseRate: u128;
  readonly jumpRate: u128;
  readonly fullRate: u128;
  readonly targetUtilization: Percent;
}

/** @name ComposableTraitsLendingUpdateInput */
export interface ComposableTraitsLendingUpdateInput extends Struct {
  readonly collateralFactor: u128;
  readonly underCollaterizedWarnPercent: Percent;
  readonly liquidators: Vec<u32>;
  readonly interestRateModel: ComposableTraitslendingMathInterestRateModel;
}

/** @name ComposableTraitsTimeLinearDecrease */
export interface ComposableTraitsTimeLinearDecrease extends Struct {
  readonly total: u64;
}

/** @name ComposableTraitsTimeStairstepExponentialDecrease */
export interface ComposableTraitsTimeStairstepExponentialDecrease extends Struct {
  readonly step: u64;
  readonly cut: Permill;
}

/** @name ComposableTraitsTimeTimeReleaseFunction */
export interface ComposableTraitsTimeTimeReleaseFunction extends Enum {
  readonly isLinearDecrease: boolean;
  readonly asLinearDecrease: {
    readonly type: ComposableTraitsTimeLinearDecrease;
  } & Struct;
  readonly isStairstepExponentialDecrease: boolean;
  readonly asStairstepExponentialDecrease: ComposableTraitsTimeStairstepExponentialDecrease;
  readonly type: 'LinearDecrease' | 'StairstepExponentialDecrease';
}

/** @name ComposableTraitsVaultVaultConfig */
export interface ComposableTraitsVaultVaultConfig extends Struct {
  readonly assetId: u128;
  readonly reserved: Perquintill;
  readonly manager: AccountId32;
  readonly strategies: BTreeMap<AccountId32, Perquintill>;
}

/** @name ComposableTraitsVestingVestingSchedule */
export interface ComposableTraitsVestingVestingSchedule extends Null {}

/** @name CumulusPalletDmpQueueConfigData */
export interface CumulusPalletDmpQueueConfigData extends Null {}

/** @name CumulusPalletDmpQueuePageIndexData */
export interface CumulusPalletDmpQueuePageIndexData extends Null {}

/** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot */
export interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Null {}

/** @name CumulusPalletXcmpQueueInboundChannelDetails */
export interface CumulusPalletXcmpQueueInboundChannelDetails extends Null {}

/** @name CumulusPalletXcmpQueueInboundStatus */
export interface CumulusPalletXcmpQueueInboundStatus extends Null {}

/** @name CumulusPalletXcmpQueueOutboundChannelDetails */
export interface CumulusPalletXcmpQueueOutboundChannelDetails extends Null {}

/** @name CumulusPalletXcmpQueueOutboundStatus */
export interface CumulusPalletXcmpQueueOutboundStatus extends Null {}

/** @name CumulusPalletXcmpQueueQueueConfigData */
export interface CumulusPalletXcmpQueueQueueConfigData extends Null {}

/** @name CumulusPrimitivesParachainInherentParachainInherentData */
export interface CumulusPrimitivesParachainInherentParachainInherentData extends ParachainInherentData {}

/** @name DaliRuntimeOpaqueSessionKeys */
export interface DaliRuntimeOpaqueSessionKeys extends Null {}

/** @name DaliRuntimeOriginCaller */
export interface DaliRuntimeOriginCaller extends Null {}

/** @name FrameSupportScheduleLookupError */
export interface FrameSupportScheduleLookupError extends Null {}

/** @name FrameSupportScheduleMaybeHashed */
export interface FrameSupportScheduleMaybeHashed extends Null {}

/** @name OrmlTokensAccountData */
export interface OrmlTokensAccountData extends Null {}

/** @name OrmlTokensBalanceLock */
export interface OrmlTokensBalanceLock extends Null {}

/** @name PalletAssetsRegistryCandidateStatus */
export interface PalletAssetsRegistryCandidateStatus extends Null {}

/** @name PalletAssetsRegistryForeignMetadata */
export interface PalletAssetsRegistryForeignMetadata extends Null {}

/** @name PalletCollatorSelectionCandidateInfo */
export interface PalletCollatorSelectionCandidateInfo extends Null {}

/** @name PalletCrowdloanRewardsModelsProof */
export interface PalletCrowdloanRewardsModelsProof extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: ITuple<[AccountId32, MultiSignature]>;
  readonly isEthereum: boolean;
  readonly asEthereum: EcdsaSignature;
  readonly type: 'RelayChain' | 'Ethereum';
}

/** @name PalletCrowdloanRewardsModelsRemoteAccount */
export interface PalletCrowdloanRewardsModelsRemoteAccount extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: AccountId32;
  readonly isEthereum: boolean;
  readonly asEthereum: EthereumAccountId;
  readonly type: 'RelayChain' | 'Ethereum';
}

/** @name PalletCrowdloanRewardsModelsReward */
export interface PalletCrowdloanRewardsModelsReward extends Null {}

/** @name PalletCrowdloanRewardsReward */
export interface PalletCrowdloanRewardsReward extends Null {}

/** @name PalletCurrencyFactoryRanges */
export interface PalletCurrencyFactoryRanges extends Null {}

/** @name PalletCurrencyFactoryRangesRange */
export interface PalletCurrencyFactoryRangesRange extends Null {}

/** @name PalletDemocracyConviction */
export interface PalletDemocracyConviction extends Null {}

/** @name PalletDemocracyPreimageStatus */
export interface PalletDemocracyPreimageStatus extends Null {}

/** @name PalletDemocracyReferendumInfo */
export interface PalletDemocracyReferendumInfo extends Null {}

/** @name PalletDemocracyReleases */
export interface PalletDemocracyReleases extends Null {}

/** @name PalletDemocracyVoteAccountVote */
export interface PalletDemocracyVoteAccountVote extends Null {}

/** @name PalletDemocracyVoteThreshold */
export interface PalletDemocracyVoteThreshold extends Null {}

/** @name PalletDemocracyVoteVoting */
export interface PalletDemocracyVoteVoting extends Null {}

/** @name PalletDutchAuctionSellOrder */
export interface PalletDutchAuctionSellOrder extends Null {}

/** @name PalletDutchAuctionTakeOrder */
export interface PalletDutchAuctionTakeOrder extends Null {}

/** @name PalletIdentityBitFlags */
export interface PalletIdentityBitFlags extends Null {}

/** @name PalletIdentityIdentityInfo */
export interface PalletIdentityIdentityInfo extends Null {}

/** @name PalletIdentityJudgement */
export interface PalletIdentityJudgement extends Null {}

/** @name PalletIdentityRegistrarInfo */
export interface PalletIdentityRegistrarInfo extends Null {}

/** @name PalletIdentityRegistration */
export interface PalletIdentityRegistration extends Null {}

/** @name PalletLiquidationsLiquidationStrategyConfiguration */
export interface PalletLiquidationsLiquidationStrategyConfiguration extends Enum {
  readonly isDutchAuction: boolean;
  readonly asDutchAuction: ComposableTraitsTimeTimeReleaseFunction;
  readonly isUniswapV2: boolean;
  readonly asUniswapV2: {
    readonly slippage: Perquintill;
  } & Struct;
  readonly isXcmDex: boolean;
  readonly asXcmDex: {
    readonly parachainId: u32;
  } & Struct;
  readonly type: 'DutchAuction' | 'UniswapV2' | 'XcmDex';
}

/** @name PalletMosaicAssetInfo */
export interface PalletMosaicAssetInfo extends Null {}

/** @name PalletMosaicDecayBudgetPenaltyDecayer */
export interface PalletMosaicDecayBudgetPenaltyDecayer extends Null {}

/** @name PalletMosaicNetworkInfo */
export interface PalletMosaicNetworkInfo extends Struct {
  readonly enabled: bool;
  readonly maxTransferSize: Balance;
}

/** @name PalletMosaicRelayerStaleRelayer */
export interface PalletMosaicRelayerStaleRelayer extends Null {}

/** @name PalletOracleAssetInfo */
export interface PalletOracleAssetInfo extends Null {}

/** @name PalletOraclePrePrice */
export interface PalletOraclePrePrice extends Null {}

/** @name PalletOraclePrice */
export interface PalletOraclePrice extends Null {}

/** @name PalletOracleWithdraw */
export interface PalletOracleWithdraw extends Struct {
  readonly stake: u128;
  readonly unlockBlock: u32;
}

/** @name PalletPreimageRequestStatus */
export interface PalletPreimageRequestStatus extends Null {}

/** @name PalletSchedulerReleases */
export interface PalletSchedulerReleases extends Null {}

/** @name PalletSchedulerScheduledV2 */
export interface PalletSchedulerScheduledV2 extends Null {}

/** @name PalletSchedulerScheduledV3 */
export interface PalletSchedulerScheduledV3 extends Null {}

/** @name PalletTreasuryProposal */
export interface PalletTreasuryProposal extends Null {}

/** @name PalletVaultModelsStrategyOverview */
export interface PalletVaultModelsStrategyOverview extends Null {}

/** @name PalletVaultModelsVaultInfo */
export interface PalletVaultModelsVaultInfo extends Null {}

/** @name PolkadotParachainPrimitivesXcmpMessageFormat */
export interface PolkadotParachainPrimitivesXcmpMessageFormat extends Null {}

/** @name PolkadotPrimitivesV1AbridgedHostConfiguration */
export interface PolkadotPrimitivesV1AbridgedHostConfiguration extends Null {}

/** @name PolkadotPrimitivesV1PersistedValidationData */
export interface PolkadotPrimitivesV1PersistedValidationData extends PersistedValidationData {}

/** @name SpConsensusAuraSr25519AppSr25519Public */
export interface SpConsensusAuraSr25519AppSr25519Public extends Null {}

/** @name XcmVersionedMultiAsset */
export interface XcmVersionedMultiAsset extends Null {}

export type PHANTOM_CROWDLOANREWARDS = 'crowdloanRewards';
