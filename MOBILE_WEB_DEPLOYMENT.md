# Mobile and Web Deployment Guide

## Status: Web-Ready, Mobile via Web Browser

The FieldCalc interactive editor is built with **egui + wgpu**, which has excellent cross-platform support including:
- ✅ Native (Windows, macOS, Linux) - implemented
- ✅ WebAssembly (browser) - needs build configuration
- ⚠️ Mobile (iOS, Android) - best accessed via web browser

## Quick Start: Web Deployment (Recommended for Mobile Access)

### Why Web-First for Mobile?
- **No app stores**: Access via browser immediately
- **Cross-platform**: Works on iOS, Android, desktop browsers
- **Easy updates**: Just deploy new WASM, no app updates
- **Smaller bundle**: ~2-5MB vs native app overhead
- **egui already supports it**: Zero code changes needed

### Build for Web (WASM)

#### 1. Install wasm-pack
```bash
cargo install wasm-pack
```

#### 2. Add wasm-bindgen dependency
```toml
# Cargo.toml
[dependencies]
wasm-bindgen = "0.2"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["console"] }
```

#### 3. Create wasm-specific main
```rust
// src/lib.rs (if not already a lib)
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    // Run the app
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
```

#### 4. Build
```bash
wasm-pack build --target web --out-dir www/pkg
```

#### 5. Create index.html
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FieldCalc - Interactive SDF Editor</title>
    <style>
        body { margin: 0; padding: 0; overflow: hidden; }
        canvas { display: block; width: 100vw; height: 100vh; }
    </style>
</head>
<body>
    <script type="module">
        import init from './pkg/solid_modeling.js';
        init();
    </script>
</body>
</html>
```

#### 6. Serve locally
```bash
python3 -m http.server 8000 --directory www
# Or use: npx serve www
```

#### 7. Test on Mobile
- Connect to `http://YOUR_IP:8000` from your phone
- Should work on iOS Safari, Android Chrome, etc.

### Deploy to GitHub Pages / Vercel / Netlify
1. Build: `wasm-pack build --target web --release`
2. Copy `www/` directory to hosting
3. Done! Access from anywhere

## Native Mobile Apps (Advanced)

If you need a native app (for App Store/Play Store):

### iOS via winit + wgpu
**Status**: Possible but requires setup
**Effort**: ~1-2 days configuration + testing

1. Install cargo-mobile2:
   ```bash
   cargo install cargo-mobile2
   cargo mobile init
   ```

2. Add iOS target:
   ```bash
   rustup target add aarch64-apple-ios
   ```

3. Build:
   ```bash
   cargo mobile ios build --release
   ```

4. Challenges:
   - Need macOS + Xcode
   - Code signing certificates
   - App Store review process
   - Touch input handling (egui supports it)

### Android via winit + wgpu
**Status**: Possible but requires setup
**Effort**: ~1-2 days configuration + testing

1. Install cargo-mobile2 (same as iOS)

2. Add Android target:
   ```bash
   rustup target add aarch64-linux-android
   ```

3. Install Android SDK/NDK

4. Build:
   ```bash
   cargo mobile android build --release
   ```

5. Challenges:
   - Android SDK setup
   - Play Store review
   - Multiple device testing
   - Permissions handling

## Recommendation: Start with Web

### For Mobile Testing Now:
1. Build for WASM (10 minutes)
2. Serve locally or deploy to free hosting
3. Access from mobile browser
4. Full functionality, zero app store hassle

### Progressive Enhancement:
1. **Week 1**: Web deployment (done in hours)
2. **Week 2**: PWA for "install to home screen" (optional)
3. **Later**: Native apps if needed for:
   - Offline-first use cases
   - Hardware acceleration requirements
   - App store presence

## Current Status

### ✅ Completed
- Interactive editor with egui
- Split-pane UI (code + output)
- Live parsing and evaluation
- Cross-platform desktop build

### 🔨 Next Steps for Web
1. Add wasm-bindgen dependencies
2. Create wasm entry point
3. Test WASM build
4. Deploy to hosting

### 📦 Estimated Bundle Sizes
- **WASM**: 2-5 MB (compressed)
- **Native iOS**: 10-20 MB
- **Native Android**: 15-25 MB

## Quick Test: Can I Try It Now?

**On your machine**:
```bash
cargo run --bin solid_viewer
# Press TAB to toggle editor mode
# Edit FieldCalc code in left pane
# See live rendering on right
```

**On mobile**: Web build required first (see above), then access via browser.

## Claude Code on Mobile

You asked: "Still trying to figure out how effective I can work with Claude Code on mobile"

**Current best approach**:
1. Use Claude Code on desktop to write/edit code
2. Build for web locally or via CI
3. Access running app on mobile browser for testing
4. Iterate with Claude Code on desktop

**Future improvement**:
- GitHub Codespaces + mobile browser for full Claude Code experience
- Termux + code-server on Android (advanced)

## Resources

- [egui web demo](https://www.egui.rs/#demo)
- [wgpu on web](https://wgpu.rs/doc/wgpu/#platform-support)
- [cargo-mobile2](https://github.com/tauri-apps/cargo-mobile2)
- [WebAssembly book](https://rustwasm.github.io/docs/book/)

---

**Summary**: Web deployment is fastest path to mobile testing. Native apps possible but more work. Choose based on needs:
- **Just testing**: Web (hours)
- **Public beta**: Web + PWA (days)
- **App stores**: Native builds (weeks)
