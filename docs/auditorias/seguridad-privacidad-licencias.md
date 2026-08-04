# Auditoría de seguridad, privacidad y licencias

## Seguridad y privacidad

- El crate declara `#![forbid(unsafe_code)]`.
- No hay clientes de red, proveedores, SDKs, credenciales, secretos, tarjetas,
  cuentas ni datos personales.
- Los montos, referencias, eventos y decisiones son sintéticos y locales.
- Tokenización, 3-D Secure, SCA y PCI-DSS se presentan como fronteras y
  vocabulario; el curso no declara cumplimiento ni certificación.
- El repositorio no debe recibir datos financieros ni tokens de producción.

## Licencias

- El código del crate se distribuye bajo MIT o Apache-2.0.
- El contenido pedagógico se identifica mediante CC BY-SA 4.0 en
  [LICENSE.md](../../LICENSE.md).
- No se añadieron dependencias, imágenes ni materiales de terceros que exijan
  atribución adicional.

## Conclusión

El draft mantiene su alcance de demostración. Cualquier integración externa,
criptografía aplicada, secreto, dato personal o requisito regulatorio requiere
una decisión humana y un plan nuevo antes de incorporarse.
