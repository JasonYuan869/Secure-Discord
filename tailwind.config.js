/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./*.html",
    "./src/**/*.{js,ts,svelte,html}",
  ],
  theme: {
    extend: {
      colors: {
        'discord-blue': '#7289da',
        'discord-dark': {
          100: '#1e2124',
          200: '#282b30',
          300: '#36393e',
          400: '#424549',
        },
        'discord-text': 'rgb(242, 243, 245)',
      }
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["dark"],
  },
};

