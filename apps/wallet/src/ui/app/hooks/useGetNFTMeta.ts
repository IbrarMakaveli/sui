// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { processDisplay } from '@mysten/core';
import { is, SuiObjectData } from '@mysten/sui.js';
import { useMemo } from 'react';

import { useGetObject } from './useGetObject';

export type NFTMetadata = {
    name: string | null;
    description: string | null;
    url: string;
};

export function useGetNFTMeta(objectID: string) {
    const data = useGetObject(objectID, { showDisplay: true });
    const nftMeta = useMemo(() => {
        if (!data.data) return null;
        const { details } = data.data || {};
        if (!is(details, SuiObjectData) || !details.display) return null;
        return processDisplay(details.display);
    }, [data]);

    return {
        ...data,
        data: nftMeta,
    };
}
