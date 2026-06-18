/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
    "./docs/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        slate: {
          950: '#020617',
          900: '#0f172a',
          800: '#1e293b',
          700: '#334155',
          600: '#475569',
          500: '#64748b',
          400: '#94a3b8',
          300: '#cbd5e1',
          200: '#e2e8f0',
          100: '#f1f5f9',
          50: '#f8fafc',
        },
        gray: {
          800: '#1f2937',
          500: '#6b7280',
          400: '#9ca3af',
          200: '#e5e7eb',
          100: '#f3f4f6',
          50: '#f9fafb',
        },
        blue: {
          600: '#2563eb',
          500: '#3b82f6',
          400: '#60a5fa',
        },
        green: {
          600: '#16a34a',
          500: '#22c55e',
          400: '#4ade80',
          700: '#15803d',
        },
        yellow: {
          600: '#ca8a04',
          500: '#f59e0b',
          400: '#fbbf24',
        },
        red: {
          600: '#dc2626',
          500: '#ef4444',
          400: '#f87171',
        },
      },
      boxShadow: {
        card: '0 1px 3px 0 rgb(0 0 0 / 0.05), 0 1px 2px -1px rgb(0 0 0 / 0.04)',
      },
    },
  },
  plugins: [],
}
