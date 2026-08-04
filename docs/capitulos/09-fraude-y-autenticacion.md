# 09. Fraude, 3-D Secure y SCA

## Concepto

Una decisión de riesgo combina señales y una política: permitir, pedir un
paso adicional o rechazar. 3-D Secure y SCA son marcos de autenticación y
cumplimiento que el curso nombra para delimitar responsabilidades, no para
certificarlas.

## Problema

Un puntaje opaco o una regla de “aprobar todo” hacen imposible explicar una
decisión. Usar datos personales o perfiles reales convertiría el ejercicio en
un sistema antifraude sin evidencia ni autorización.

## Alternativas y justificación

Se elige una política pequeña sobre señales booleanas sintéticas. Permite ver
cuándo se solicita un desafío adicional sin afirmar que aplica SCA ni que
detecta fraude. No hay perfiles, modelos estadísticos ni datos personales.

## Calidad

No aplica benchmark ni property testing: las combinaciones de tres señales se
prueban de forma directa y el modelo no pretende medir precisión antifraude.
