// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

import { writable, derived } from "svelte/store";
import { browser } from "$app/environment";

export interface User {
    id: number;
    username: string;
    email: string;
}

export interface AuthState {
    user: User | null;
    token: string | null;
    isAuthenticated: boolean;
}

const TOKEN_KEY = "anki_auth_token";
const USER_KEY = "anki_user";

function createAuthStore() {
    const initialState: AuthState = {
        user: null,
        token: null,
        isAuthenticated: false,
    };

    // Load from localStorage if in browser
    if (browser) {
        const storedToken = localStorage.getItem(TOKEN_KEY);
        const storedUser = localStorage.getItem(USER_KEY);
        
        if (storedToken && storedUser) {
            try {
                initialState.token = storedToken;
                initialState.user = JSON.parse(storedUser);
                initialState.isAuthenticated = true;
            } catch (e) {
                console.error("Failed to parse stored user data:", e);
                localStorage.removeItem(TOKEN_KEY);
                localStorage.removeItem(USER_KEY);
            }
        }
    }

    const { subscribe, set, update } = writable<AuthState>(initialState);

    return {
        subscribe,
        login: (user: User, token: string) => {
            if (browser) {
                localStorage.setItem(TOKEN_KEY, token);
                localStorage.setItem(USER_KEY, JSON.stringify(user));
            }
            set({
                user,
                token,
                isAuthenticated: true,
            });
        },
        logout: () => {
            if (browser) {
                localStorage.removeItem(TOKEN_KEY);
                localStorage.removeItem(USER_KEY);
            }
            set({
                user: null,
                token: null,
                isAuthenticated: false,
            });
        },
        updateUser: (user: User) => {
            if (browser) {
                localStorage.setItem(USER_KEY, JSON.stringify(user));
            }
            update((state) => ({ ...state, user }));
        },
    };
}

export const authStore = createAuthStore();

// Derived store for easy access to just the token
export const authToken = derived(authStore, ($auth) => $auth.token);

// Derived store for easy access to just the user
export const currentUser = derived(authStore, ($auth) => $auth.user);

// Derived store for authentication status
export const isAuthenticated = derived(authStore, ($auth) => $auth.isAuthenticated);
