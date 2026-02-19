# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-02-19

### Added

#### Global Keyboard Shortcuts
- Custom keyboard shortcuts for sound buttons (work even when app is minimized)
- Shortcut badge display on buttons (centered at top)
- Right-click menu to set/clear shortcuts

#### Navigation Keyboard Shortcuts
- `Q/A/S` + `←` - Previous page
- `D/Z/W` + `→` - Next page  
- `Space` - Pause/Resume all sounds
- `T` - Toggle queue panel
- `Escape` - Close queue panel
- `Ctrl+S` - Stop all sounds
- `M` - Mute/Unmute master volume

#### Audio Format Support
- **Opus** codec support (OGG container)
- Extended format support: MP1, MP2, AIFF, WebM, MKV
- Better error messages for unsupported codecs

### Changed

- Volume changes now apply in real-time to playing sounds
- Audio visualizer responds to volume changes in real-time
- Page navigation now has slide animation (direction-aware)
- `enable_gapless: true` for better OGG/Vorbis playback
- Upgraded Symphonia to 0.5.5 for OGG fixes

### Fixed

- OGG files with Opus codec now play correctly
- Visualizer reflects volume changes in real-time
- Space key no longer scrolls the page when triggering pause
