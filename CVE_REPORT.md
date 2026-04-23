# CVE / Security Advisory Report — sysworxx-io

Erstellt: 2026-04-21  
Aktualisiert: 2026-04-22

---

## Rust-Abhängigkeiten (via RustSec / OSV)

Analysierte Versionen aus `Cargo.lock` (Stand: 2026-04-22).

> **Hinweis:** Das Cargo.lock enthält für `nix` mehrere Versionen: 0.16.1 (direkte Abhängigkeit in `sysworxx-io`), sowie 0.26.4, 0.27.1, 0.29.0, 0.30.1 (Workspace-Mitglieder). Nur 0.16.1 ist betroffen.

| Priorität | Paket | Version | Advisory | CVE | CVSS | Problem | Fix |
|-----------|-------|---------|----------|-----|------|---------|-----|
| **KRITISCH** | `nix` | 0.16.1 | RUSTSEC-2021-0119 / GHSA-76w9-p8mg-j927 | CVE-2021-45707 | Mem. Corruption | Out-of-bounds Write in `getgrouplist` bei Nutzern mit >16 Gruppen → Memory Corruption | Upgrade auf `>= 0.20.2` (0.16.x hat kein Backport) |
| **HOCH** | `crossbeam-utils` | 0.7.2 | RUSTSEC-2022-0041 / GHSA-qc84-gqf4-9926 | CVE-2022-23639 | 8.1 | Unsound `AtomicCell<u64>` auf 32-bit Targets mit 64-bit Atomic-Support (z. B. ARMv7) → Data Race | Upgrade auf `>= 0.8.7` (0.7.x kein Patch; nur relevant bei 32-bit!) |
| LOW | `atty` | 0.2.14 | RUSTSEC-2021-0145 / GHSA-g98v-hv3f-hcfr | — | Low (Windows only) | Potenziell unalignierter Read auf Windows; kein Fix mehr möglich (Crate inaktiv) | Ersetzen durch `std::io::IsTerminal` (stable seit Rust 1.70) oder `is-terminal` |
| INFO | `atty` | 0.2.14 | RUSTSEC-2024-0375 | — | — | Crate **unmaintained** und archiviert | Ersetzen durch `is-terminal` |
| INFO | `serde_cbor` | 0.11.2 | RUSTSEC-2021-0127 | — | — | Crate **unmaintained** | Migration auf `ciborium` oder `minicbor` |
| INFO | `json` | 0.12.4 | RUSTSEC-2022-0081 | — | — | Crate **unmaintained** (kein Release seit >3 Jahren, Maintainer nicht erreichbar) | Migration auf `serde_json`, `jzon` oder `simd-json` |

**Sauber (keine Advisories):** `parking_lot 0.10.2`, `signal-hook 0.1.17`, `env_logger 0.7.1`, `evdev 0.13.1`, `libc 0.2.x`

---

## Node.js-Abhängigkeiten (npm audit — `node-red-contrib-sysworxx-io`)

Analysierte Versionen aus `package-lock.json` (Stand: 2026-04-22).

| Schwere | Paket | Version (Lock) | Advisory | CVE | CVSS | Problem | Fix |
|---------|-------|----------------|----------|-----|------|---------|-----|
| **HIGH** | `flatted` | 3.3.3 | GHSA-25h7-pfq9-p65f | CVE-2026-32141 | 7.5 | DoS via unbegrenzte Rekursion in `parse()` → Stack Overflow | `>= 3.4.2` |
| moderate | `flatted` | 3.3.3 | GHSA-rf6f-7fwh-wjgh | CVE-2026-33228 | 4.0 | Prototype Pollution via `parse()` (Array-Index ohne Validierung von `__proto__`) | `>= 3.4.2` |
| **HIGH** | `minimatch` | 3.1.2 | GHSA-3ppc-4f35-3m26 | CVE-2026-26996 | 8.7 | ReDoS via repeated Wildcards — O(4^N) Backtracking | `>= 3.1.3` |
| **HIGH** | `minimatch` | 3.1.2 | GHSA-7r86-cg39-jmmj | CVE-2026-27903 | 7.5 | ReDoS via mehrfache nicht-benachbarte `**` GLOBSTAR-Segmente | `>= 3.1.3` |
| **HIGH** | `minimatch` | 3.1.2 | GHSA-23c5-xmqv-rm74 | CVE-2026-27904 | 7.5 | ReDoS via verschachtelte `*()` extglobs — katastrophales Backtracking | `>= 3.1.4` |
| **HIGH** | `picomatch` | 2.3.1 | GHSA-c2c7-rcm5-vvqj | CVE-2026-33671 | **9.1** | ReDoS via extglob-Pattern (z. B. `+(a|aa)`) — 41-Zeichen-Input blockiert Event Loop für ~2 s | `>= 2.3.2` |
| LOW | `picomatch` | 2.3.1 | GHSA-3v7f-55p6-f55p | CVE-2026-33672 | 3.1 | Method Injection via POSIX Character Class (`[[:constructor:]]`) → fehlerhafte Glob-Auswertung | `>= 2.3.2` |
| moderate | `ajv` | 6.12.4 | GHSA-2g4f-4pwh-qvx6 | CVE-2025-69873 | 4.0 | ReDoS mit `$data`-Option — nutzerkontrolliertes Regex-Pattern, 31-Zeichen-Payload = 44 s CPU-Block | `>= 6.14.0` |
| moderate | `axios` | 1.8.4 | GHSA-3p68-rc4w-qgx5 | CVE-2025-62718 | 5.4 | NO_PROXY-Bypass → SSRF (Anfragen an `localhost.` / `[::1]` umgehen Proxy-Regeln) | `>= 1.15.0` |
| LOW | `axios` | 1.8.4 | GHSA-fvcv-3m26-pcqx | CVE-2026-40175 | 3.1 | Header Injection via Prototype Pollution → Cloud-Metadata-Exfiltration (AWS IMDSv2 Bypass) | `>= 1.15.0` |
| moderate | `brace-expansion` | 1.1.11 | GHSA-f886-m6hf-6m8v | CVE-2026-33750 | 6.5 | Zero-Step-Sequence (`{1..2..0}`) → Infinite Loop, ~1,9 GB RAM-Verbrauch, nur 10 Byte Input | `>= 1.1.13` |
| LOW | `brace-expansion` | 1.1.11 | GHSA-v6h2-p8h4-qcjw | CVE-2025-5889 | 3.1 | ReDoS in `expand()` — schwer ausnutzbar | `>= 1.1.12` |

**12 Advisories über 6 Pakete: 5 HIGH, 4 moderate, 3 low**

> **Hinweis `ajv`:** Das Lock-File enthält zwei Versionen: `6.12.4` (aus `@eslint/eslintrc`, betroffen) und `8.17.1` (aus `@node-red/nodes`, nach aktuellen Informationen ebenfalls in der betroffenen Range 7.0–8.17.1; Fix: `>= 8.17.2`).

---

## Empfehlungen

### Sofort (KRITISCH)

**nix upgraden (CVE-2021-45707 — Memory Corruption):**

```toml
# Cargo.toml
nix = "0.29"
```

```bash
cargo update -p nix
```

### Node.js — alle 12 Advisories automatisch beheben

```bash
cd Bindings/node-red-contrib-sysworxx-io
npm audit fix
```

Manuelle Ziel-Versionen falls `npm audit fix` nicht ausreicht:

| Paket | Ziel-Version |
|-------|-------------|
| `flatted` | `>= 3.4.2` |
| `minimatch` | `>= 3.1.4` |
| `picomatch` | `>= 2.3.2` |
| `ajv` (6.x) | `>= 6.14.0` |
| `axios` | `>= 1.15.0` |
| `brace-expansion` | `>= 1.1.13` |

### Mittelfristig

- **`crossbeam-utils 0.7.2`** auf `>= 0.8.7` updaten — relevant bei 32-bit Targets (ARMv7):
  ```bash
  cargo update -p crossbeam-utils
  ```
- **`atty`** durch `std::io::IsTerminal` ersetzen (stabil seit Rust 1.70):
  ```bash
  cargo tree --invert atty  # abhängige Pakete finden
  ```
- **`serde_cbor`** auf `ciborium` migrieren (falls aktiv genutzt)
- **`json`** auf `serde_json` oder `jzon` migrieren
