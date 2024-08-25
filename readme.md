# HTTP Server Performance test



## Test Results

|          | type               | transactions per second |      |
| -------- | :----------------- | ----------------------- | ---- |
| **Node** | next(app router)   | 1700                    |      |
|          | nest(with express) | 7000                    |      |
|          | express            | 7500                    |      |
|          | native             | 25000                   |      |
|          | hono               | 27000                   |      |
|          | fastify            | 28000                   |      |
|          | hyper-express      | 45000                   |      |
| **Bun**  | native             | 46000                   |      |
| **Go**   | native(net/http)   | 60000                   |      |
| **Rust** | native             | not yet                 |      |
|          | actix              | not yet                 |      |

```mermaid
xychart-beta
    title "Throughput Test"
    x-axis [next, nest, express, "nodejs(native)", hono, fastify, hyper-express, "bun(native)", "go(native)"]
    y-axis "TPC(transactions/second)"
    bar [1700, 7000, 7500, 25000, 27000, 28000, 45000, 46000, 60000]
    line [1700, 7000, 7500, 25000, 27000, 28000, 45000, 46000, 60000]
```

## Test Environment 

- CPU: 5600x
- RAM: 16gb
- OS: windows 11



![image-20240815225643011](assets/image-20240815225643011.png)



