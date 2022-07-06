cargo watch -w $PWD -s 'trunk build && npx --prefer-offline tailwindcss --output dist/tailwind.css && cargo run -p server'
