import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ url }) => {
    const deckIdStr = url.searchParams.get('deck');
    if (!deckIdStr) {
        throw error(400, 'Missing deck ID');
    }

    const deckId = parseInt(deckIdStr);
    if (isNaN(deckId)) {
        throw error(400, 'Invalid deck ID');
    }

    return {
        deckId
    };
};
