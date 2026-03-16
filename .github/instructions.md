# UI Design Instructions

- Treat the interface as mobile-first, even though the current runtime target is Tauri desktop.
- Design primary screens for phone-sized viewports first, then scale up for tablet and desktop.
- Prioritize touch-friendly controls, simple vertical flow, and clear first-view actions.
- Assume portrait orientation as the default layout unless a screen has a strong reason to behave differently.
- Keep important actions and status information visible without requiring precise mouse input.
- Avoid layouts that only work when there is abundant horizontal space.
- Use responsive spacing and typography so the UI remains readable from small phones through desktop windows.
- When making UI decisions, prefer patterns that would still feel natural if the same product ships on mobile later.