# Nihongo — Windows

Versión de Nihongo para **Windows 10/11**. Es el mismo proyecto que la versión de macOS, con los componentes nativos adaptados:

- **TTS (voz japonesa)** → usa Windows SAPI vía PowerShell (`System.Speech.Synthesis`). Necesita el paquete de idioma japonés instalado.
- **Notificaciones** → toast notifications nativas de Windows 10/11 vía PowerShell + `Windows.UI.Notifications`.
- **Mic settings** → abre `ms-settings:privacy-microphone` directamente.
- **Bundle** → genera `.exe` (NSIS installer) y `.msi`.

Todo el resto (lecciones, kanji, gramática, diario con Ollama/Claude, mini-juegos, exámenes, etc.) es idéntico.

---

## Cómo compilar en Windows

### Prerequisitos (instalar una sola vez)

1. **Rust** — descarga desde <https://rustup.rs/> y ejecuta el instalador. Acepta los valores por defecto.
2. **Node.js LTS** — desde <https://nodejs.org/>. Mínimo Node 18.
3. **Microsoft C++ Build Tools** — desde <https://visualstudio.microsoft.com/visual-cpp-build-tools/>. Marca "Desktop development with C++" durante la instalación.
4. **WebView2 Runtime** — en Windows 11 viene preinstalado. En Windows 10, instálalo desde <https://developer.microsoft.com/microsoft-edge/webview2/>.
5. **Paquete de idioma japonés** (opcional, solo necesario para TTS) — Configuración → Hora e idioma → Idioma → Agregar idioma → 日本語.

### Construir el `.exe`

Desde PowerShell o CMD, en la carpeta del proyecto:

```powershell
cd nihongo-windows
npm install
npm run tauri build
```

Esto tarda ~5–10 minutos la primera vez. Al terminar encontrarás los instaladores en:

```
src-tauri\target\release\bundle\nsis\Nihongo_0.1.0_x64-setup.exe   ← instalador NSIS (recomendado)
src-tauri\target\release\bundle\msi\Nihongo_0.1.0_x64_en-US.msi    ← instalador MSI
```

Cualquiera de los dos sirve para distribuir.

### Probar sin instalar (dev)

```powershell
npm run tauri dev
```

Abre una ventana Tauri con recarga en caliente del frontend. Útil mientras desarrollas; usa el `.exe` para distribución.

---

## Diferencias con la versión de macOS

Si más adelante quieres sincronizar cambios desde la versión Mac:

| Cambio | Archivo | Por qué |
|---|---|---|
| `tauri.conf.json` | sin `macOSPrivateApi`, sin `entitlements`, sin `titleBarStyle`/`transparent`. Targets: `nsis`, `msi`. | Configuración específica de Windows. |
| `Cargo.toml` | quita la feature `macos-private-api` de Tauri. | No existe en Windows. |
| `src-tauri/src/commands/listening.rs` | `play_japanese_tts` usa PowerShell + SAPI. | macOS usa `say`, Windows usa SAPI. |
| `src-tauri/src/commands/notifications.rs` | `send_notification` usa toast Windows, `open_mic_settings` abre `ms-settings:`. | macOS usa `osascript` y `x-apple.systempreferences:`. |
| Eliminados | `Info.plist`, `entitlements.plist` | Solo macOS. |

Cualquier otro cambio (UI, lecciones, kanji, módulos, etc.) es compatible y puede copiarse directo de la versión Mac.

---

## Compartir el `.exe` con amigos

Mira [`SHARE.md`](./SHARE.md) para la guía completa de distribución sin presupuesto (cero servicios pagados).
