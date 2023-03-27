/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
    colors: {
      appGreen: "#1DB954",
      appWhite: "#FFFFFF",
      appBlack: "#191414",
      white: "#FFFFFF",
    },
  },
  daisyui: {
    themes: ["forest"],
  },
  plugins: [require("daisyui")],
};
