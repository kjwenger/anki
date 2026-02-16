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
            const authState = get(authStore);
            console.log("Auth state:", authState);
            if (authState?.token) {
                headers["Authorization"] = `Bearer ${authState.token}`;
            } else {
                console.warn("No auth token available!");
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
        console.log("=== API client.login ===");
        console.log("Endpoint: /api/v1/auth/login");
        console.log("Username:", username);
        console.log("Sending POST request...");
        
        const result = await this.post<{ 
            token: string; 
            user: { id: number; username: string; email: string };
            success?: boolean;
            data?: any;
            error?: any;
        }>(
            "/api/v1/auth/login",
            { username, password },
            false,
        );
        
        console.log("Raw API response:", result);
        
        // Handle different response formats
        if (result.success && result.data) {
            console.log("Response has success/data format");
            return result.data;
        } else if (result.token && result.user) {
            console.log("Response has direct token/user format");
            return result;
        } else {
            console.error("Unexpected response format:", result);
            throw new Error("Invalid login response format");
        }
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
        console.log("=== API client.renameDeck ===");
        console.log("Deck ID:", id);
        console.log("New name:", name);
        console.log("Endpoint:", `/api/v1/decks/${id}`);
        console.log("Request body:", { name });
        const result = await this.put<{ message: string }>(`/api/v1/decks/${id}`, { name });
        console.log("Rename API response:", result);
        return result;
    }

    async deleteDeck(id: number) {
        return this.delete<{ message: string }>(`/api/v1/decks/${id}`);
    }

    // Notetype endpoints
    async getNotetypes() {
        return this.get<{
            notetypes: Array<{
                id: number;
                name: string;
            }>;
        }>("/api/v1/notetypes");
    }

    async getNotetype(id: number) {
        return this.get<{
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
        }>(`/api/v1/notetypes/${id}`);
    }

    // Notes endpoints
    async createNote(
        deckId: number,
        notetypeId: number,
        fields: string[],
        tags: string[],
    ) {
        return this.post<{
            success: boolean;
            note_id: number;
            message: string;
        }>("/api/v1/notes", {
            deck_id: deckId,
            notetype_id: notetypeId,
            fields,
            tags,
        });
    }

    async getNote(id: number) {
        return this.get<{
            id: number;
            fields: string[];
            tags: string[];
        }>(`/api/v1/notes/${id}`);
    }

    async updateNote(id: number, fields: string[], tags: string[]) {
        return this.put<{ success: boolean; message: string }>(
            `/api/v1/notes/${id}`,
            { fields, tags },
        );
    }

    // Card endpoints
    async getCard(id: number) {
        return this.get<{
            id: number;
            note_id: number;
            deck_id: number;
            template_idx: number;
            queue: number;
            due: number;
            interval: number;
            ease_factor: number;
            reps: number;
            lapses: number;
            remaining_steps: number;
            original_deck_id: number;
        }>(`/api/v1/cards/${id}`);
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

    // Search endpoints
    async searchCards(query: string, sortColumn?: string, reverse?: boolean) {
        return this.post<{
            card_ids: number[];
            count: number;
        }>("/api/v1/search/cards", {
            query,
            sort_column: sortColumn,
            reverse: reverse || false,
        });
    }

    async searchNotes(query: string, sortColumn?: string, reverse?: boolean) {
        return this.post<{
            note_ids: number[];
            count: number;
        }>("/api/v1/search/notes", {
            query,
            sort_column: sortColumn,
            reverse: reverse || false,
        });
    }

    // Statistics endpoints
    async getTodayStats() {
        return this.get<{
            answer_count: number;
            answer_millis: number;
            correct_count: number;
            mature_correct: number;
            mature_count: number;
            learn_count: number;
            review_count: number;
            relearn_count: number;
            early_review_count: number;
        }>("/api/v1/stats/today");
    }

    async getCollectionStats() {
        return this.get<{
            total_cards: number;
            new_cards: number;
            young_cards: number;
            mature_cards: number;
            suspended_cards: number;
            buried_cards: number;
            total_notes: number;
        }>("/api/v1/stats/collection");
    }

    async getGraphs(search?: string, days?: number) {
        const params = new URLSearchParams();
        if (search) params.append("search", search);
        if (days) params.append("days", days.toString());
        const query = params.toString() ? `?${params.toString()}` : "";
        return this.get<any>(`/api/v1/stats/graphs${query}`);
    }
}

export const api = new ApiClient();
