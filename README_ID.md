# praborrow-sidl (Bahasa Indonesia)

[English](./README.md) | Indonesia

Sovereign Interface Definition Language (SIDL).

## Ikhtisar (Overview)

Menyediakan definisi dan logika ekspansi untuk macro `sovereign_interface!`. Crate ini bertanggung jawab untuk menghasilkan binding FFI yang aman secara tipe (type-safe) dan trait internal untuk komunikasi lintas batas dalam PraBorrow.

## Fitur Utama (Key Features)

- **Definisi Antarmuka**: DSL berbasis macro untuk mendefinisikan kontrak layanan terdistribusi.
- **Pembuatan Binding**: Mengotomatiskan pembuatan stub klien/server.
- **Keamanan Tipe (Type Safety)**: Memastikan konsistensi tipe argumen dan pengembalian di seluruh batas jaringan.
