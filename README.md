<div align="center">
  <img src="src-tauri/icons/icon.png" width="128" height="128" alt="Claket Logo - High-Performance Native Soundboard for Streamers and Gamers">
  <h1>Claket Soundboard</h1>
  <p><strong>A high-performance, native low-latency soundboard for streamers, gamers, and power users.</strong></p>

  <p>
    <a href="https://v2.tauri.app/"><img src="https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri&logoColor=white" alt="Tauri v2"></a>
    <a href="https://vuejs.org/"><img src="https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vuedotjs&logoColor=white" alt="Vue 3"></a>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.75+-000000?logo=rust&logoColor=white" alt="Rust"></a>
    <a href="https://www.typescriptlang.org/"><img src="https://img.shields.io/badge/TypeScript-5.6-3178C6?logo=typescript&logoColor=white" alt="TypeScript"></a>
    <a href="https://tailwindcss.com/"><img src="https://img.shields.io/badge/Tailwind-v4-06B6D4?logo=tailwindcss&logoColor=white" alt="Tailwind CSS v4"></a>
    <a href="https://bun.sh/"><img src="https://img.shields.io/badge/Bun-1.1-fbf0df?logo=bun&logoColor=000" alt="Bun"></a>
  </p>
</div>

---

## ⚡ Professional-Grade Audio for Your Stream

**Claket** is a modern, streamer-ready soundboard application engineered for **zero-latency** performance using **Tauri v2** and **Rust**. Whether you are a Twitch streamer, a competitive gamer, or a podcast host, Claket provides the precision and reliability needed for high-stakes audio production.

Unlike standard browser-based soundboards, Claket utilizes a **native Rust audio engine** (powered by `rodio` and `cpal`). This allows it to bypass Web Audio API limitations, offering **bit-perfect audio routing** directly to specific hardware outputs or virtual audio mixers like **VoiceMeeter**, **VB-Audio Cable**, and **OBS Studio**.

## ✨ Key Features

- 🎧 **Native Output Routing**: Send audio directly to any hardware device or virtual cable with precision.
- 🚀 **Zero-Latency Rust Engine**: Native audio processing ensures instant playback with no lag.
- 💾 **Local Library Mirroring**: Automatically copies imported sounds to a dedicated AppData folder—your library stays intact even if original files move.
- 🎨 **Modern Streamer UI**: A sleek, dark-mode-first interface built with Tailwind CSS v4 and shadcn-vue.
- 🖱️ **Instant Grid Mapping**: Simply drag and drop audio files onto the grid to create custom sound triggers.
- 🎚️ **Dynamic Volume Control**: Per-button normalization and a global Master Volume with real-time level meters.
- 🔄 **Persistent Settings**: Your custom grids, device preferences, and volume levels are saved automatically.

## 🎯 Why Claket? (Use Cases)

### 🎙️ Streamer-Ready Soundboard
Route Claket through a virtual audio cable (VAC) into **OBS Studio** or **Streamlabs**. Keep your soundboard audio on a separate track from your game audio for perfect mix control.

### 🎮 Gaming & Discord
Trigger funny clips or memes during matches. By routing Claket to a virtual input, your sounds will play directly into **Discord**, **Teamspeak**, or in-game voice chats.

### 🎛️ VoiceMeeter Integration
Optimized for power users. Seamlessly integrate Claket as a hardware input in **VoiceMeeter Banana/Potato** to manage your sound effects alongside your microphone and music.

### 📻 Podcast Production
Use the multi-page grid to organize sound effects, intro music, and transitions. The low-latency engine ensures your timing is always perfect.

## 🛠️ Tech Stack

| Layer | Technology |
| :--- | :--- |
| **App Framework** | [Tauri v2](https://v2.tauri.app/) (Native Performance) |
| **Frontend** | [Vue 3](https://vuejs.org/) (Composition API) |
| **Backend** | [Rust](https://www.rust-lang.org/) (Low Latency Engine) |
| **Styling** | [Tailwind CSS v4](https://tailwindcss.com/), [shadcn-vue](https://www.shadcn-vue.com/) |
| **Audio Library** | [Rodio](https://github.com/RustAudio/rodio) & [CPAL](https://github.com/RustAudio/cpal) |
| **Package Manager** | [Bun](https://bun.sh/) |

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Bun](https://bun.sh/)
- [Tauri Prerequisites](https://v2.tauri.app/guides/start/prerequisites/)

### Installation

1. Clone the repository:
   ```bash
    git clone https://github.com/aera128/claket-tauri.git
   cd claket-tauri
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Run in development mode:
   ```bash
   bun run dev
   ```

### Building for Production

To create a native installer for your platform:
```bash
bun run build
```
*Artifacts will be available in `src-tauri/target/release/bundle`.*

## 📖 Under the Hood

1. **Mirroring**: Claket copies imported audio to `%APPDATA%/com.sk4ur.claket/sounds` (on Windows) to ensure persistence.
2. **Decoding**: Audio is pre-decoded using `rodio` and cached in memory for zero-delay triggers.
3. **Routing**: The backend uses `cpal` to interface directly with the OS audio stack, enabling multi-device selection.

## 🤝 Contributing

Contributions are welcome! If you have ideas for streamer integrations or audio improvements, feel free to open an issue or pull request.

---

<div align="center">
  Built with ❤️ for the streaming community by <a href="https://github.com/aera128">sk4ur</a>
</div>
