/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs,css}"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["cupcake", "business"],
  },
};
