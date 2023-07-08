# vas汇编语言

标号以`#`开头，不像其他汇编一样需要加冒号，标号独占一行。

寄存器以`%`开头。

常数以`$`开头。

地址以`*`开头。

## 段声明

`!section <sec_name> [align=<bytes>] [starts=<addr>]`
