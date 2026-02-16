// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

export function load({ url }: { url: URL }) {
    const deckId = parseInt(url.searchParams.get("deck") || "1");
    return {
        deckId,
    };
}
