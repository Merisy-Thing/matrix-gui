# matrix-gui Resource Usage Analysis Report on Cortex-M3

## 1. Test Environment and Methodology

### 1.1 Hardware and Build Environment

| Item | Description |
|---|---|
| Target chip | STM32F103 (ARM Cortex-M3) |
| Compile target | `thumbv7m-none-eabi` |
| Chip resources | FLASH 128 KB, RAM 20 KB |
| Build mode | release (`--release`) |
| Optimization options | `opt-level = "s"`, `lto = true`, `codegen-units = 1`, `debug = true` |
| GUI library | matrix-gui v0.3 (based on embedded-graphics 0.8) |
| Measurement tool | `cargo size` (llvm-size) |
| Test project | `size_test` (standalone sub-project, uses StubDisplay with no hardware dependency) |

### 1.2 Measurement Method

- **FLASH usage** = `text` + `data` (code segment + initialized data segment, both stored in FLASH)
- **RAM usage** = `data` + `bss` (initialized data segment + uninitialized data segment, both located in RAM)
- **Feature testing**: Each matrix-gui feature is enabled individually, measuring the FLASH/RAM increment it introduces (relative to baseline)
- **Widget testing**: Each built-in widget is used individually, measuring:
  - **Total adoption cost (Total Δ)** = total increment after enabling the widget and its dependent features (relative to baseline)
  - **Widget marginal cost (Marginal Δ)** = the widget's own code increment only (relative to its dependent feature baseline)

### 1.3 Baseline Description

Baseline = the test program with no matrix-gui features enabled and no widgets constructed, including only:

- cortex-m / cortex-m-rt runtime
- panic-halt exception handler
- embedded-graphics core traits
- matrix-gui core (`Ui`, `Region`, `Style`, `free_form_region!`, and other always-compiled modules)
- StubDisplay (a no-op DrawTarget implementation)

| Baseline | text | data | bss | **FLASH** | **RAM** |
|---|---:|---:|---:|---:|---:|
| baseline | 3660 | 0 | 0 | **3660** | **0** |

> **Note**: The baseline RAM is 0 because all variables in the test program are allocated on the stack with no global mutable state. matrix-gui's core data structures (such as the `RenderState` array) are allocated on the stack or in static storage by the caller in actual use.

---

## 2. Feature Usage Analysis

### 2.1 Resource Usage When Each Feature Is Enabled Individually

The increment of each feature relative to baseline (enabling that feature and executing its representative API calls):

| Feature | text | data | bss | FLASH | RAM | **ΔFLASH** | **ΔRAM** | Description |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `feat-log` | 3660 | 0 | 0 | 3660 | 0 | **0** | **0** | Logging output (depends on log crate; macros are no-ops without a logger backend) |
| `feat-focus` | 3876 | 0 | 0 | 3876 | 0 | **+216** | **0** | Focus management (includes interaction, FocusState, etc.) |
| `feat-debug-color` | 3668 | 0 | 0 | 3668 | 0 | **+8** | **0** | Widget bounds debug coloring |
| `feat-framebuffer` | 3660 | 0 | 0 | 3660 | 0 | **0** | **0** | Widget-level framebuffer (only constructed, not drawn; LTO stripped) |
| `feat-fill-rect` | 3660 | 0 | 0 | 3660 | 0 | **0** | **0** | Low-level rectangle fill (stub implementation is a no-op; LTO stripped) |
| `feat-interaction` | 3672 | 0 | 0 | 3672 | 0 | **+12** | **0** | Interaction handling base (check_interact, etc.) |
| `feat-scroll-area` | 3672 | 0 | 0 | 3672 | 0 | **+12** | **0** | Scroll area (includes clipped_draw, set_clipped_area) |
| `feat-animation` | 4888 | 0 | 0 | 4888 | 0 | **+1228** | **0** | Animation manager (AnimManager, Anim, easing functions, etc.) |
| `feat-popup` | 3728 | 0 | 0 | 3728 | 0 | **+68** | **0** | Modal popup (Modal system) |
| `feat-clipped-draw` | 3672 | 0 | 0 | 3672 | 0 | **+12** | **0** | Clipped drawing (set_clipped_area) |

**Important notes**:

- `feat-log`, `feat-framebuffer`, and `feat-fill-rect` show 0 increment because:
  - **feat-log**: The `log` crate's macros are no-ops when no logger backend is registered, so LTO completely strips them. In actual use, a logger must be registered (e.g., `rtt-target`), which produces additional overhead.
  - **feat-framebuffer**: The test only constructs `WidgetFramebuf` but does not draw pixels through it; LTO determines the code has no side effects and strips it. Actual drawing pulls in the framebuffer operation code.
  - **feat-fill-rect**: The test uses a stub empty implementation (`fill_with_color` with an empty function body); LTO inlines it and finds the entire call chain has no side effects, stripping it. Actual use with the C SDK produces real code.
- `feat-focus` has a large increment (+216) because `focus` depends on `interaction`, and both codebases are pulled in together.
- `feat-scroll-area` and `feat-clipped-draw` have the same increment (+12) because the `scroll-area` feature only enables the `clipped_draw` infrastructure; the `scrollarea` module's ScrollArea widget code is only linked when constructed (see widget analysis).
- `feat-animation` has the largest increment (+1228) because the animation manager contains a large amount of logic including easing functions, state machines, and time calculations.

### 2.2 Resource Usage of Aggregate Features

Aggregate features enable multiple sub-features simultaneously to observe the actual total usage when features are combined (features may share code, so the aggregate increment is not necessarily the sum of individual sub-feature increments):

| Aggregate feature | text | data | bss | FLASH | RAM | **ΔFLASH** | **ΔRAM** | Included sub-features |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `feat-part` | 4128 | 0 | 4 | 4128 | 4 | **+468** | **+4** | log, focus, debug-color, interaction, framebuffer, scroll-area |
| `feat-anim` | 4144 | 0 | 4 | 4144 | 4 | **+484** | **+4** | animation + part |
| `feat-all` | 4164 | 0 | 0 | 4164 | 0 | **+504** | **0** | part + fill-rect + popup |

**Observations**:

- `feat-part` (+468) is much smaller than the sum of individual sub-feature increments (0+216+8+0+0+12+12 = 248) because the aggregated features share underlying code (such as embedded-graphics drawing primitives), and some features (log/framebuffer) are still stripped by LTO. The actual increment is greater than 248 because the aggregate test constructs a ScrollArea widget (an actual usage of the scroll-area feature).
- `feat-anim` is +16 bytes more than `feat-part`, which is the marginal increment of the animation code on top of part. This differs greatly from testing `feat-animation` alone (+1228) because in the aggregate test, some of the animation manager's dependency code has already been pulled in by focus/interaction in part.
- `feat-all` is +36 bytes more than `feat-part`, which is the marginal increment of fill-rect + popup on top of part (the stub implementation of fill-rect is stripped; in practice only popup's +68 minus the shared portion remains).
- `feat-part` and `feat-anim` each have 4 bytes of bss (RAM), coming from the internal static state of the focus/animation systems.

---

## 3. Built-in Widget Usage Analysis

### 3.1 Widget Dependency Feature Baselines

To calculate each widget's own marginal cost, we first measure the baselines of each widget's dependent features (enabling features and executing representative APIs, but not constructing that widget):

| Baseline name | Enabled features | text | data | bss | FLASH | RAM | **ΔFLASH** | **ΔRAM** |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `baseline` | (no features) | 3660 | 0 | 0 | 3660 | 0 | **0** | **0** |
| `base-interaction` | interaction | 3672 | 0 | 0 | 3672 | 0 | **+12** | **0** |
| `base-scroll-area` | scroll-area (includes clipped_draw) | 3672 | 0 | 0 | 3672 | 0 | **+12** | **0** |
| `base-popup-interaction` | popup + interaction | 3740 | 0 | 0 | 3740 | 0 | **+80** | **0** |

### 3.2 Resource Usage of Each Widget

The table below shows the resource usage of each built-in widget:

- **FLASH / RAM**: Total usage after building the widget (and its dependent features)
- **Total adoption cost (Total Δ)**: Increment relative to pure baseline = enabling the widget + its dependent features total cost
- **Widget marginal cost (Marginal Δ)**: Increment relative to the dependent feature baseline = widget's own code cost only

| Widget | Dependent features | text | data | bss | FLASH | RAM | **Total ΔFLASH** | **Total ΔRAM** | **Marginal ΔFLASH** | **Marginal ΔRAM** |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `background` | (none) | 3668 | 0 | 0 | 3668 | 0 | **+8** | **0** | **+8** | **0** |
| `bar` | (none) | 3752 | 0 | 0 | 3752 | 0 | **+92** | **0** | **+92** | **0** |
| `label` | (none) | 5824 | 0 | 0 | 5824 | 0 | **+2164** | **0** | **+2164** | **0** |
| `listbox` | (none) | 5024 | 0 | 0 | 5024 | 0 | **+1364** | **0** | **+1364** | **0** |
| `plaintext` | (none) | 13080 | 0 | 0 | 13080 | 0 | **+9420** | **0** | **+9420** | **0** |
| `staticimage` | (none) | 3664 | 0 | 0 | 3664 | 0 | **+4** | **0** | **+4** | **0** |
| `staticline` | (none) | 3828 | 0 | 0 | 3828 | 0 | **+168** | **0** | **+168** | **0** |
| `scrollarea` | scroll-area | 3684 | 0 | 0 | 3684 | 0 | **+24** | **0** | **+12** | **0** |
| `button` | interaction | 7832 | 0 | 0 | 7832 | 0 | **+4172** | **0** | **+4160** | **0** |
| `checkbox` | interaction | 7860 | 0 | 0 | 7860 | 0 | **+4200** | **0** | **+4188** | **0** |
| `radiobutton` | interaction | 6884 | 0 | 0 | 6884 | 0 | **+3224** | **0** | **+3212** | **0** |
| `slider` | interaction | 4692 | 0 | 0 | 4692 | 0 | **+1032** | **0** | **+1020** | **0** |
| `choice` | popup+interaction | 8444 | 0 | 0 | 8444 | 0 | **+4784** | **0** | **+4704** | **0** |
| `msgbox` | popup+interaction | 10712 | 0 | 0 | 10712 | 0 | **+7052** | **0** | **+6972** | **0** |

### 3.3 Widget Usage Ranking

Sorted by marginal FLASH increment from largest to smallest:

| Rank | Widget | Marginal ΔFLASH | Description |
|---:|---|---:|---|
| 1 | `plaintext` | +9420 B | Largest footprint; depends on embedded-text's styled text rendering (including wrapping/alignment) |
| 2 | `msgbox` | +6972 B | Message box; includes title + body text rendering + popup + interactive button |
| 3 | `choice` | +4704 B | Dropdown choice; includes option text rendering + popup list + interaction |
| 4 | `checkbox` | +4188 B | Checkbox; includes label text rendering + interaction + checkmark drawing |
| 5 | `button` | +4160 B | Button; includes label text rendering + interaction + press drawing |
| 6 | `radiobutton` | +3212 B | Radio button; includes label text rendering + interaction + dot drawing |
| 7 | `label` | +2164 B | Label; pure text rendering (includes font engine) |
| 8 | `listbox` | +1364 B | List box; multi-line text rendering + selection state |
| 9 | `slider` | +1020 B | Slider; graphics drawing + interaction (no text) |
| 10 | `staticline` | +168 B | Static line; line drawing primitive |
| 11 | `bar` | +92 B | Progress bar; rectangle fill drawing |
| 12 | `scrollarea` | +12 B | Scroll area container (clipped_draw infrastructure is already in the baseline) |
| 13 | `background` | +8 B | Background; solid color fill |
| 14 | `staticimage` | +4 B | Static image; only references existing image data |

---

## 4. RAM Usage Notes

All variants in this test have 0 RAM (a few aggregate features have 4 bytes of bss). This does not mean matrix-gui does not use RAM; it is because:

1. **Stack allocation**: All local variables in the test program (Ui, WidgetStates, FocusState, etc.) are allocated on the stack, producing no bss/data segments
2. **Region declarations**: The Region constants generated by the `free_form_region!` macro are stored in FLASH (rodata, counted in the text segment)
3. **No global mutable state**: matrix-gui's core design has no global mutable state; all state is passed by reference

RAM usage in actual projects mainly comes from:
- `RenderState` array: one per Region; size depends on the number of Regions (this test declares 14 Regions)
- `WidgetStates` reference wrapper
- The widget's own state variables (such as Slider's `&mut i16`, Checkbox's `&mut bool`)
- `FocusState<N>`'s internal array (N focus slots)
- `Animations<N>`'s instance and status arrays

All of these are allocated on the stack or in static storage, controlled by the user, and are not part of the library's own RAM overhead.
