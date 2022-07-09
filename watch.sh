cargo watch --why --clear --ignore 'dist/**' -w server -w src -w index.html -w static -s 'trunk build && npx --prefer-offline tailwindcss --output dist/tailwind.css && cargo run -p server'
