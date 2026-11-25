# PixelVault — Android prototype

**Mask. Transfer. Preserve.** This sandbox package is an early Flutter implementation and build target, not a completed production product.

## Included now
- Flutter UI scaffold with yellow/white/grey/graphite theme and 3D/user-provided assets.
- Home, image-carrier package, share, vault, activity and settings screens.
- Importing copies selected files into app-local storage; persistent index uses SharedPreferences.
- Android share-sheet support via `share_plus` (WhatsApp can be a share target if installed).
- `PVIC01` carrier container: explicitly stores an image and a payload side-by-side with SHA-256 checksums and allows restoration. This is a documented package/container, **not pixel steganography or covert hiding**.
- Codemagic workflow to generate Android scaffolding, test, analyze and build APK.
- Separate Rust/WASM crate skeleton for transport framing and XOR shard reconstruction.

## Not implemented / not verified
- Pixel-level steganographic masking/hidden payload extraction.
- PVAB codec integration (existing PVAB repository remains separate).
- Dedicated Wi-Fi discovery/transport, Bluetooth transport, optical camera/frame transmitter/receiver, RaptorQ symbol coding.
- Full vault encryption, biometrics gating, app-level permissions UX and production threat model.
- Flutter compilation: Flutter SDK is not installed in the sandbox; build validation is deferred to Codemagic.

## Local build
1. Install Flutter stable and Android SDK.
2. Run `flutter create --platforms android --org com.genic.pixelvault --project-name pixelvault --overwrite .`.
3. `flutter pub get`, `dart run flutter_launcher_icons`, `dart run flutter_native_splash:create`.
4. `flutter test`, `flutter analyze`, `flutter build apk --release`.

## Carrier container format
- 6 bytes ASCII `PVIC01`
- 4-byte big-endian JSON metadata length
- JSON metadata (names, byte lengths, SHA-256 checksums)
- carrier image bytes
- payload file bytes

The carrier format is transparent and does not attempt to make embedded data invisible. The generated `.pvic` file should be treated as an ordinary archive-like file.
