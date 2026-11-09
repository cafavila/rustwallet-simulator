# RustWallet Simulator

**Simulador de gestión de wallets y operaciones financieras desarrollado en Rust.**

## ¿Qué es?

RustWallet Simulator es un proyecto en desarrollo orientado al diseño e implementación de un sistema de gestión de wallets, activos y operaciones financieras.

Nace como parte de una iniciativa personal para construir una plataforma propia de gestión de inversiones, explorando los fundamentos necesarios para administrar distintos tipos de activos, controlar saldos, registrar movimientos y gestionar transferencias de manera consistente.

El proyecto utiliza Rust como lenguaje principal y busca desarrollar una arquitectura modular, extensible y mantenible, capaz de evolucionar a medida que se incorporan nuevos requerimientos y funcionalidades.

## Objetivos

El objetivo es construir progresivamente un modelo que permita representar y administrar:

- **Wallets:** contenedores de activos asociados a un propietario.
- **Activos:** representación de diferentes instrumentos o activos digitales.
- **Cuentas operacionales:** administración de saldos disponibles, bloqueados, en staking y en tránsito.
- **Ledger:** registro de movimientos financieros como fuente de verdad para la determinación de saldos.
- **Transferencias:** gestión de movimientos entre cuentas y wallets, tanto internas como externas.
- **Escrow:** mecanismos de reserva y liberación de fondos sujetos a condiciones y confirmaciones de las partes.

Estos componentes constituyen la base sobre la cual se podrán desarrollar operaciones financieras de mayor complejidad.

## Estado actual

<badge color="success">En desarrollo activo</badge>

Actualmente, el trabajo está concentrado en la construcción del modelo de dominio y sus componentes fundamentales.

Entre los elementos iniciales se encuentran:

- Modelo de Wallet y su relación con el propietario.
- Administración de cuentas internas.
- Modelo de activos y sus propiedades.
- Tipos de cuentas operacionales.
- Definición de KRNC, un activo digital ficticio utilizado para las simulaciones y pruebas.

La implementación se realiza de manera incremental, incorporando y validando cada componente antes de avanzar hacia nuevas funcionalidades.

## Criterios de diseño

El desarrollo considera los siguientes principios:

- **Modelado de dominio:** representar explícitamente las entidades, relaciones y reglas del negocio.
- **Encapsulamiento:** proteger el estado interno y controlar las operaciones que lo modifican.
- **Consistencia financiera:** preservar la integridad de los movimientos y saldos.
- **Trazabilidad:** mantener un registro verificable de las operaciones realizadas.
- **Modularidad:** facilitar la incorporación de nuevos activos, operaciones y capacidades.
- **Calidad de software:** utilizar pruebas automatizadas y herramientas del ecosistema Rust para validar la implementación.

## Evolución del proyecto

El proyecto no tiene como objetivo alcanzar una versión cerrada en el corto plazo. Su desarrollo contempla distintas etapas que podrán ampliarse o modificarse según las necesidades de la plataforma.

Las principales líneas de evolución son:

1. **Modelo de dominio:** consolidación de wallets, cuentas y activos.
2. **Ledger:** registro de movimientos y cálculo de saldos.
3. **Gestión de fondos:** reservas, bloqueos y liberaciones.
4. **Transferencias:** operaciones entre cuentas y wallets.
5. **Escrow:** transferencias sujetas a confirmación bilateral.
6. **Servicios backend:** exposición de las operaciones mediante interfaces y API.
7. **Integraciones:** conexión con servicios y plataformas financieras externas.

## Tecnologías

- Rust
- Cargo
- Ecosistema de Rust

## Ejecución

Clonar el repositorio:

```bash
git clone https://github.com/YOUR_USERNAME/rustwallet-simulator.git
cd rustwallet-simulator
```

Ejecutar las pruebas:

```bash
cargo test
```

Verificar formato:

```bash
cargo fmt --check
```

Ejecutar Clippy:

```bash
cargo clippy -- -D warnings
```

## Alcance

RustWallet Simulator es actualmente un proyecto experimental en desarrollo. No se conecta a redes blockchain, no administra criptomonedas reales, no almacena claves privadas ni ejecuta transacciones financieras reales.

KRNC es un activo ficticio utilizado exclusivamente para el desarrollo y las pruebas.

## Un proyecto en evolución

Este repositorio representa el desarrollo progresivo de una solución propia, desde el modelado de sus fundamentos hasta la implementación de operaciones financieras cada vez más complejas.

La arquitectura, las funcionalidades y los objetivos podrán evolucionar a medida que el proyecto avance.

**Construyendo una plataforma financiera, un componente a la vez.**
