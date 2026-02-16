/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./routes/webapp/**/*.{svelte,ts}",
        "./lib/webapp/**/*.{svelte,ts}",
    ],
    // All Tailwind utilities get !important to override Bootstrap/Anki
    // global styles that are inherited from the root layout.
    important: true,
    darkMode: "class",
    theme: {
        extend: {
            colors: {
                primary: {
                    50: "#eef2ff",
                    100: "#e0e7ff",
                    200: "#c7d2fe",
                    300: "#a5b4fc",
                    400: "#818cf8",
                    500: "#6366f1",
                    600: "#4f46e5",
                    700: "#4338ca",
                    800: "#3730a3",
                    900: "#312e81",
                },
            },
        },
    },
    plugins: [],
};
