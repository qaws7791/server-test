# HTTP Server Performance test



## Test Results

|        | type                                       | transactions per second |      |
| ------ | :----------------------------------------- | ----------------------- | ---- |
| Nodejs | [express](./nodejs/express.js)             | 7500                    |      |
|        | [native](./nodejs/native.js)               | 25000                   |      |
|        | [fastify](./nodejs/fastify.js)             | 28000                   |      |
|        | [hyper-express](./nodejs/hyper-express.js) | 45000                   |      |
| Bun    | [native](./bun/native.ts)                  | 46000                   |      |
| Go     | [native(net/http)](./golang/nethttp.go)    | 60000                   |      |
| Rust   | [native](./rust/native)                    | not yet                 |      |
|        | [actix](./rust/actix)                      | not yet                 |      |

```mermaid
xychart-beta
    title "Simple Throughput Test"
    x-axis [express, "nodejs(native)", fastify, hyper-express, "bun(native)", "go(native)"]
    y-axis "TPC(transactions per second)"
    bar [7500, 25000, 28000, 45000, 46000, 60000]
    line [7500, 25000, 28000, 45000, 46000, 60000]
```

## Test Environment 

- CPU: 5600x
- RAM: 16gb
- OS: windows 11



![image-20240815225643011](assets/image-20240815225643011.png)



