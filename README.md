# Down the Whole: Depot

Time to speed up Express.

```
export default (req: Request, res: Response) => {
    response.status(200).json({
        hello: 'world!',
    })
}
```

Languages: Rust and Node.js

# Why

I got tired of writing the same code over and over to get a simple server setup for actix-web and/or Express with TypeScript. I took it a step further here trying to mirror the interface for Express controllers but using actix-web as the underlying server.

This is also pretty similar to the Node.js AWS lambda syntax for handlers which will come in handy later as well.

# Goals

- `actix-web` stood up with a single service
- service forwards to 
