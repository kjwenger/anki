// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

import { get } from "svelte/store";
import { authStore } from "../stores/auth";

const API_BASE_URL = import.meta.env.VITE_API_URL || "http://localhost:8080";

export interface ApiError {
    message: string;
    status: number;
}

export class ApiClient {
    private baseUrl: string;

    constructor(baseUrl: string = API_BASE_URL) {
        this.baseUrl = baseUrl;
    }

    private getHeaders(includeAuth: boolean = true): HeadersInit {
        const headers: HeadersInit = {
            "Content-Type": "application/json",
        };

        if (includeAuth) {
            const { token } = get(authStore);
            if (token) {
                headers["Authorization"] = `Bearer ${token}`;
            }
        }

        return headers;
    }

    private async handleResponse<T>(response: Response): Promise<T> {
        if (!response.ok) {
            const errorText = await response.text();
            let errorMessage = `HTTP ${response.status}: ${response.statusText}`;
            
            try {
                const errorData = JSON.parse(errorText);
                errorMessage = errorData.message || errorData.error || errorMessage;
            } catch {
                errorMessage = errorText || errorMessage;
            }

            throw {
                message: errorMessage,
                status: response.status,
            } as ApiError;
        }

        const text = await response.text();
        return text ? JSON.parse(text) : ({} as T);
    }

    async get<T>(endpoint: string, includeAuth: boolean = true): Promise<T> {
        const response = await fetch(`${this.baseUrl}${endpoint}`, {
            method: "GET",
            headers: this.getHeaders(includeAuth),
        });

        return this.handleResponse<T>(response);
    }

    async post<T>(
        endpoint: string,
        data?: unknown,
        includeAuth: boolean = true,
    ): Promise<T> {
        const response = await fetch(`${this.baseUrl}${endpoint}`, {
            method: "POST",
            headers: this.getHeaders(includeAuth),
            body: data ? JSON.stringify(data) : undefined,
        });

        return this.handleResponse<T>(response);
    }

    async put<T>(
        endpoint: string,
        data?: unknown,
        includeAuth: boolean = true,
    ): Promise<T> {
        const response = await fetch(`${this.baseUrl}${endpoint}`, {
            method: "PUT",
            headers: this.getHeaders(includeAuth),
            body: data ? JSON.stringify(data) : undefined,
        });

        return this.handleResponse<T>(response);
    }

    async delete<T>(endpoint: string, includeAuth: boolean = true): Promise<T> {
        const response = await fetch(`${this.baseUrl}${endpoint}`, {
            method: "DELETE",
            headers: this.getHeaders(includeAuth),
        });

        return this.handleResponse<T>(response);
    }

    // Authentication endpoints
    async login(username: string, password: string) {
        return this.post<{ token: string; user: { id: number; username: string; email: string } }>(
            "/api/v1/auth/login",
            { username, password },
            false,
        );
    }

    async register(username: string, email: string, password: string) {
        return this.post<{ message: string; user_id: number }>(
            "/api/v1/auth/register",
            { username, email, password },
            false,
        );
    }

    async logout() {
        return this.post<{ message: string }>("/api/v1/auth/logout");
    }

    async me() {
        return this.get<{ id: number; username: string; email: string }>(
            "/api/v1/auth/me",
        );
    }

    // Collection endpoints
    async getCollections() {
        return this.get<{ collections: Array<{ path: string; name: string }> }>(
            "/api/v1/collections",
        );
    }

    async createCollection(name: string) {
        return this.post<{ path: string; message: string }>(
            "/api/v1/collections",
            { name },
        );
    }

    async deleteCollection(path: string) {
        return this.delete<{ message: string }>(
            `/api/v1/collections/${encodeURIComponent(path)}`,
        );
    }

    // Deck endpoints
    async getDecks() {
        return this.get<{
            decks: Array<{
                id: number;
                name: string;
                new_count: number;
                learn_count: number;
                review_count: number;
            }>;
        }>("/api/v1/decks");
    }

    async createDeck(name: string) {
        return this.post<{ id: number; message: string }>(
            "/api/v1/decks",
            { name },
        );
    }

    async renameDeck(id: number, name: string) {
        return this.put<{ message: string }>(`/api/v1/decks/${id}`, { name });
    }

    async deleteDeck(id: number) {
        return this.delete<{ message: string }>(`/api/v1/decks/${id}`);
    }

    // Scheduler endpoints
    async getNextCard(deckId: number) {
        return this.get<{
            card: {
                card_id: number;
                question_html: string;
                answer_html: string;
                css: string;
                counts: {
                    new: number;
                    learning: number;
                    review: number;
                };
            } | null;
            finished: boolean;
        }>(`/api/v1/scheduler/decks/${deckId}/next`);
    }

    async answerCard(deckId: number, cardId: number, rating: number) {
        return this.post<{ success: boolean; message: string }>(
            `/api/v1/scheduler/decks/${deckId}/cards/${cardId}/answer`,
            { rating },
        );
    }

    async getDeckCounts(deckId: number) {
        return this.get<{
            new: number;
            learning: number;
            review: number;
        }>(`/api/v1/scheduler/decks/${deckId}/counts`);
    }

    async undo() {
        return this.post<{ success: boolean; message: string }>(
            "/api/v1/scheduler/undo",
        );
    }

    async redo() {
        return this.post<{ success: boolean; message: string }>(
            "/api/v1/scheduler/redo",
        );
    }
}

export const api = new ApiClient();
