Welcome to Platzi programming language (english version).
Write your sentece to start.

```
>> let a = 5;
>> let b = 10;
>> a + b;
>> 15
>> let mayor_de_edad = fn(edad) {
        // return edad > 18;
        if(edad > 18) {
            return true;
        } else {
            return false;
        }
    };

>> mayor_de_edad(20);
>> true
>> mayor_de_edad(15);
>> false
>> let sumador = fn(x) {

       return fn(y) {
           return x + y;
       };

};

>> let suma_dos = sumador(2);
>> suma_dos(5);
>> 7
>> let suma_cinco = sumador(5);
>> suma_cinco(20);
>> 25
>> mayor_de_edad(suma_cinco(20));
>> true
```
