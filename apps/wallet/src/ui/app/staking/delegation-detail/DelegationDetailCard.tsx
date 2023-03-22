// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFeature } from '@growthbook/growthbook-react';
import { calculateAPY } from '@mysten/core';
import { useMemo } from 'react';

import { useActiveAddress } from '../../hooks/useActiveAddress';
import { Heading } from '../../shared/heading';
import { getDelegationDataByStakeId } from '../getDelegationByStakeId';
import { StakeAmount } from '../home/StakeAmount';
import { useGetDelegatedStake } from '../useGetDelegatedStake';
import { useSystemState } from '../useSystemState';
import BottomMenuLayout, { Content } from '_app/shared/bottom-menu-layout';
import Button from '_app/shared/button';
import { Card } from '_app/shared/card';
import { CardItem } from '_app/shared/card/CardItem';
import { Text } from '_app/shared/text';
import { IconTooltip } from '_app/shared/tooltip';
import Alert from '_components/alert';
import Icon, { SuiIcons } from '_components/icon';
import LoadingIndicator from '_components/loading/LoadingIndicator';
import { useGetInactiveValidators } from '_hooks';
import { FEATURES } from '_src/shared/experimentation/features';

type DelegationDetailCardProps = {
    validatorAddress: string;
    stakedId: string;
};

export function DelegationDetailCard({
    validatorAddress,
    stakedId,
}: DelegationDetailCardProps) {
    const {
        data: system,
        isLoading: loadingValidators,
        isError: errorValidators,
    } = useSystemState();

    const accountAddress = useActiveAddress();

    const {
        data: allDelegation,
        isLoading,
        isError,
    } = useGetDelegatedStake(accountAddress || '');

    const validatorData = useMemo(() => {
        if (!system) return null;
        return system.activeValidators.find(
            (av) => av.suiAddress === validatorAddress
        );
    }, [validatorAddress, system]);

    const delegationData = useMemo(() => {
        return allDelegation
            ? getDelegationDataByStakeId(allDelegation, stakedId)
            : null;
    }, [allDelegation, stakedId]);

    const totalStake = BigInt(delegationData?.principal || 0n);

    const suiEarned = BigInt(delegationData?.estimatedReward || 0n);

    const apy = useMemo(() => {
        if (!validatorData || !system) return 0;
        return calculateAPY(validatorData, +system.epoch);
    }, [validatorData, system]);

    const delegationId =
        delegationData?.status === 'Active' && delegationData?.stakedSuiId;

    const stakeByValidatorAddress = `/stake/new?${new URLSearchParams({
        address: validatorAddress,
        staked: stakedId,
    }).toString()}`;

    const { data: inActiveValidatorsIds } = useGetInactiveValidators(
        system?.inactivePoolsId
    );

    const inActiveValidator = inActiveValidatorsIds.length;
    const commission = validatorData ? +validatorData.commissionRate / 100 : 0;
    const stakingEnabled = useFeature(FEATURES.STAKING_ENABLED).on;

    if (isLoading || loadingValidators) {
        return (
            <div className="p-2 w-full flex justify-center items-center h-full">
                <LoadingIndicator />
            </div>
        );
    }

    if (isError || errorValidators) {
        return (
            <div className="p-2">
                <Alert mode="warning">
                    <div className="mb-1 font-semibold">
                        Something went wrong
                    </div>
                </Alert>
            </div>
        );
    }

    return (
        <div className="flex flex-col flex-nowrap flex-grow h-full">
            <BottomMenuLayout>
                <Content>
                    <div className="justify-center w-full flex flex-col items-center">
                        {inActiveValidator ? (
                            <Alert className="mb-3">
                                Unstake SUI from this inactive validator and
                                stake on an active validator to start earning
                                rewards again.
                            </Alert>
                        ) : null}
                        <div className="w-full flex">
                            <Card
                                header={
                                    <div className="grid grid-cols-2 divide-x divide-solid divide-gray-45 divide-y-0 w-full">
                                        <CardItem title="Your Stake">
                                            <StakeAmount
                                                balance={totalStake}
                                                variant="heading5"
                                            />
                                        </CardItem>

                                        <CardItem title="Earned">
                                            <StakeAmount
                                                balance={suiEarned}
                                                variant="heading5"
                                                isEarnedRewards
                                            />
                                        </CardItem>
                                    </div>
                                }
                                padding="none"
                            >
                                <div className="divide-x flex divide-solid divide-gray-45 divide-y-0">
                                    <CardItem
                                        title={
                                            <div className="flex text-steel-darker gap-1 items-start">
                                                APY
                                                <div className="text-steel">
                                                    <IconTooltip
                                                        tip="Annual Percentage Yield"
                                                        placement="top"
                                                    />
                                                </div>
                                            </div>
                                        }
                                    >
                                        <div className="flex gap-0.5 items-baseline">
                                            <Heading
                                                variant="heading4"
                                                weight="semibold"
                                                color="gray-90"
                                                leading="none"
                                            >
                                                {apy}
                                            </Heading>

                                            <Text
                                                variant="subtitleSmall"
                                                weight="medium"
                                                color="steel-dark"
                                            >
                                                %
                                            </Text>
                                        </div>
                                    </CardItem>

                                    <CardItem
                                        title={
                                            <div className="flex text-steel-darker gap-1">
                                                Commission
                                                <div className="text-steel">
                                                    <IconTooltip
                                                        tip="Validator commission"
                                                        placement="top"
                                                    />
                                                </div>
                                            </div>
                                        }
                                    >
                                        <div className="flex gap-0.5 items-baseline">
                                            <Heading
                                                variant="heading4"
                                                weight="semibold"
                                                color="gray-90"
                                                leading="none"
                                            >
                                                {commission}
                                            </Heading>

                                            <Text
                                                variant="subtitleSmall"
                                                weight="medium"
                                                color="steel-dark"
                                            >
                                                %
                                            </Text>
                                        </div>
                                    </CardItem>
                                </div>
                            </Card>
                        </div>
                        <div className="flex gap-2.5 w-full my-3.75">
                            {!inActiveValidator ? (
                                <Button
                                    size="large"
                                    mode="outline"
                                    href={stakeByValidatorAddress}
                                    className="border-bg-steel-dark border-solid w-full hover:border-bg-steel-darker"
                                    disabled={!stakingEnabled}
                                >
                                    <Icon
                                        icon={SuiIcons.Add}
                                        className="font-normal"
                                    />
                                    Stake SUI
                                </Button>
                            ) : null}
                            {Boolean(totalStake) && delegationId && (
                                <Button
                                    size="large"
                                    mode="outline"
                                    href={
                                        stakeByValidatorAddress +
                                        '&unstake=true'
                                    }
                                    className="border-bg-steel-dark border-solid w-full hover:border-bg-steel-darker"
                                >
                                    <Icon
                                        icon={SuiIcons.Remove}
                                        className="text-heading6 font-normal"
                                    />
                                    Unstake SUI
                                </Button>
                            )}
                        </div>
                    </div>
                </Content>
                <Button
                    size="large"
                    mode="neutral"
                    href="/stake"
                    className="!text-steel-darker"
                >
                    <Icon
                        icon={SuiIcons.ArrowLeft}
                        className="text-body text-gray-60 font-normal"
                    />
                    Back
                </Button>
            </BottomMenuLayout>
        </div>
    );
}
