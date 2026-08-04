# 10. Caso integrador de operación fintech

## Concepto

Una operación integra contratos distintos: intención local, dinero sintético,
idempotencia, proveedor simulado, evento asíncrono y conciliación. El valor no
está en fingir una plataforma completa, sino en mostrar dónde una evidencia
puede cambiar de estado o requerir investigación.

## Problema

Estudiar cada contrato aislado puede ocultar sus interacciones: un reintento
debe conservar resultado, un evento puede repetirse y una evidencia externa
puede discrepar. Una demo conectada a un proveedor real sería costosa,
insegura y difícil de reproducir.

## Alternativas y justificación

Se descartan una integración externa y una simulación que reproduzca todos los
servicios. Se elige un recorrido determinista que usa los módulos previos y
expone un resumen con resultados explícitos. Esto conserva trazabilidad y hace
posibles pruebas sin infraestructura.

```mermaid
flowchart LR
    I[Intención] --> K[Idempotencia]
    K --> P[Proveedor simulado]
    P --> W[Evento sintético]
    W --> R[Conciliación]
    R --> A[Auditoría del draft]
```

## Calidad

No aplica benchmark ni property testing adicional. Las invariantes son los
resultados verificables de los módulos ya probados; la meta es integración
pedagógica, no rendimiento de una infraestructura inexistente.
