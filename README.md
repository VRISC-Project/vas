# vas汇编语言

标号以`#`开头，不像其他汇编一样需要加冒号，标号独占一行。

定位标号`#`，作为常数或地址使用时`$(*)p`表示此处之前的最近定位标号，`$(*)n`表示此处之后最近的定位标号。

寄存器以`%`开头。

常数以`$`开头。

地址（标号）以`*`开头。

## 段声明

`section <sec_name> [align=<bytes>] [starts=<addr>]`

## 伪指令

只取nasm的常量写入伪指令，`db`、`dw`、`dd`、`dq`。
