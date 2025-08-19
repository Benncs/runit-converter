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

---

## Examples

### Example 1: Pa → bar
- **Dimension check**:
  Both `Pa` and `bar` are *Pressure*: `M L⁻¹ T⁻²`

- **Conversion factors**:
  - `Pa`: 1 (reference SI unit)
  - `bar`: 1 × 10⁵

---

### Example 2: Pa → pg/mm/h²
- **Dimension check**:
Both units correspond to *Pressure*: `M L⁻¹ T⁻²`

- **Conversion path**:
- Convert `Pa` to SI (no change)
- Apply conversions for `pg`, `mm`, and `h`
- Compute resulting value

- **Result**:
Conversion is valid and computed via SI intermediates.

---

## Roadmap
- [ ] CLI tool for unit conversion
- [ ] Web/REST API interface
- [ ] Extended derived dimensions (energy, power, etc.)
- [ ] User-defined units & dimensions

## License
MIT License – feel free to use and adapt.
