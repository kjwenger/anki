// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

import { writable } from "svelte/store";

export interface Card {
    card_id: number;
    question_html: string;
    answer_html: string;
    css: string;
    counts: {
        new: number;
        learning: number;
        review: number;
    };
}

export interface ReviewerState {
    currentCard: Card | null;
    showingAnswer: boolean;
    finished: boolean;
    deckId: number | null;
    canUndo: boolean;
    canRedo: boolean;
}

const initialState: ReviewerState = {
    currentCard: null,
    showingAnswer: false,
    finished: false,
    deckId: null,
    canUndo: false,
    canRedo: false,
};

function createReviewerStore() {
    const { subscribe, set, update } = writable<ReviewerState>(initialState);

    return {
        subscribe,
        setCard: (card: Card | null, finished: boolean) =>
            update((state) => ({
                ...state,
                currentCard: card,
                showingAnswer: false,
                finished,
            })),
        showAnswer: () =>
            update((state) => ({
                ...state,
                showingAnswer: true,
            })),
        setDeckId: (deckId: number) =>
            update((state) => ({
                ...state,
                deckId,
            })),
        setUndoRedo: (canUndo: boolean, canRedo: boolean) =>
            update((state) => ({
                ...state,
                canUndo,
                canRedo,
            })),
        reset: () => set(initialState),
    };
}

export const reviewerStore = createReviewerStore();
