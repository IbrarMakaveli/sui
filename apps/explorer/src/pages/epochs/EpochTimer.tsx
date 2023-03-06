// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';

import { getMockEpochData } from './mocks';
import { useEpochProgress } from './useEpochProgress';

import { useGetCurrentEpochStaticInfo } from '~/hooks/useGetObject';
import { ProgressCircle } from '~/ui/ProgressCircle';
import { Text } from '~/ui/Text';

export function EpochTimer() {
    const { data: currentEpoch } = useGetCurrentEpochStaticInfo();
    // todo: replace this when we have real epoch end times
    const { data: mockEpochData } = useQuery(
        ['epoch', 'current'],
        async () => await getMockEpochData()
    );
    const { progress, label } = useEpochProgress(
        currentEpoch?.epoch_start_timestamp_ms,
        mockEpochData?.endTimestamp
    );

    return (
        <div className="shadow-notification flex w-fit items-center gap-1.5 rounded-lg border border-gray-45 py-2 px-2.5">
            <div className="w-5 text-steel-darker">
                <ProgressCircle progress={progress} />
            </div>
            <Text variant="p2/medium" color="steel-darker">
                Epoch {currentEpoch?.epoch} in progress. {label}
            </Text>
        </div>
    );
}
