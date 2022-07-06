# Gaia 🌳

> Opinionated full-stack rust web framework

## Components

- Route-aware SSR for first page load
- SPA

## TODO:

- [x] Yew router method that takes in a Routable and matches recognized routes
- [x] tailwind batteries included
  - it was pretty easy to use `npx tailwindcss` with a couple config edits
- [x] nested Html vnodes as component properties
  - use `yew::Children`
  - still doesn't work with SSR :( -- oh it was just because of \<p> nested in \<p>
- [ ] dynamic styles (stylist _should_ work but might need to enforce inlining for SSR/CSR determinism)
