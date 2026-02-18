// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

import { writable } from "svelte/store";

export interface Notetype {
    id: number;
    name: string;
    fields: Array<{
        name: string;
        ord: number;
    }>;
    templates: Array<{
        name: string;
        ord: number;
    }>;
    is_cloze: boolean;
}

export interface EditorState {
    deckId: number | null;
    notetypeId: number | null;
    notetype: Notetype | null;
    fields: string[];
    tags: string[];
    stickyFields: boolean[];
    editingNoteId: number | null;
}

const initialState: EditorState = {
    deckId: null,
    notetypeId: null,
    notetype: null,
    fields: [],
    tags: [],
    stickyFields: [],
    editingNoteId: null,
};

function loadStickyFields(notetypeId: number): boolean[] {
    try {
        const saved = localStorage.getItem(`anki_sticky_${notetypeId}`);
        return saved ? JSON.parse(saved) : [];
    } catch {
        return [];
    }
}

function saveStickyFields(notetypeId: number, stickyFields: boolean[]) {
    try {
        localStorage.setItem(`anki_sticky_${notetypeId}`, JSON.stringify(stickyFields));
    } catch (e) {
        console.error("Failed to save sticky fields:", e);
    }
}

function createEditorStore() {
    const { subscribe, set, update } = writable<EditorState>(initialState);

    return {
        subscribe,
        setDeck: (deckId: number) =>
            update((state) => ({
                ...state,
                deckId,
            })),
        setNotetype: (notetypeId: number, notetype: Notetype) =>
            update((state) => {
                const stickyFields = loadStickyFields(notetypeId);
                // Ensure stickyFields array matches notetype fields length
                const adjustedSticky = new Array(notetype.fields.length).fill(false);
                for (let i = 0; i < Math.min(stickyFields.length, adjustedSticky.length); i++) {
                    adjustedSticky[i] = stickyFields[i];
                }

                return {
                    ...state,
                    notetypeId,
                    notetype,
                    fields: new Array(notetype.fields.length).fill(""),
                    stickyFields: adjustedSticky,
                };
            }),
        setField: (index: number, value: string) =>
            update((state) => {
                const fields = [...state.fields];
                fields[index] = value;
                return {
                    ...state,
                    fields,
                };
            }),
        setSticky: (index: number, sticky: boolean) =>
            update((state) => {
                if (!state.notetypeId) return state;
                const stickyFields = [...state.stickyFields];
                stickyFields[index] = sticky;
                saveStickyFields(state.notetypeId, stickyFields);
                return {
                    ...state,
                    stickyFields,
                };
            }),
        setTags: (tags: string[]) =>
            update((state) => ({
                ...state,
                tags,
            })),
        loadNote: (noteId: number, fields: string[], tags: string[]) =>
            update((state) => ({
                ...state,
                editingNoteId: noteId,
                fields,
                tags,
            })),
        reset: () => set(initialState),
        resetFields: () =>
            update((state) => {
                const newFields = new Array(state.notetype?.fields.length || 0).fill("");
                // Retain values of sticky fields
                if (state.notetype) {
                    for (let i = 0; i < newFields.length; i++) {
                        if (state.stickyFields[i]) {
                            newFields[i] = state.fields[i];
                        }
                    }
                }

                return {
                    ...state,
                    fields: newFields,
                    tags: [],
                    editingNoteId: null,
                };
            }),
    };
}

export const editorStore = createEditorStore();
