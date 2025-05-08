# TactiClip

[![Rust](https://img.shields.io/badge/Rust-1.86.0-black?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)  [![Tauri](https://img.shields.io/badge/Tauri-v2.0-blue?style=flat-square&logo=tauri)](https://tauri.app/)  [![Qwik](https://img.shields.io/badge/Qwik-1.13-lightgrey?style=flat-square&logo=qwik)](https://qwik.builder.io/)  [![Vite](https://img.shields.io/badge/Vite-5.3.5-yellow?style=flat-square&logo=vite&logoColor=black)](https://vitejs.dev/)  [![Flowbite](https://img.shields.io/badge/Flowbite-1.0-blue?style=flat-square&logo=flowbite)](https://flowbite.com/)  [![SQLite](https://img.shields.io/badge/SQLite-3.40-lightgrey?style=flat-square&logo=sqlite&logoColor=4479A1)](https://sqlite.org/)  [![Node](https://img.shields.io/badge/Node.js-22.11.0-green?style=flat-square&logo=node.js&logoColor=white)](https://nodejs.org/) 


A lightweight, cross-platform clipboard manager built with Tauri V2, Qwik, Vite, and Flowbite — backed by a SQLite database. 
Easily browse, search, and organize your clipboard history with a snappy native-like UI.

![](src-tauri/icons/icon.png)

---

## 🚀 Features

- **Persistent Clipboard History**  
  Automatically capture and store all clipboard entries in a local SQLite database.

- **Never lose important items**  
  One-click pinning to keep important clipboard entries at the top.

- **Easy to manage**  
  Easy configuration and management of clipboard history.

- **Cross-Platform**  
  Runs on Windows, macOS, and Linux via Tauri V2’s native shell.

- **Lightweight & Secure**  
  No external server—your data stays on your machine, encrypted at rest.

## 🕰️ Coming soon

- **Search & Filter**  
  Quickly find specific clipboard entries with a powerful search feature (text, type of entry, code language).

- **Customizable UI**
  Extension of the customizable UI with custom CSS variables (and custom CSS injection ?).

- **Leveled-Up productivity**  
  Quick actions and for some special clipboard entries (links, emails, colors, etc.).

- **File support**  
  Keep a track of files copied to the clipboard (images, videos, etc.).

- **RTF Support**  
  Support for rich text format (RTF) clipboard entries, might not be possible on Windows after a few tests.

- **Multi-device sync**  
  Sync clipboard history across multiple devices using a secure cloud service (may be with a premium plan or a self-hosted server instance).

---

## 🛠 Tech Stack

- **Rust & Tauri V2** — Native backend & windowing  
- **Qwik** — Ultra-fast, resumable frontend framework  
- **Vite** — Lightning-fast build tool  
- **Flowbite** — Tailwind-powered UI components  
- **SQLite** — Embedded, zero-configuration database  

---

## 🔧 Prerequisites

- [Node.js ≥ 18.x](https://nodejs.org/)  
- [Rust toolchain ≥ 1.70](https://www.rust-lang.org/tools/install)  
- [Tauri CLI V2](https://tauri.app/v2/)  
  ```bash
  cargo install tauri-cli --version "^2.0"

---

## ⚙️ Installation

1. **Clone the repo**

   ```bash
   git clone https://github.com/PetchouDev/tacticlip.git
   cd tacticlip
   ```

2. **Install frontend dependencies**

   ```bash
   npm install
   # or
   yarn install
   ```

## ▶️ Running in Development

```bash
# Start the Vite dev server and Tauri concurrently
npx tauri dev
```

* The **Qwik** app is served via Vite on `http://localhost:5173`.
* On load, Tauri V2 will open a native window pointing to that address.

---

## 🧩 Qwik & Tauri Integration note

Because the Tauri API (V2) is only available client-side in a desktop context, **SSR is disabled**. Instead of `useTask$`, you should use Qwik’s `useVisibleTask$` to ensure code runs after hydration:

```tsx
import { useVisibleTask$ } from '@builder.io/qwik';
import { clipboard } from '@tauri-apps/api';

export const ClipboardListener = component$(() => {
  useVisibleTask$(async () => {
    clipboard.readText().then((text) => {
      // save to SQLite via your Rust command
      invoke('save_clip', { content: text });
    });
  });

  return <div>Listening for clipboard changes…</div>;
});
```

---

## 📦 Build for Production

```bash
npm run build       # builds the Qwik frontend into `dist/`
npm run tauri build # bundles Tauri app into native installer
```

Installers for Windows, macOS, and Linux appear under `src-tauri/target/release/bundle/`.

---

## 🧑‍🤝‍🧑 Contributing

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Commit your changes: `git commit -m "feat: add awesome feature"`
4. Push to your branch: `git push origin feat/your-feature`
5. Open a Pull Request

Please follow the existing code style and write Qwik components with `*.tsx` and Tailwind class names. You are free to rename elements and refactor the code as long as it makes more sense and do not break the existing functionalities.

---

## 📜 License

Distributed under the BSD 4-Clauses License. See [LICENSE](LICENSE) for details.

---

## 🙏 Thanks

TactiClip is inspired by the need for a reliable, native clipboard manager without the bloat. Built with ❤️ using [Tauri](https://github.com/tauri-apps/tauri) and [Qwik](https://github.com/QwikDev/qwik).


