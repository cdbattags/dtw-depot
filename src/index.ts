import express from 'express'
import homeController from './controllers/index'

const main = async () => {
    const app = express()
    const port = 3000

    app.get('/', (req, res) => res.sendStatus(200))
    app.get('/hello-world', homeController)

    app.listen(
        port,
        () => console.log(`Example app listening at http://localhost:${port}`)
    )
}

main()
