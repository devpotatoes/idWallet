
**Documentation technique du projet**

- **Description**: Projet wallet d'identité numérique BBS+ / ZKP en Rust.

**Structure**
- **perfs/**: Crate Rust contenant des tests de performances (tests/*).
- **wallet/**: application UI (Tauri + Leptos).


**Scripts et commandes**
- **`tailwind`**: lance `npx @tailwindcss/cli -i ./src/app.css -o ./styles.css --watch` (watcher CSS Tailwind).

    ```powershell
    npm run tailwind
    ```

**Comment lancer les tests**

Pré-requis généraux:
- Rust toolchain installée (stable), `cargo` disponible.

Commandes utiles (depuis le dossier `perfs/tests/`)

```powershell
cargo test --test nom_fichier_test
```

Pour obtenir une sortie verbeuse

```powershell
cargo test --test nom_fichier_test -- --nocapture
```

**Dépendances**
- Rust: `pairing_crypto`, `serde`, `hex`, `tauri` (dans `src-tauri`), `leptos` (UI)
- Node: `tailwindcss`, `@tailwindcss/cli`, `daisyui`

**Lancer l'application après un clone depuis github**

Pré-requis généraux:
- Rust toolchain installée (stable), `cargo` disponible.
- Node.js

```powershell
cd ./idwallet/wallet/wallet
npm install
npm run tailwind

#Ouvrir un autre terminal

cd ./idwallet/wallet/wallet
cargo tauri dev
```

**Build le projet (obtenir le .exe)**

```powershell
cargo tauri build
```