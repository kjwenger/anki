// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

import { browser } from "$app/environment";
import { derived, writable } from "svelte/store";

export interface Collection {
    path: string;
    name: string;
}

export interface CollectionState {
    collections: Collection[];
    currentCollection: Collection | null;
}

const CURRENT_COLLECTION_KEY = "anki_current_collection";

function createCollectionStore() {
    const initialState: CollectionState = {
        collections: [],
        currentCollection: null,
    };

    // Load current collection from localStorage
    if (browser) {
        const storedCollection = localStorage.getItem(CURRENT_COLLECTION_KEY);
        if (storedCollection) {
            try {
                initialState.currentCollection = JSON.parse(storedCollection);
            } catch (e) {
                console.error("Failed to parse stored collection:", e);
                localStorage.removeItem(CURRENT_COLLECTION_KEY);
            }
        }
    }

    const { subscribe, set, update } = writable<CollectionState>(initialState);

    return {
        subscribe,
        setCollections: (collections: Collection[]) => {
            update((state) => ({ ...state, collections }));
        },
        selectCollection: (collection: Collection) => {
            if (browser) {
                localStorage.setItem(CURRENT_COLLECTION_KEY, JSON.stringify(collection));
            }
            update((state) => ({ ...state, currentCollection: collection }));
        },
        clearSelection: () => {
            if (browser) {
                localStorage.removeItem(CURRENT_COLLECTION_KEY);
            }
            update((state) => ({ ...state, currentCollection: null }));
        },
        addCollection: (collection: Collection) => {
            update((state) => ({
                ...state,
                collections: [...state.collections, collection],
            }));
        },
        removeCollection: (path: string) => {
            update((state) => ({
                ...state,
                collections: state.collections.filter((c) => c.path !== path),
                currentCollection: state.currentCollection?.path === path
                    ? null
                    : state.currentCollection,
            }));
        },
    };
}

export const collectionStore = createCollectionStore();

export const currentCollection = derived(
    collectionStore,
    ($store) => $store.currentCollection,
);

export const collections = derived(
    collectionStore,
    ($store) => $store.collections,
);
