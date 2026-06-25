# SDKWork IM Flutter Mobile Application

## Entry Point

This is the Flutter mobile application root for SDKWork IM. See [../../AGENTS.md](../../AGENTS.md) for repository-level agent instructions.

## SDKWork Specs

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md`
- `../../../sdkwork-specs/APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`
- `../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md`
- `../../../sdkwork-specs/CONFIG_SPEC.md`

## Application Identity

- App ID: `sdkwork-im-flutter-mobile`
- Runtime family: `mobile`
- Framework: `flutter`
- Deep link callback: `sdkworkim://auth/callback`

## Build And Verify

```powershell
flutter pub get
flutter analyze
flutter test
pnpm run test:sdkwork-im-flutter-mobile-architecture-standard
```
