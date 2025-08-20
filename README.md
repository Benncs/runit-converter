# Dimensional Analysis & Unit Conversion Tool

This project is a sandbox for experimenting with **Rust crates** and features, particularly the **Turso database**.
It provides functionality for **dimensional analysis** and **unit conversions** across both fundamental and derived physical quantities.

---

## Features
- Store **units** in a database with:
  - Unit **name** (e.g., `meter`, `bar`, `gram`)
  - Associated **dimension** (e.g., `length`, `pressure`)
  - **Conversion factor** relative to the SI base unit
- Support for:
  - **Base dimensions**:
    - Mass, Length, Time, Electric Current, Amount of Substance, Temperature, Luminous Intensity
  - **Derived dimensions**:
    - Pressure, Voltage, etc. (expressed as products/powers of base dimensions)
- Safe **conversion** between units by:
  1. Validating that dimensions match
  2. Converting via SI as the common reference

---

## How It Works
1. Units are defined in the database with:
   - `name`
   - `dimension`
   - `conversion_factor_to_SI`

2. Conversion algorithm:
   - Verify source and target dimensions are identical
   - Retrieve each unit’s factor relative to SI
   - Convert using SI as an intermediate


## Roadmap
- [x] CLI tool for unit conversion
  - [ ] Interactive cli
- [ ] Web/REST API interface
- [ ] Extended derived dimensions (energy, power, etc.)
- [ ] User-defined units & dimensions

## License
SPDX-License-Identifier: GPL-3.0-or-later
