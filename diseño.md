Para poder hacer que el programa sea fácil de programar, vamos a tener que dividirlo, atacanto diferentes partes en diferentes tiempos. 


- DATOS; obtención de datos y procesamiento de datos: GET_DATA y DATA_ANA
- PORTFOLIO: Crear portafolios, añadir-eliminar posiciones, seguir el flujo de caja, seguir dividendos.
- DASHBOARD: Basado en los portafolios de principio pero me gustaría poder añadir funcionalidades sociales. 


Para poder declarar la versión 1.0 estos serán los requisitos mínimos.


### PORTFOLIO

- Crear portafolio: Tiene que crear un portafolio, tiene que tener los siguientes datos; ID, NOMBRE DEL PORTAFOLIO, CLIENTE
- Agregamos transacciones. Aquí tenemos que ver la forma de hacer una relación, entre el portafolio, el ticker y el id del activo. 
- Una vez hacemos esa relación hay dos caminos; dividendos y transacciones. Esas estan aninadadas en el portafolio pero tienen una base de datos diferente. 


Funciones:

- portafolios: De momento solo tengo planeao una función que cree un portafolio y otro que lo borre. 
- movimientos en portafolio: añadir, eliminar
- dividendos: Ver dividendos en el portafolio y consultarlos.