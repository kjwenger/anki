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
}

export interface EditorState {
    deckId: number | null;
    notetypeId: number | null;
    notetype: Notetype | null;
    fields: string[];
    tags: string[];
    editingNoteId: number | null;
}

const initialState: EditorState = {
    deckId: null,
    notetypeId: null,
    notetype: null,
    fields: [],
    tags: [],
    editingNoteId: null,
};

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
            update((state) => ({
                ...state,
                notetypeId,
                notetype,
                fields: new Array(notetype.fields.length).fill(""),
            })),
        setField: (index: number, value: string) =>
            update((state) => {
                const fields = [...state.fields];
                fields[index] = value;
                return {
                    ...state,
                    fields,
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
            update((state) => ({
                ...state,
                fields: new Array(state.notetype?.fields.length || 0).fill(""),
                tags: [],
                editingNoteId: null,
            })),
    };
}

export const editorStore = createEditorStore();
