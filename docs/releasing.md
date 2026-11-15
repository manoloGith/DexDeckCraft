# Publicar una versión

1. Sube `dexdeckcraft.version` y `dexdeckcraft.versionCode` en `apps/dexdeckcraft/android/gradle.properties`
   y `version` en `Cargo.toml` (`[workspace.package]`), y ejecuta `cargo update -w` para que
   `Cargo.lock` recoja la nueva versión (el build usa `--locked`). El código de versión debe crecer
   siempre.
2. Escribe las notas en `docs/release-notes/v<versión>.md`.
3. Haz commit y sube un tag `v<versión>`, o lanza a mano el workflow **Release** (Actions ▸ Release ▸
   Run workflow), que crea el tag con la versión de `gradle.properties`:

```bash
git tag v0.1.0 && git push origin v0.1.0
```

El workflow `.github/workflows/release.yml` compila el APK (arm64-v8a), lo firma y publica el
release con el APK y `SHA256SUMS.txt`.

## La clave de firma

Android solo instala una actualización si está firmada con la misma clave que la versión
instalada. La clave **no está en el repositorio**: el workflow la lee de cuatro secretos
(Settings ▸ Secrets and variables ▸ Actions):

| Secreto | Contenido |
|---|---|
| `DEXDECKCRAFT_KEYSTORE_BASE64` | el archivo `.jks` en base64 (`base64 -w0 dexdeckcraft-release.jks`) |
| `DEXDECKCRAFT_KEYSTORE_PASSWORD` | la contraseña del almacén |
| `DEXDECKCRAFT_KEY_ALIAS` | `dexdeckcraft` |
| `DEXDECKCRAFT_KEY_PASSWORD` | la contraseña de la clave (la misma; el workflow usa la del almacén, como en todo almacén PKCS12) |

El workflow quita espacios y saltos de línea pegados por error, y si el alias no existe en el
almacén usa su única clave. Sin los secretos, el workflow firma con una clave de depuración y publica una *pre-release*, que no se
puede actualizar con versiones firmadas con la clave real. Guarda la clave y su contraseña en un
lugar seguro, con copia: si se pierden, ninguna versión futura podrá actualizar las instaladas.

Todavía **no hay clave de DexDeckCraft**. Para crearla (una vez):

```bash
keytool -genkeypair -v -storetype PKCS12 -keystore dexdeckcraft-release.jks -alias dexdeckcraft -keyalg RSA -keysize 4096 -validity 10000
```

Después añade los cuatro secretos y anota aquí la huella SHA-256 del certificado
(`keytool -list -v -keystore dexdeckcraft-release.jks`).
