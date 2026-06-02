# Compartir Nihongo con tus amigos (sin presupuesto)

Esta guía explica cómo distribuir el `.exe` (o `.msi`) generado a tus amigos de Windows usando **solo opciones gratuitas**.

---

## Lo que vas a entregarles

Después de correr `npm run tauri build` en una máquina Windows tendrás:

```
src-tauri\target\release\bundle\nsis\Nihongo_0.1.0_x64-setup.exe   (~5 MB)
src-tauri\target\release\bundle\msi\Nihongo_0.1.0_x64_en-US.msi    (~6 MB)
```

Comparte **uno** de los dos (el NSIS es más amigable). Cada amigo lo ejecuta, instala Nihongo, y la app aparece en el menú Inicio.

---

## Dónde vas a subir el archivo

### Opción 1 — GitHub Releases (recomendada)

Gratis, sin límite de descargas, da link bonito.

1. Crea cuenta en <https://github.com/signup>.
2. En tu Mac, donde tienes la versión de origen (`lenguaje-e-learning`), sube el código:
   ```bash
   cd /Users/rodrigoflores/dev-rodrigo/personal-projects/lenguaje-e-learning
   git init
   git add .
   git commit -m "Initial commit"
   gh repo create nihongo --public --source=. --push
   ```
   (Necesitas instalar GitHub CLI: `brew install gh && gh auth login`.)
3. Una vez compilado el `.exe` en Windows, súbelo creando una "Release":
   ```bash
   gh release create v0.1.0 Nihongo_0.1.0_x64-setup.exe --notes "Primera versión"
   ```
4. Compartes el link `https://github.com/TU_USUARIO/nihongo/releases/latest` — tus amigos ven la página con un botón "Download".

### Opción 2 — Google Drive / Dropbox / iCloud

Más simple, sin línea de comandos:

1. Sube el `.exe` a tu Drive/Dropbox.
2. Genera un link de "compartir con cualquiera que tenga el link".
3. Compartes ese link.

Funciona bien pero el link se ve menos profesional y a veces Drive le mete escaneo antivirus que pide confirmación extra.

### Opción 3 — WeTransfer

<https://wetransfer.com> — hasta 2 GB gratis, sin cuenta. El link caduca en 7 días, así que cada vez que saques nueva versión tienes que reenviar. Útil para una entrega puntual a uno o dos amigos.

---

## Lo que tus amigos verán al instalar

Como **no firmaste digitalmente el `.exe`** (los certificados de Windows cuestan ~$200/año), Windows mostrará:

> **Windows protegió tu PC**
> Microsoft Defender SmartScreen evitó el inicio de una aplicación no reconocida.

Para abrirlo:

1. Click en **"Más información"** (link pequeño).
2. Aparece un botón **"Ejecutar de todas formas"** — púlsenlo.
3. Solo la primera vez. Después Windows recuerda la decisión.

Algunos antivirus pueden marcar también el `.exe`. Pídeles a tus amigos que lo agreguen a la lista de excepciones.

Mensaje útil para mandarles junto con el link:

> Hola! Te mando Nihongo, mi app para practicar japonés. Bájate el `.exe`, instalalo, y al abrir la primera vez Windows te dirá "no reconocido" — pulsa "Más información" → "Ejecutar de todas formas". Es porque no firmé el binario (certificados de Microsoft cuestan $200/año y solo es para uso entre amigos). El código es público y está en mi GitHub, no hay nada raro.

---

## Datos y configuración por usuario

Cada amigo tendrá su propia base de datos local, en:

```
%APPDATA%\com.nihongo.app\nihongo.db
```

Eso significa que su nivel, XP, kanjis aprendidos, etc. son suyos — nada se comparte, ni se sube a internet, ni se entera nadie.

Si quieren correcciones IA del diario, cada uno elige:
- **Ollama (local, gratis)** — instalan Ollama desde <https://ollama.com/download> y corren `ollama pull llama3.1:8b`. Una vez. Después funciona sin internet.
- **Claude API (de pago)** — necesitan su propia API key.

---

## Cuando saques una nueva versión

1. Subes una versión `.exe` nueva al mismo lugar (otro tag en GitHub Releases, otro archivo en Drive).
2. Tus amigos la descargan e instalan encima de la anterior — sus datos se conservan (viven en `%APPDATA%`, fuera de la carpeta de instalación).

---

## ¿No tienes acceso a una máquina Windows?

Para compilar el `.exe` necesitas Windows. Opciones:

1. **Pedir prestada una PC** unos 30 minutos para instalar Rust + Node + correr `npm run tauri build`.
2. **Máquina virtual gratis** — descarga una imagen de evaluación de Windows desde <https://developer.microsoft.com/windows/downloads/virtual-machines/> (vence en 90 días, suficiente).
3. **GitHub Actions** — más adelante podemos configurar un workflow que builde el `.exe` automáticamente cada vez que crees un tag. Es gratis para repos públicos.
