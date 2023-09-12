const express = require('express');

const app = express();

const port = '4000';

app.get('/', async (req: any, res:any) => {
    try {
        res.send('Hello world!')
    } catch (error) {
        console.error(error)
    }
})

app.listen(port, () => {
    console.log(`Sever listening on port: http:${port}`);
})